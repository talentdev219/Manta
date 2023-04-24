// Copyright 2020-2023 Manta Network.
// This file is part of Manta.
//
// Manta is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Manta is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Manta.  If not, see <http://www.gnu.org/licenses/>.

use super::*;
use codec::alloc::collections::BTreeSet;
use frame_support::dispatch::RawOrigin;
use frame_support::ensure;
use frame_support::traits::EstimateCallFee;
use frame_support::traits::Get;
use frame_support::traits::Randomness;
use pallet_parachain_staking::BalanceOf;
use sp_runtime::traits::Saturating;
use sp_runtime::traits::Zero;
use sp_runtime::DispatchResult;
use sp_runtime::PerThing;
use sp_runtime::Percent;
use sp_std::{vec, vec::Vec};

impl<T: Config> Pallet<T> {
    #[named]
    pub(crate) fn calculate_deposit_distribution(
        new_deposit: BalanceOf<T>,
    ) -> Vec<(T::AccountId, BalanceOf<T>)> {
        log::trace!(function_name!());
        log::debug!(
            "Calculating distribution for deposit of {:?} tokens",
            new_deposit
        );
        if new_deposit < <T as pallet_parachain_staking::Config>::MinDelegation::get() {
            return vec![];
        }
        let mut deposits: Vec<(T::AccountId, BalanceOf<T>)> = vec![];

        // first concern: If we fell out of the active set on one or more collators, we need to get back into it
        let active_collators = pallet_parachain_staking::Pallet::<T>::selected_candidates()
            .into_iter()
            .collect();
        let collators_we_are_staked_with: BTreeSet<_> = StakedCollators::<T>::iter_keys().collect();
        let active_collators_we_are_staked_with =
            collators_we_are_staked_with.intersection(&active_collators);

        let mut remaining_deposit = new_deposit;
        for collator in active_collators_we_are_staked_with {
            let staked = StakedCollators::<T>::get(collator.clone());
            let info = pallet_parachain_staking::Pallet::<T>::candidate_info(collator.clone())
                .expect("is active collator, therefor it has collator info. qed");
            if staked < info.lowest_top_delegation_amount {
                let deposit =
                    remaining_deposit.saturating_sub(info.lowest_top_delegation_amount - staked);
                deposits.push((collator.clone(), deposit)); // TODO: Sort collators ascending by missing amount so we get the largest amount of collators active before running out of funds
                remaining_deposit = remaining_deposit.saturating_sub(deposit);
                if remaining_deposit.is_zero() {
                    break;
                }
            }
        }
        // If we have any collators to re-activate, we distribute all tokens to those and call it a day
        if !deposits.is_empty() {
            if !remaining_deposit.is_zero() {
                // distribute remaining tokens evenly
                let deposit_per_collator =
                    Percent::from_rational(1, deposits.len().into()).mul_ceil(remaining_deposit); // this overshoots the amount if there's a remainder
                for deposit in &mut deposits {
                    let add = remaining_deposit.saturating_sub(deposit_per_collator);
                    deposit.1 += add;
                    remaining_deposit -= add;
                }
            }
            return deposits;
        }

        // second concern: We want to maximize staking APY earned, so we want to balance the staking pools with our deposits while conserving gas
        // We only consider active collators for deposits
        // TODO: Also consider points / pointsAwarded to not stake to collators missing blocks

        let top_collator_accounts = pallet_parachain_staking::Pallet::<T>::compute_top_candidates(); // XXX/TODO: This can select collators that are not joined but not yet active
        let mut collators_and_counted_balances: Vec<_> = top_collator_accounts
            .iter()
            .map(|collator| {
                (
                    collator,
                    pallet_parachain_staking::Pallet::<T>::candidate_info(collator.clone())
                        .expect("is active collator, therefore it has collator info. qed")
                        .total_counted,
                )
            })
            .collect();
        // sort ascending by counted stake
        collators_and_counted_balances.sort_by(|a, b| a.1.cmp(&b.1));
        debug_assert!(
            collators_and_counted_balances.len() == 1
                || pallet_parachain_staking::Pallet::<T>::candidate_info(
                    collators_and_counted_balances[0].0.clone()
                )
                .unwrap()
                .total_counted
                    <= pallet_parachain_staking::Pallet::<T>::candidate_info(
                        collators_and_counted_balances[1].0.clone()
                    )
                    .unwrap()
                    .total_counted
        );

        let median_collator_balance = collators_and_counted_balances
            [collators_and_counted_balances.len() / 2]
            .1
            .clone();

        // build collator => deviation from mean map
        let mut underallocated_collators: Vec<_> =
            collators_and_counted_balances[..collators_and_counted_balances.len() / 2].to_vec();
        let mut underallocated_collators: Vec<_> = underallocated_collators
            .into_iter()
            .filter_map(|(collator, balance)| {
                let underallocation = median_collator_balance.saturating_sub(balance);
                if !underallocation.is_zero() {
                    Some((collator, underallocation))
                } else {
                    None
                }
            })
            .collect();
        // After this calculation, underallocated_collators is in descending order of underallocation

        // take up to 4 collators with the highest deficit ( stopping at mean )
        let num_collators_to_take = core::cmp::min(4, underallocated_collators.len());
        let underallocated_collators = underallocated_collators[..num_collators_to_take].to_vec();

        debug_assert!(
            underallocated_collators.is_empty()
                || pallet_parachain_staking::Pallet::<T>::candidate_info(
                    underallocated_collators[0].0.clone()
                )
                .unwrap()
                .total_counted
                    <= median_collator_balance
        );
        debug_assert!(
            underallocated_collators.len() < 2
                || pallet_parachain_staking::Pallet::<T>::candidate_info(
                    underallocated_collators[0].0.clone()
                )
                .unwrap()
                .total_counted
                    <= pallet_parachain_staking::Pallet::<T>::candidate_info(
                        underallocated_collators[1].0.clone()
                    )
                    .unwrap()
                    .total_counted
        );
        debug_assert!(
            underallocated_collators.len() < 2
                || underallocated_collators[0].1.clone() >= underallocated_collators[1].1.clone()
        );
        log::debug!(
            "Total Underallocated collators: {:?}",
            underallocated_collators.len()
        );
        if !underallocated_collators.is_empty() {
            let total_underallocation = underallocated_collators
                .iter()
                .cloned()
                .map(|a| a.1)
                .reduce(|acc, balance| acc + balance)
                .unwrap();
            log::debug!(
                "Underallocated tokens {:?} on selected collators: {:?}",
                total_underallocation,
                underallocated_collators.clone()
            );
            let deposit_to_distribute = remaining_deposit;
            for (account, tokens_to_reach_mean) in underallocated_collators {
                // If a proportional deposit is over the min deposit and can get us into the top balance, deposit it, if not just skip it
                let info = pallet_parachain_staking::Pallet::<T>::candidate_info(account.clone())
                    .expect("is active collator, therefor it has collator info. qed");
                let collator_proportion =
                    Percent::from_rational(tokens_to_reach_mean.clone(), total_underallocation);
                let to_reach_mean = collator_proportion.mul_ceil(deposit_to_distribute);
                let to_deposit = to_reach_mean.min(remaining_deposit);
                let our_stake = StakedCollators::<T>::get(account.clone());
                if to_deposit > <T as pallet_parachain_staking::Config>::MinDelegation::get()
                    && to_deposit + our_stake > info.lowest_top_delegation_amount
                {
                    deposits.push((account.clone(), to_deposit));
                    remaining_deposit -= to_deposit;
                    log::debug!(
                        "Selected collator {:?} for deposit of {:?} token",
                        account.clone(),
                        to_deposit
                    );
                };
                if remaining_deposit.is_zero() {
                    break;
                }
            }
        }
        // if we had to skip a collator above due to not getting into the top deposit, we just lump the rest into the collator with the lowest stake
        if !deposits.is_empty() && !remaining_deposit.is_zero() {
            let mut underallocated_collators = deposits.pop().unwrap();
            underallocated_collators.1 += remaining_deposit;
            remaining_deposit.set_zero();
            deposits.push(underallocated_collators);
        }

        // fallback: just assign to a random active collator
        if !remaining_deposit.is_zero() {
            let active_collators = pallet_parachain_staking::Pallet::<T>::selected_candidates();
            // TODO: Better randomness
            use sp_runtime::traits::SaturatedConversion;
            let nonce: u128 = Self::total_pot().saturated_into();
            let random = sp_core::U256::from_big_endian(
                T::RandomnessSource::random(&nonce.to_be_bytes()).0.as_ref(),
            );
            let random_index: usize = random.low_u64() as usize % active_collators.len();
            if let Some(random_collator) = active_collators.get(random_index) {
                deposits.push((random_collator.clone(), remaining_deposit));
                log::warn!(
                    "Failed to select staking outputs. Staking {:?} randomly to {:?}",
                    remaining_deposit,
                    random_collator
                );
                remaining_deposit.set_zero();
            }
        }

        if !remaining_deposit.is_zero() {
            log::error!(
                "We have {:?} unstaked balance left over after depositing",
                remaining_deposit
            );
        }
        if deposits.is_empty() {
            log::error!("COULD NOT FIND ANY COLLATOR TO STAKE TO");
        }
        log::debug!("Deposits: {:?}", deposits);
        deposits
    }

