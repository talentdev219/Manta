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

//! # Lottery Pallet generating funds with parachain_staking
//! User funds are staked and the lottery draws its prize pool from the staking rewards accrued during the lottery period
//! Funds deposited to the lottery become eligible to win after one drawing.
//! Funds withdrawn from the the lottery are subject to a timelock determined by parachain-staking before they can be claimed.
//!
//! ### Rules
//! 1. A drawing is scheduled to happen every `<LotteryInterval<T>>::get()` blocks.
//! 2. A designated manager can start & stop the drawings as well as rebalance the stake to better collators
//! 3. Winnings are paid out directly to the winner's wallet after each drawing
//! 4. In order to prevent gaming of the lottery winner, no modifications to this pallet are allowed a configurable amount of time before a drawing
//!     This is needed e.g. using BABE Randomness, where the randomness will be known a day before the scheduled drawing
//! 5. Deposits happen instantly
//! 6. Withdrawals have a up-to 7 day timelock and are paid out automatically (via scheduler) in the first lottery drawing after it expires

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub mod weights;

pub use pallet::*;

// pub use weights::WeightInfo;
#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        ensure, ensure_eq, log,
        pallet_prelude::*,
        traits::{ExistenceRequirement::KeepAlive, *},
        PalletId,
    };
    use frame_system::pallet_prelude::*;
    pub use frame_system::WeightInfo;
    use manta_primitives::{constants::time::DAYS, types::BlockNumber};
    use sp_runtime::DispatchResult;
    use sp_std::prelude::*;
    use sp_arithmetic::traits::Saturating;

    // TODO: Remove
    use session_key_primitives::util::unchecked_account_id;

    const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

    type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_scheduler::Config {
        /// Overarching event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        /// The currency mechanism.
        type Currency: ReservableCurrency<Self::AccountId>;
        // Randomness source to use for determining lottery winner
        type RandomnessSource: Randomness<Self::Hash, Self::BlockNumber>;
        /// Origin that can manage lottery parameters and start/stop drawings
        type ManageOrigin: EnsureOrigin<frame_system::Origin<Self>>;
        /// Account Identifier from which the internal Pot is generated.
        type LotteryPot: Get<PalletId>;
        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;
    }

    #[pallet::pallet]
    #[pallet::storage_version(STORAGE_VERSION)]
    // #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(PhantomData<T>);

    // Configurable (constant) storage items

    #[pallet::storage]
    #[pallet::getter(fn lottery_interval)]
    pub(super) type LotteryInterval<T: Config> = StorageValue<_, BlockNumber, ValueQuery>;

    /// Depending on the randomness source, the winner might be established before the drawing, this prevents modification of the eligible winning set after the winner
    /// has been established but before it is selected by fn draw_lottery()
    #[pallet::storage]
    #[pallet::getter(fn drawing_buffer)]
    pub(super) type PreDrawingModificationLockBlocks<T: Config> =
        StorageValue<_, BlockNumber, ValueQuery>;

    /// NOTE: how much KMA to keep in the pallet for gas
    /// This must be initialized at genesis, otherwise the pallet will run out of gas at the first drawing
    #[pallet::storage]
    #[pallet::getter(fn gas_reserve)]
    pub(super) type GasReserve<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn min_deposit)]
    pub(super) type MinDeposit<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn min_withdraw)]
    pub(super) type MinWithdraw<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    /// This value is imposed by the staking solution used and must be configured >= than what it uses
    #[pallet::storage]
    #[pallet::getter(fn unstake_time)]
    pub(super) type UnstakeTime<T: Config> = StorageValue<_, BlockNumber, ValueQuery>;

    // Dynamic Storage Items

    /// NOTE: sum of all user's deposits, to ensure balance never drops below
    #[pallet::storage]
    #[pallet::getter(fn sum_of_deposits)]
    pub(super) type SumOfDeposits<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    /// NOTE: the total pot is the total number of KMA eligible to win in the current drawing cycle
    #[pallet::storage]
    #[pallet::getter(fn total_pot)]
    pub(super) type TotalPot<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn next_drawing)]
    pub(super) type NextDrawingAt<T: Config> = StorageValue<_, BlockNumber, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn is_rebalancing)]
    pub(super) type RebalanceInProgress<T: Config> = StorageValue<_, bool, ValueQuery>;

    #[pallet::storage]
    pub(super) type ActiveBalancePerUser<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, BalanceOf<T>>;

    #[derive(Encode, Decode, TypeInfo)]
    pub(super) struct UnstakingCollator {
        account: T::AccountId,
        since: T::BlockNumber,
    }

    #[pallet::storage]
    pub(super) type UnstakingCollators<T: Config> =
        StorageValue<_, Vec<UnstakingCollator>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn remaining_unstaking_balance)]
    pub(super) type RemainingUnstakingBalance<T: Config> =
        StorageValue<_, BalanceOf<T>, ValueQuery>;
    // type RemainingBalanceOnUnstakingCollator<T: Config> =
    //     StorageMap<_, Blake2_128Concat, T::AcccountId, T::BalanceOf>;

    #[derive(Encode, Decode, TypeInfo)]
    pub(super) struct Request {
        user: T::AccountId,
        block: T::BlockNumber,
        balance: T::BalanceOf,
    }

    #[pallet::storage]
    pub(super) type WithdrawalRequestQueue<T: Config> = StorageValue<_, Vec<Request>, ValueQuery>;

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub lottery_interval: BlockNumber,
        pub drawing_freezeout: BlockNumber,
        pub unstake_time: BlockNumber,
        /// amount of token to keep in the pot for paying gas fees
        pub gas_reserve: BalanceOf<T>,
        pub min_deposit: BalanceOf<T>,
        pub min_withdraw: BalanceOf<T>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                lottery_interval: 7 * DAYS,
                drawing_freezeout: 1 * DAYS,
                min_deposit: 1,
                min_withdraw: 1,
                unstake_time: 7 * DAYS,
                gas_reserve: 10_000,
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        #[inline]
        fn build(&self) {
            LotteryInterval::<T>::set(self.lottery_interval);
            PreDrawingModificationLockBlocks::<T>::set(self.drawing_freezeout);
            UnstakeTime::<T>::set(self.unstake_time);
            GasReserve::<T>::set(self.gas_reserve);
            MinDeposit::<T>::set(self.min_deposit);
            MinWithdraw::<T>::set(self.min_withdraw);
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(fn deposit_event)]
    pub enum Event<T: Config> {
        AddedToLottery(T::AccountId, BalanceOf<T>),
        LotteryStarted(T::BlockNumber),
        LotteryStopped(T::BlockNumber),
        LotteryWinner(T::AccountId),
    }

    #[pallet::error]
    pub enum Error<T> {
        OnlyRootOrigin,
        OnlyRootOrManageOrigin,
        LotteryNotStarted,
        LotteryAlreadyStarted,
        LotteryAlreadyStopped,
        LotteryNotScheduled,
        TooEarlyForDrawing,
        TooCloseToDrawing,
        PotBalanceTooLow,
        NoWinnerFound,
        DepositBelowMinAmount,
        WithdrawBelowMinAmount,
        WithdrawAboveDeposit,
        PalletMisconfigured
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(0)]
        pub fn deposit(origin: OriginFor<T>, amount: BalanceOf<T>) -> DispatchResult {
            let caller_account = ensure_signed(origin)?;

            ensure!(
                amount >= Self::min_deposit(),
                Error::<T>::DepositBelowMinAmount
            );
            ensure!(
                Self::not_in_drawing_freezeout,
                Error::<T>::TooCloseToDrawing
            );

            // Transfer funds to pot
            Currency::transfer(
                &origin,
                &LotteryPot::get().into_account_truncating(),
                amount,
            )?;

            // Attempt to stake them with some collator
            Self::do_stake(amount)?;

            // Add to active funds
            ActiveBalancePerUser::<T>::mutate(caller_account, |balance| balance += amount);
            TotalPot::<T>::mutate(|balance| balance + amount);

            Self::deposit_event(Event::AddedToLottery(origin, amount));
            Ok(())
        }

        #[pallet::weight(0)]
        pub fn request_withdraw(origin: OriginFor<T>, amount: BalanceOf<T>) -> DispatchResult {
            let caller = ensure_signed(origin)?;

            ensure!(
                amount >= Self::min_withdraw(),
                Error::<T>::WithdrawBelowMinAmount
            );
            ensure!(
                Self::not_in_drawing_freezeout,
                Error::<T>::TooCloseToDrawing
            );

            let now = <frame_system::Pallet<T>>::block_number();

            // Ensure user has enough funds active and mark them as offboarding
            ActiveBalancePerUser::<T>::try_mutate(caller, |balance| {
                // Withdraw only what's active
                ensure!(balance >= amount, Error::<T>::WithdrawAboveDeposit);
                // Mark funds as offboarding
                WithdrawalRequestQueue::<T>::mutate(|withdraw_vec| {
                    withdraw_vec.push(Request {
                        user: caller,
                        block: now,
                        balance: amount,
                    })
                });
                // store reduced balance
                balance -= amount;
                T::TotalPot::mutate(|pot| pot -= amount);
            });

            // Unstaking workflow
            // 1. See if this withdrawal can be serviced with left-over balance from an already unstaking collator, if so deduct remaining balance and schedule the request
            // 2. If it can't, find the collator with the smallest delegation that is able to handle this withdrawal request and fully unstake it
            // 3. Add balance overshoot to "remaining balance" to handle further requests from

            // If the withdrawal fits in the available funds, do nothing else
            RemainingUnstakingBalance::<T>::try_mutate(|&mut remaining| {
                remaining -= amount;
                (remaining > 0).then(remaining).ok_or(Err(
                    "not enough left to handle this request from current unstaking funds",
                ))
            })
            .or_else(|_| {
                // Withdrawal needs an extra collator to unstake to have enough funds to serve withdrawals, do it

                // If this fails, something weird is going on or the next line needs to be implemented
                // TODO: add some arithmetic to only unstake the diff between needed and remaining instead of using needed
                Self::do_unstake_collator(amount, now)?;

                // TODO: Try mutate again
                RemainingUnstakingBalance::<T>::try_mutate(|&mut remaining| {
                    remaining -= amount;
                    (remaining > 0).then(remaining).ok_or(Err(
                        "not enough left to handle this request from current unstaking funds",
                    ))
                })?;
            })?;
            // END UNSTAKING SECTION

            // schedule payout after T::ReduceBondDelay expires
            // RAD: What happens if delegation_execute_scheduled_request fails?
            // TODO: pallet_scheduler::<T>::schedule(batch(delegation_execute_scheduled_request(),transfer_to_user))
            Self::deposit_event(Event::scheduled_withdraw(origin, amount));
            Ok(())
        }

        #[pallet::weight(0)]
        pub fn cancel_withdraw(origin: OriginFor<T>, amount: BalanceOf<T>) -> DispatchResult {
            ensure_signed(origin)?;

            ensure!(
                Self::not_in_drawing_freezeout,
                Error::<T>::TooCloseToDrawing
            );

            let some_collator = unchecked_account_id::<sr25519::Public>("Alice");

            // TODO: Mark funds as eligible again -> onboarding
            pallet_parachain_staking::Pallet::<T>::cancel_delegation_request(
                LotteryPot::get().into_account(),
                some_collator,
                amount,
            );

            // Revoke delegation of those funds
            // TODO: Once we have multiple collators we need a method to decide from whom to unbond
            pallet_parachain_staking::Pallet::<T>::delegation_schedule_bond_decrease(
                LotteryPot::get().into_account(),
                some_collator,
                amount,
            );
            // or
            // pallet_parachain_staking::Pallet::<T>::schedule_revoke_delegation(LotteryPot::into_account(),some_collator);

            // schedule payout after T::ReduceBondDelay expires
            // RAD: What happens if delegation_execute_scheduled_request fails?
            // TODO: pallet_scheduler::<T>::schedule(batch(delegation_execute_scheduled_request(),transfer_to_user))

            Self::deposit_event(Event::scheduled_withdraw(origin, amount));
            Ok(())
        }

        /// Rebalances stake by removing stake from overallocated collators and adding to underallocated
        ///
        /// It may be necessary to call this if large amounts of token become unstaked, e.g. due to a collator leaving
        #[pallet::weight(0)]
        pub fn rebalance_stake(origin: OriginFor<T>, amount: BalanceOf<T>) -> DispatchResult {
            Self::ensure_root_or_manager(origin)?;

            // withdraw from overallocated collators, wait until funds unlock, re-allocate to underallocated collators
            // RAD: This can run in parallel with a drawing, it will just reduce the staking revenue generated in this drawing by the amount of funds being rebalanced
            // TODO: find some balancing algorithm that does this

            // Self::deposit_event(Event::AddedToLottery(origin,amount));
            Ok(())
        }

        #[pallet::weight(0)]
        pub fn start_lottery(origin: OriginFor<T>) -> DispatchResult {
            Self::ensure_root_or_manager(origin)?;
            // TODO: Check that the pallet has enough funds to pay gas fees for at least the first drawing

            let now = <frame_system::Pallet<T>>::block_number();
            let drawing_interval = LotteryInterval::get();
            ensure!(drawing_interval > 0, Error::<T>::PalletMisconfigured);
            let drawing_scheduled_at = now + drawing_interval;
            let lottery_drawing_call = Call::draw_lottery { origin };

            // Schedule the draw lottery call to autoreschedule periodically, it will fail if ID already exists
            pallet_scheduler::Pallet::<T>::schedule_named(
                origin,
                LotteryPot::get().0,
                drawing_scheduled_at,
                drawing_interval,
                pallet_scheduler::pallet::<T>::Priority::LOWEST_PRIORITY,
                lottery_drawing_call,
            )?;

            Self::deposit_event(Event::LotteryStarted(drawing_scheduled_at));
            Ok(())
        }

        #[pallet::weight(0)]
        pub fn stop_lottery(origin: OriginFor<T>) -> DispatchResult {
            Self::ensure_root_or_manager(origin)?;

            ensure!(
                pallet_scheduler::Pallet::<T>::Lookup::contains_key(LotteryPot::get().0),
                Error::<T>::LotteryNotScheduled
            );

            let now = <frame_system::Pallet<T>>::block_number();

            pallet_scheduler::Pallet::<T>::cancel_named(origin, LotteryPot::get().0)
                .map_err(|_| Error::<T>::LotteryAlreadyStopped)?;

            Self::deposit_event(Event::LotteryStopped(now));
            Ok(())
        }

        #[pallet::weight(0)]
        pub fn draw_lottery(origin: OriginFor<T>) -> DispatchResult {
            let origin = ensure_signed(origin)?;
            ensure_eq!(origin, LotteryPot::get()); // Allow only this pallet to execute

            let pot_account_id = LotteryPot::get().into_account_truncating();

            let now = <frame_system::Pallet<T>>::block_number();
            ensure!(now >= Self::next_drawing(), Error::<T>::TooEarlyForDrawing);

            let funds_in_pot = T::Currency::total_balance(pot_account_id);
            let total_deposits = SumOfDeposits::<T>::get();

            // always keep some funds for gas
            // TODO: Convert to saturating math
            let payout = funds_in_pot - total_deposits - Self::gas_reserve();

            ensure!(payout > 0, Error::<T>::PotBalanceTooLow);
            ensure!(
                funds_in_pot - payout >= total_deposits,
                Error::<T>::PotBalanceTooLow
            );

            // Match random number to winner. We select a winning **balance** and then just add up accounts in the order they're stored until the sum of balance exceeds the winning amount
            // IMPORTANT: This order and active balances must be locked to modification after the random seed is created (relay BABE randomness, 2 epochs ago)
            let random = T::Randomness::random();
            let winning_balance = random % Self::Pallet::<T>::total_pot();
            let mut winner: Option<T::AccountId> = None;
            let mut count = 0;

            for (account, balance) in T::ActiveBalancePerUser::iter() {
                count += balance;
                if count >= winning_balance {
                    winner = Some(account);
                    break;
                }
            }
            // Should be impossible: If no winner was selected, return Error
            ensure!(winner.is_some(), Err::<T>::NoWinnerFound);

            T::Currency::transfer(Self::account_id, winner, payout, KeepAlive)?;
            Self::deposit_event(Event::LotteryWinner(winner));

            // TODO: Update bookkeeping
            NextDrawingAt::<T>::set(now + Self::lottery_interval());

            Self::update_active_funds();
            Self::finish_unstaking_collators();
            Self::schedule_withdrawal_payouts();
            Self::rebalance_remaining_funds(origin);
            Ok()
        }
    }

    impl<T: Config> Pallet<T> {
        /// Get a unique, inaccessible account id from the `PotId`.
        fn account_id() -> T::AccountId {
            T::LotteryPot::get().into_account_truncating()
        }

        fn do_stake(amount: BalanceOf<T>) -> DispatchResult {
            // TODO: choose collator, split amount over multiple?
            let some_collator = unchecked_account_id::<sr25519::Public>("Alice");
            pallet_parachain_staking::Pallet::<T>::delegate(
                LotteryPot::get().into_account(),
                some_collator,
                amount,
            )
        }

        fn do_unstake_collator(amount: BalanceOf<T>, now: BlockNumber) -> DispatchResult {
            let some_collator = unchecked_account_id::<sr25519::Public>("Alice");

            // TODO: Find the smallest currently active delegation larger than `amount`
            let delegated_amount_to_be_unstaked = 0;

            // TODO: If none can be found, find a combination of collators so it can
            // TODO: If it still can't be found, either there's a logic bug or we've been hacked

            // unstake from parachain staking
            // NOTE: All funds that were delegated here no longer produce staking rewards
            pallet_parachain_staking::Pallet::<T>::schedule_revoke_delegation(
                LotteryPot::get().into_account(),
                some_collator,
            )?;
            // TODO: Error handling

            // TODO: Update remaining balance
            RemainingUnstakingBalance::<T>::mutate(|bal| bal += delegated_amount_to_be_unstaked);

            // update unstaking storage
            UnstakingCollators::<T>::mutate(|collators| {
                collators.push(UnstakingCollator {
                    account: some_collator,
                    since: now,
                })
            });

            // unstake from parachain staking
            // NOTE: All funds that were delegated here no longer produce staking rewards
            pallet_parachain_staking::Pallet::<T>::schedule_revoke_delegation(
                LotteryPot::get().into_account(),
                some_collator,
            )?
        }

        fn update_active_funds() -> DispatchResult {
            // TODO: move cancelled offboarding funds to active
            Ok(())
        }

        /// NOTE: This code assumes UnstakingCollators is sorted
        fn finish_unstaking_collators(origin: OriginFor<T>) {
            Self::ensure_root_or_manager(origin);

            let now = <frame_system::Pallet<T>>::block_number();
            let unstaking = UnstakingCollators::<T>::get();
            UnstakingCollators::<T>::try_mutate(|unstaking| {
                let remaining_collators = unstaking.filter_map(|collator| {
                    // only attempt to resolve fully unstaked collators
                    if collator.since + UnstakingTime::<T>::get() > now {
                        Some(collator)
                    };
                    // There can only be one request per collator and it is always a full revoke_delegation call
                    match pallet_parachain_staking::Pallet::<T>::execute_delegation_request(
                        origin,
                        LotteryPot::get().into_account_truncating(),
                        collator,
                    ){
                        Ok(_) => {
                            // collator was unstaked, remove from unstaking collators
                             None
                        },
                        Err(e) => {
                            log::error!("Collator finished unstaking timelock but could not be removed with error {:?}",e);
                            Some(collator)
                        },
                    };
                });
                if remaining_collators.len() != unstaking.len() {
                    unstaking = remaining_collators;
                    Ok()
                } else {
                    Err()
                }
            });
        }

        pub fn rebalance_remaining_funds(origin: OriginFor<T>) {
            Self::ensure_root_or_manager(origin);

            let pot_account_id = LotteryPot::get().into_account_truncating();
            let available_balance = T::Currency::free_balance(pot_account_id);
            let stakable_balance = available_balance - T::gas_reserve();

            // TODO: Calculate these from current values
            const candidate_delegation_count: u32 = 500;
            const delegation_count: u32 = 500;

            // TODO: Find highest APY for this deposit (possibly balance deposit to multiple collators)
            let some_output: Vec<(T::AccountId, BalanceOf<T>)> = vec![(
                unchecked_account_id::<sr25519::Public>("Alice"),
                stakable_balance,
            )];

            // Stake it to one or more collators
            for (collator, balance) in some_output {
                pallet_parachain_staking::Pallet::<T>::delegate(
                    origin,
                    collator,
                    balance,
                    candidate_delegation_count,
                    delegation_count,
                )
                .ok_or_else(|e| {
                    log::error!("Could not delegate to collator with error {:?}", e);
                });
            }
        }

        /// This fn schedules a single shot payout of all matured withdrawals
        /// It is meant to be executed in the course of a drawing
        fn schedule_withdrawal_payouts(origin: OriginFor<T>) -> DispatchResult {
            let some_collator = unchecked_account_id::<sr25519::Public>("Alice");
            let now = <frame_system::Pallet<T>>::block_number();

            T::WithdrawalRequestQueue::try_mutate(|request_vec| {
                ensure!(request_vec.length() > 0, Error::<T>::WithdrawBelowMinAmount);
                let left_overs;
                for request in request_vec {
                    if now < request.block + T::UnstakeTime::get() {
                        // too early to withdraw this request
                        left_overs.push(request);
                        continue;
                    }
                    Pallet::<T>::SumOfDeposits::mutate(|sum| sum - request.amount);

                    // TODO: immediately execute unstake
                    pallet_parachain_staking::Pallet::<T>::execute_delegation_request(
                        origin,
                        LotteryPot::get().into_account_truncating(),
                        some_collator,
                    );

                    // TODO: schedule transfer offboarded funds to owner
                    let res = T::Currency::transfer(
                        &LotteryPot::get().into_account(),
                        &request.user,
                        request.amount,
                        KeepAlive,
                    );
                }
                // Update T::WithdrawalRequestQueue if changed
                if left_overs.len() {
                    request_vec = left_overs;
                    Ok(())
                } else {
                    Err("no changes")
                }
            });
            Ok(())
        }

        pub(super) fn not_in_drawing_freezeout() -> bool {
            let now = <frame_system::Pallet<T>>::block_number();
            now < Self::next_drawing().saturating_sub(Self::drawing_buffer())
        }

        pub(super) fn ensure_root_or_manager<T: Config>(origin: OriginFor<T>) -> DispatchResult {
            ensure!(
                // TODO: Schedule origin must be council and root, not root only
                frame_system::ensure_root(origin).is_ok(),
                // frame_system::ensure_root(origin) || T::ManageOrigin::ensure_origin(origin)
                Error::<T>::OnlyRootOrManageOrigin
            );
            Ok(())
        }
    }
}