    #[named]
    pub(crate) fn calculate_withdrawal_distribution(
        withdrawal_amount: BalanceOf<T>,
    ) -> Vec<T::AccountId> {
        log::trace!(function_name!());
        if withdrawal_amount.is_zero() {
            return vec![];
        }
        let mut withdrawals = vec![];
        let mut remaining_balance = withdrawal_amount;

        // first concern: If there are inactive collators we are staked with, prefer these
        let now = <frame_system::Pallet<T>>::block_number();
        let info = pallet_parachain_staking::Pallet::<T>::round();
        let active_collators: BTreeSet<_> =
            pallet_parachain_staking::Pallet::<T>::selected_candidates()
                .into_iter()
                .filter(|collator| {
                    // being selected is not enough, it must also be actively participating in block production
                    // but we need to ensure it had a chance to produce blocks, so we only check this if we're at least 25% into a round
                    if now < info.first + (info.length / 4u32).into() {
                        return true;
                    }
                    !pallet_parachain_staking::Pallet::<T>::awarded_pts(info.current, collator)
                        .is_zero()
                })
                .collect();
        let collators_we_are_staked_with: BTreeSet<_> = StakedCollators::<T>::iter_keys().collect();
        let inactive_collators_we_are_staked_with: BTreeSet<_> = collators_we_are_staked_with
            .difference(&active_collators)
            .cloned()
            .collect();

        // since these collators are inactive, we just unstake in any order until we have satisfied the withdrawal request
        for collator in inactive_collators_we_are_staked_with {
            let balance = StakedCollators::<T>::get(&collator);
            log::debug!("Unstaking {:?} from inactive {:?}", balance, collator);
            remaining_balance = remaining_balance.saturating_sub(balance);
            withdrawals.push(collator.clone());
            if remaining_balance.is_zero() {
                return withdrawals;
            }
        }
        log::debug!("Remaining after inactive: {:?}", remaining_balance);

        // If we have balance to withdraw left over, we have to unstake some healthy collator.
        // Unstake starting from the highest overallocated collator ( since that yields the lowest APY ) going down until request is satisfied
        let mut apy_ordered_active_collators_we_are_staked_with: Vec<_> =
            collators_we_are_staked_with
                .intersection(&active_collators)
                .cloned()
                .collect();
        apy_ordered_active_collators_we_are_staked_with.sort_by(|a, b| {
            let ainfo = pallet_parachain_staking::Pallet::<T>::candidate_info(a.clone())
                .expect("is active collator, therefore it has collator info. qed");
            let binfo = pallet_parachain_staking::Pallet::<T>::candidate_info(b.clone())
                .expect("is active collator, therefore it has collator info. qed");
            binfo.total_counted.cmp(&ainfo.total_counted)
        });
        log::debug!(
            "Active collators: {:?}",
            apy_ordered_active_collators_we_are_staked_with.len()
        );
        for c in apy_ordered_active_collators_we_are_staked_with {
            let our_stake = StakedCollators::<T>::get(c.clone()).clone();
            log::debug!("Unstaking {:?} from active {:?}", our_stake, c);
            withdrawals.push(c);
            remaining_balance = remaining_balance.saturating_sub(our_stake);
            if remaining_balance.is_zero() {
                break;
            }
        }

        if !remaining_balance.is_zero() {
            log::error!(
                "We have {:?} left that COULD NOT BE UNSTAKED",
                remaining_balance
            );
        }
        if withdrawals.is_empty() {
            log::error!("COULD NOT FIND ANY COLLATOR TO WITHDRAW FROM");
        }
        log::debug!("Withdrawals: {:?}", withdrawals.len());
        withdrawals
    }

    fn average_stake_per_collator() -> BalanceOf<T> {
        let total_staked = pallet_parachain_staking::Pallet::<T>::staked(
            pallet_parachain_staking::Pallet::<T>::round().current,
        ); // XXX/TODO: pallet_parachain_staking::Pallet::<T>::Staked storage is only updated *at the beginning of a round* this is not suitable multi-transfers in the same block don't get recalculated
           // this overshoots the amount if there's a remainder
        Percent::from_rational(1, pallet_parachain_staking::Pallet::<T>::total_selected())
            .mul_ceil(total_staked)
            .into()
    }

    #[named]
    pub(crate) fn do_stake_one_collator(
        collator: T::AccountId,
        amount: BalanceOf<T>,
    ) -> DispatchResult {
        log::trace!(function_name!());
        // preconditions
        if Self::lottery_funds_surplus().is_zero() {
            return Err(Error::<T>::PotBalanceTooLow.into());
        }

        // TODO: Calculate these from current values
        const CANDIDATE_DELEGATION_COUNT: u32 = 500;
        const DELEGATION_COUNT: u32 = 500;

        // If we're already delegated to this collator, we must call `delegate_more`
        if StakedCollators::<T>::get(&collator).is_zero() {
            // Ensure the pallet has enough gas to pay for this
            let fee_estimate: BalanceOf<T> = T::EstimateCallFee::estimate_call_fee(
                &pallet_parachain_staking::Call::delegate {
                    candidate: collator.clone(),
                    amount,
                    candidate_delegation_count: CANDIDATE_DELEGATION_COUNT,
                    delegation_count: DELEGATION_COUNT,
                },
                None.into(),
            );
            ensure!(
                Self::lottery_funds_surplus() > fee_estimate,
                Error::<T>::PotBalanceTooLowToPayTxFee
            );
            pallet_parachain_staking::Pallet::<T>::delegate(
                RawOrigin::Signed(Self::account_id()).into(),
                collator.clone(),
                amount.into(),
                CANDIDATE_DELEGATION_COUNT,
                DELEGATION_COUNT,
            )
            .map_err(|e| {
                log::error!(
                    "Could not delegate {:?} to collator {:?} with error {:?}",
                    amount.clone(),
                    collator.clone(),
                    e
                );
                e.error
            })?;
        } else {
            // Ensure the pallet has enough gas to pay for this
            let fee_estimate: BalanceOf<T> = T::EstimateCallFee::estimate_call_fee(
                &pallet_parachain_staking::Call::delegator_bond_more {
                    candidate: collator.clone(),
                    more: amount.clone(),
                },
                None.into(),
            );
            ensure!(
                Self::lottery_funds_surplus() > fee_estimate,
                Error::<T>::PotBalanceTooLowToPayTxFee
            );
            pallet_parachain_staking::Pallet::<T>::delegator_bond_more(
                RawOrigin::Signed(Self::account_id()).into(),
                collator.clone(),
                amount.clone(),
            )
            .map_err(|e| {
                log::error!(
                    "Could not bond more {:?} to collator {:?} with error {:?}",
                    amount.clone(),
                    collator.clone(),
                    e
                );
                e.error
            })?;
        }
        StakedCollators::<T>::mutate(&collator, |balance| *balance += amount);

        log::debug!("Delegated {:?} tokens to {:?}", amount, collator);
        Ok(())
    }

    #[named]
    pub(crate) fn do_unstake_collator(
        now: T::BlockNumber,
        some_collator: T::AccountId,
    ) -> DispatchResult {
        log::trace!(function_name!());
        let delegated_amount_to_be_unstaked = StakedCollators::<T>::take(some_collator.clone());
        if delegated_amount_to_be_unstaked.is_zero() {
            log::error!("requested to unstake a collator that isn't staked");
            return Err(Error::<T>::TODO.into());
        };
        log::debug!(
            "Unstaking collator {:?} with balance {:?}",
            some_collator.clone(),
            delegated_amount_to_be_unstaked.clone()
        );
        // Ensure the pallet has enough gas to pay for this
        let fee_estimate: BalanceOf<T> = T::EstimateCallFee::estimate_call_fee(
            &pallet_parachain_staking::Call::schedule_revoke_delegation {
                collator: some_collator.clone(),
            },
            None.into(),
        );
        ensure!(
            Self::lottery_funds_surplus() > fee_estimate,
            Error::<T>::PotBalanceTooLowToPayTxFee
        );
        // unstake from parachain staking
        // NOTE: All funds that were delegated here will no longer produce staking rewards
        pallet_parachain_staking::Pallet::<T>::schedule_revoke_delegation(
            RawOrigin::Signed(Self::account_id()).into(),
            some_collator.clone(),
        )
        .map_err(|e| e.error)?;

        // Update bookkeeping
        RemainingUnstakingBalance::<T>::mutate(|bal| {
            *bal = (*bal).saturating_add(delegated_amount_to_be_unstaked.into());
        });
        UnstakingCollators::<T>::mutate(|collators| {
            collators.push(UnstakingCollator {
                account: some_collator.clone(),
                since: now,
            })
        });
        TotalPot::<T>::mutate(|pot| *pot = (*pot).saturating_sub(delegated_amount_to_be_unstaked));

        Ok(())
    }
}

#[test]
fn sorting_collators_works() {
    use codec::alloc::collections::HashMap;
    use manta_primitives::types::AccountId;

    const AL: sp_runtime::AccountId32 = sp_runtime::AccountId32::new([0u8; 32]);
    const BO: sp_runtime::AccountId32 = sp_runtime::AccountId32::new([1u8; 32]);
    const CH: sp_runtime::AccountId32 = sp_runtime::AccountId32::new([2u8; 32]);
    const DA: sp_runtime::AccountId32 = sp_runtime::AccountId32::new([3u8; 32]);
    const EV: sp_runtime::AccountId32 = sp_runtime::AccountId32::new([4u8; 32]);
    let active_collators = vec![DA.clone(), BO.clone(), CH.clone(), AL.clone(), EV.clone()];

    let mut staked: HashMap<AccountId, u32> = HashMap::new();
    staked.insert(AL.clone(), 10);
    staked.insert(BO.clone(), 11);
    staked.insert(CH.clone(), 12);
    staked.insert(DA.clone(), 13);
    staked.insert(EV.clone(), 14);

    let mean_stake = (10 + 11 + 12 + 13 + 14) / 5;

    // logic under test
    let mut underallocated_collators = vec![];
    for collator in active_collators.iter() {
        let total_stake = staked.get(collator).unwrap().clone();
        if total_stake < mean_stake {
            underallocated_collators.push((collator.clone(), total_stake));
        }
    }
    assert_eq!(underallocated_collators[0], (BO.clone(), 11));
    assert_eq!(underallocated_collators[1], (AL.clone(), 10));

    underallocated_collators.sort_unstable_by(|a, b| a.1.cmp(&b.1));
    let (_rest, mut last) =
        underallocated_collators.split_at(underallocated_collators.len().saturating_sub(4)); // TODO: 4 is hardcoded make configurable

    assert_eq!(underallocated_collators[0], (AL.clone(), 10));
    assert_eq!(underallocated_collators[1], (BO.clone(), 11));
}
