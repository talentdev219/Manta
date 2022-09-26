// Copyright 2020-2022 Manta Network.
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

//! Autogenerated weights for parachain_staking
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-05-23, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/moonbeam
// benchmark
// pallet
// --chain
// dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// parachain_staking
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --template=./benchmarking/frame-weight-template.hbs
// --record-proof
// --json-file
// raw.json
// --output
// weights.rs

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
    traits::Get,
    weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for parachain_staking.
pub trait WeightInfo {
    #[rustfmt::skip]
	fn set_staking_expectations() -> Weight;
    #[rustfmt::skip]
	fn set_inflation() -> Weight;
    #[rustfmt::skip]
	fn set_parachain_bond_account() -> Weight;
    #[rustfmt::skip]
	fn set_parachain_bond_reserve_percent() -> Weight;
    #[rustfmt::skip]
	fn set_total_selected() -> Weight;
    #[rustfmt::skip]
	fn set_collator_commission() -> Weight;
    #[rustfmt::skip]
	fn set_blocks_per_round() -> Weight;
    #[rustfmt::skip]
	fn join_candidates(x: u32, ) -> Weight;
    #[rustfmt::skip]
	fn schedule_leave_candidates(x: u32, ) -> Weight;
    #[rustfmt::skip]
	fn execute_leave_candidates(x: u32, ) -> Weight;
    #[rustfmt::skip]
	fn cancel_leave_candidates(x: u32, ) -> Weight;
    #[rustfmt::skip]
	fn go_offline() -> Weight;
    #[rustfmt::skip]
	fn go_online() -> Weight;
    #[rustfmt::skip]
	fn candidate_bond_more() -> Weight;
    #[rustfmt::skip]
	fn schedule_candidate_bond_less() -> Weight;
    #[rustfmt::skip]
	fn execute_candidate_bond_less() -> Weight;
    #[rustfmt::skip]
	fn cancel_candidate_bond_less() -> Weight;
    #[rustfmt::skip]
	fn delegate(x: u32, y: u32, ) -> Weight;
    #[rustfmt::skip]
	fn schedule_leave_delegators() -> Weight;
    #[rustfmt::skip]
	fn execute_leave_delegators(x: u32, ) -> Weight;
    #[rustfmt::skip]
	fn cancel_leave_delegators() -> Weight;
    #[rustfmt::skip]
	fn schedule_revoke_delegation() -> Weight;
    #[rustfmt::skip]
	fn delegator_bond_more() -> Weight;
    #[rustfmt::skip]
	fn schedule_delegator_bond_less() -> Weight;
    #[rustfmt::skip]
	fn execute_revoke_delegation() -> Weight;
    #[rustfmt::skip]
	fn execute_delegator_bond_less() -> Weight;
    #[rustfmt::skip]
	fn cancel_revoke_delegation() -> Weight;
    #[rustfmt::skip]
	fn cancel_delegator_bond_less() -> Weight;
    #[rustfmt::skip]
	fn round_transition_on_initialize(x: u32, y: u32, ) -> Weight;
    #[rustfmt::skip]
	fn pay_one_collator_reward(y: u32, ) -> Weight;
    #[rustfmt::skip]
	fn base_on_initialize() -> Weight;
}

/// Weights for parachain_staking using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_parachain_staking::WeightInfo for SubstrateWeight<T> {
    // Storage: ParachainStaking InflationConfig (r:1 w:1)
	#[rustfmt::skip]
    fn set_staking_expectations() -> Weight {
		(18_520_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
    // Storage: ParachainStaking InflationConfig (r:1 w:1)
	#[rustfmt::skip]
    fn set_inflation() -> Weight {
		(53_196_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
    // Storage: ParachainStaking ParachainBondInfo (r:1 w:1)
	#[rustfmt::skip]
    fn set_parachain_bond_account() -> Weight {
		(18_168_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
    // Storage: ParachainStaking ParachainBondInfo (r:1 w:1)
	#[rustfmt::skip]
    fn set_parachain_bond_reserve_percent() -> Weight {
		(17_397_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
    // Storage: ParachainStaking TotalSelected (r:1 w:1)
	#[rustfmt::skip]
    fn set_total_selected() -> Weight {
		(20_404_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
    // Storage: ParachainStaking CollatorCommission (r:1 w:1)
	#[rustfmt::skip]
    fn set_collator_commission() -> Weight {
		(17_036_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
    // Storage: ParachainStaking TotalSelected (r:1 w:0)
    // Storage: ParachainStaking InflationConfig (r:1 w:1)
	#[rustfmt::skip]
    fn set_blocks_per_round() -> Weight {
		(57_722_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
    // Storage: ParachainStaking CandidateInfo (r:1 w:1)
    // Storage: ParachainStaking DelegatorState (r:1 w:0)
    // Storage: ParachainStaking CandidatePool (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    // Storage: ParachainStaking Total (r:1 w:1)
    // Storage: ParachainStaking TopDelegations (r:0 w:1)
    // Storage: ParachainStaking BottomDelegations (r:0 w:1)
	#[rustfmt::skip]
    fn join_candidates(x: u32, ) -> Weight {
		(70_223_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((69_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
    // Storage: ParachainStaking CandidateInfo (r:1 w:1)
    // Storage: ParachainStaking CandidatePool (r:1 w:1)
	#[rustfmt::skip]
    fn schedule_leave_candidates(x: u32, ) -> Weight {
		(53_351_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((66_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
    // Storage: ParachainStaking CandidateInfo (r:1 w:1)
    // Storage: ParachainStaking TopDelegations (r:1 w:1)
    // Storage: System Account (r:2 w:2)
    // Storage: ParachainStaking DelegatorState (r:1 w:1)
    // Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
    // Storage: ParachainStaking BottomDelegations (r:1 w:1)
    // Storage: ParachainStaking Total (r:1 w:1)
	#[rustfmt::skip]
    fn execute_leave_candidates(x: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 99_000
			.saturating_add((33_350_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().reads((2 as Weight).saturating_mul(x as Weight)))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
			.saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(x as Weight)))
	}
    // Storage: ParachainStaking CandidateInfo (r:1 w:1)
    // Storage: ParachainStaking CandidatePool (r:1 w:1)
	#[rustfmt::skip]
    fn cancel_leave_candidates(x: u32, ) -> Weight {
		(50_534_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((66_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
    // Storage: ParachainStaking CandidateInfo (r:1 w:1)
    // Storage: ParachainStaking CandidatePool (r:1 w:1)
	#[rustfmt::skip]
    fn go_offline() -> Weight {
		(27_716_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
    // Storage: ParachainStaking CandidateInfo (r:1 w:1)
    // Storage: ParachainStaking CandidatePool (r:1 w:1)
	#[rustfmt::skip]
    fn go_online() -> Weight {
		(27_463_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
    // Storage: ParachainStaking CandidateInfo (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    // Storage: ParachainStaking Total (r:1 w:1)
    // Storage: ParachainStaking CandidatePool (r:1 w:1)
	#[rustfmt::skip]
    fn candidate_bond_more() -> Weight {
		(45_671_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
    // Storage: ParachainStaking CandidateInfo (r:1 w:1)
	#[rustfmt::skip]
    fn schedule_candidate_bond_less() -> Weight {
		(26_710_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
    // Storage: ParachainStaking CandidateInfo (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    // Storage: ParachainStaking Total (r:1 w:1)
    // Storage: ParachainStaking CandidatePool (r:1 w:1)
	#[rustfmt::skip]
    fn execute_candidate_bond_less() -> Weight {
		(54_648_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
    // Storage: ParachainStaking CandidateInfo (r:1 w:1)
	#[rustfmt::skip]
    fn cancel_candidate_bond_less() -> Weight {
		(22_585_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
    // Storage: System Account (r:1 w:1)
    // Storage: ParachainStaking DelegatorState (r:1 w:1)
    // Storage: ParachainStaking CandidateInfo (r:1 w:1)
    // Storage: ParachainStaking TopDelegations (r:1 w:1)
    // Storage: ParachainStaking CandidatePool (r:1 w:1)
    // Storage: ParachainStaking Total (r:1 w:1)
	#[rustfmt::skip]
    fn delegate(x: u32, y: u32, ) -> Weight {
		(81_464_000 as Weight)
			// Standard Error: 7_000
			.saturating_add((273_000 as Weight).saturating_mul(x as Weight))
			// Standard Error: 2_000
			.saturating_add((156_000 as Weight).saturating_mul(y as Weight))
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
    // Storage: ParachainStaking DelegatorState (r:1 w:1)
    // Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	#[rustfmt::skip]
    fn schedule_leave_delegators() -> Weight {
		(32_060_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
    // Storage: ParachainStaking DelegatorState (r:1 w:1)
    // Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
    // Storage: ParachainStaking CandidateInfo (r:1 w:1)
    // Storage: ParachainStaking TopDelegations (r:1 w:1)
    // Storage: ParachainStaking CandidatePool (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    // Storage: ParachainStaking Total (r:1 w:1)
	#[rustfmt::skip]
    fn execute_leave_delegators(x: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 44_000
			.saturating_add((39_035_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(x as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(x as Weight)))
	}
    // Storage: ParachainStaking DelegatorState (r:1 w:1)
    // Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	#[rustfmt::skip]
    fn cancel_leave_delegators() -> Weight {
		(31_955_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
    // Storage: ParachainStaking DelegatorState (r:1 w:1)
    // Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	#[rustfmt::skip]
    fn schedule_revoke_delegation() -> Weight {
		(31_793_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
    // Storage: ParachainStaking DelegationScheduledRequests (r:1 w:0)
    // Storage: ParachainStaking DelegatorState (r:1 w:1)
    // Storage: ParachainStaking CandidateInfo (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    // Storage: ParachainStaking TopDelegations (r:1 w:1)
    // Storage: ParachainStaking CandidatePool (r:1 w:1)
    // Storage: ParachainStaking Total (r:1 w:1)
	#[rustfmt::skip]
    fn delegator_bond_more() -> Weight {
		(66_476_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
    // Storage: ParachainStaking DelegatorState (r:1 w:1)
    // Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	#[rustfmt::skip]
    fn schedule_delegator_bond_less() -> Weight {
		(31_467_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
    // Storage: ParachainStaking DelegatorState (r:1 w:1)
    // Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
    // Storage: ParachainStaking CandidateInfo (r:1 w:1)
    // Storage: ParachainStaking TopDelegations (r:1 w:1)
    // Storage: ParachainStaking CandidatePool (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    // Storage: ParachainStaking Total (r:1 w:1)
	#[rustfmt::skip]
    fn execute_revoke_delegation() -> Weight {
		(82_319_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
    // Storage: ParachainStaking DelegatorState (r:1 w:1)
    // Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
    // Storage: ParachainStaking CandidateInfo (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    // Storage: ParachainStaking TopDelegations (r:1 w:1)
    // Storage: ParachainStaking CandidatePool (r:1 w:1)
    // Storage: ParachainStaking Total (r:1 w:1)
	#[rustfmt::skip]
    fn execute_delegator_bond_less() -> Weight {
		(75_207_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
    // Storage: ParachainStaking DelegatorState (r:1 w:1)
    // Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	#[rustfmt::skip]
    fn cancel_revoke_delegation() -> Weight {
		(29_497_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
    // Storage: ParachainStaking DelegatorState (r:1 w:1)
    // Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	#[rustfmt::skip]
    fn cancel_delegator_bond_less() -> Weight {
		(35_951_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
    // Storage: ParachainStaking Points (r:1 w:0)
    // Storage: ParachainStaking Staked (r:1 w:2)
    // Storage: ParachainStaking InflationConfig (r:1 w:0)
    // Storage: ParachainStaking ParachainBondInfo (r:1 w:0)
    // Storage: System Account (r:302 w:301)
    // Storage: ParachainStaking CollatorCommission (r:1 w:0)
    // Storage: ParachainStaking CandidatePool (r:1 w:0)
    // Storage: ParachainStaking TotalSelected (r:1 w:0)
    // Storage: ParachainStaking CandidateInfo (r:9 w:0)
    // Storage: ParachainStaking DelegationScheduledRequests (r:9 w:0)
    // Storage: ParachainStaking TopDelegations (r:9 w:0)
    // Storage: ParachainStaking Total (r:1 w:0)
    // Storage: ParachainStaking AwardedPts (r:2 w:1)
    // Storage: ParachainStaking AtStake (r:1 w:10)
    // Storage: MoonbeamOrbiters OrbiterPerRound (r:1 w:0)
    // Storage: MoonbeamOrbiters CurrentRound (r:0 w:1)
    // Storage: ParachainStaking SelectedCandidates (r:0 w:1)
    // Storage: ParachainStaking DelayedPayouts (r:0 w:1)
	#[rustfmt::skip]
    fn round_transition_on_initialize(x: u32, y: u32, ) -> Weight {
		(5_114_000 as Weight)
			// Standard Error: 796_000
			.saturating_add((52_172_000 as Weight).saturating_mul(x as Weight))
			// Standard Error: 2_000
			.saturating_add((156_000 as Weight).saturating_mul(y as Weight))
			.saturating_add(T::DbWeight::get().reads(195 as Weight))
			.saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(x as Weight)))
			.saturating_add(T::DbWeight::get().writes(188 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(x as Weight)))
	}
    // Storage: ParachainStaking DelayedPayouts (r:1 w:0)
    // Storage: ParachainStaking Points (r:1 w:0)
    // Storage: ParachainStaking AwardedPts (r:2 w:1)
    // Storage: ParachainStaking AtStake (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    // Storage: MoonbeamOrbiters OrbiterPerRound (r:1 w:0)
	#[rustfmt::skip]
    fn pay_one_collator_reward(y: u32, ) -> Weight {
		(49_798_000 as Weight)
			// Standard Error: 9_000
			.saturating_add((16_304_000 as Weight).saturating_mul(y as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(y as Weight)))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(y as Weight)))
	}
	#[rustfmt::skip]
    fn base_on_initialize() -> Weight {
		(4_762_000 as Weight)
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
    // Storage: ParachainStaking InflationConfig (r:1 w:1)
	#[rustfmt::skip]
    fn set_staking_expectations() -> Weight {
		(18_520_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
    // Storage: ParachainStaking InflationConfig (r:1 w:1)
	#[rustfmt::skip]
    fn set_inflation() -> Weight {
		(53_196_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
    // Storage: ParachainStaking ParachainBondInfo (r:1 w:1)
	#[rustfmt::skip]
    fn set_parachain_bond_account() -> Weight {
		(18_168_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
    // Storage: ParachainStaking ParachainBondInfo (r:1 w:1)
	#[rustfmt::skip]
    fn set_parachain_bond_reserve_percent() -> Weight {
		(17_397_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
    // Storage: ParachainStaking TotalSelected (r:1 w:1)
	#[rustfmt::skip]
    fn set_total_selected() -> Weight {
		(20_404_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
    // Storage: ParachainStaking CollatorCommission (r:1 w:1)
	#[rustfmt::skip]
    fn set_collator_commission() -> Weight {
		(17_036_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
    // Storage: ParachainStaking TotalSelected (r:1 w:0)
    // Storage: ParachainStaking InflationConfig (r:1 w:1)
	#[rustfmt::skip]
    fn set_blocks_per_round() -> Weight {
		(57_722_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
    // Storage: ParachainStaking CandidateInfo (r:1 w:1)
    // Storage: ParachainStaking DelegatorState (r:1 w:0)
    // Storage: ParachainStaking CandidatePool (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    // Storage: ParachainStaking Total (r:1 w:1)
    // Storage: ParachainStaking TopDelegations (r:0 w:1)
    // Storage: ParachainStaking BottomDelegations (r:0 w:1)
	#[rustfmt::skip]
    fn join_candidates(x: u32, ) -> Weight {
		(70_223_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((69_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(6 as Weight))
	}
    // Storage: ParachainStaking CandidateInfo (r:1 w:1)
    // Storage: ParachainStaking CandidatePool (r:1 w:1)
	#[rustfmt::skip]
    fn schedule_leave_candidates(x: u32, ) -> Weight {
		(53_351_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((66_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
    // Storage: ParachainStaking CandidateInfo (r:1 w:1)
    // Storage: ParachainStaking TopDelegations (r:1 w:1)
    // Storage: System Account (r:2 w:2)
    // Storage: ParachainStaking DelegatorState (r:1 w:1)
    // Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
    // Storage: ParachainStaking BottomDelegations (r:1 w:1)
    // Storage: ParachainStaking Total (r:1 w:1)
	#[rustfmt::skip]
    fn execute_leave_candidates(x: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 99_000
			.saturating_add((33_350_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().reads((2 as Weight).saturating_mul(x as Weight)))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes((2 as Weight).saturating_mul(x as Weight)))
	}
    // Storage: ParachainStaking CandidateInfo (r:1 w:1)
    // Storage: ParachainStaking CandidatePool (r:1 w:1)
	#[rustfmt::skip]
    fn cancel_leave_candidates(x: u32, ) -> Weight {
		(50_534_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((66_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
    // Storage: ParachainStaking CandidateInfo (r:1 w:1)
    // Storage: ParachainStaking CandidatePool (r:1 w:1)
	#[rustfmt::skip]
    fn go_offline() -> Weight {
		(27_716_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
    // Storage: ParachainStaking CandidateInfo (r:1 w:1)
    // Storage: ParachainStaking CandidatePool (r:1 w:1)
	#[rustfmt::skip]
    fn go_online() -> Weight {
		(27_463_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
    // Storage: ParachainStaking CandidateInfo (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    // Storage: ParachainStaking Total (r:1 w:1)
    // Storage: ParachainStaking CandidatePool (r:1 w:1)
	#[rustfmt::skip]
    fn candidate_bond_more() -> Weight {
		(45_671_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
    // Storage: ParachainStaking CandidateInfo (r:1 w:1)
	#[rustfmt::skip]
    fn schedule_candidate_bond_less() -> Weight {
		(26_710_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
    // Storage: ParachainStaking CandidateInfo (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    // Storage: ParachainStaking Total (r:1 w:1)
    // Storage: ParachainStaking CandidatePool (r:1 w:1)
	#[rustfmt::skip]
    fn execute_candidate_bond_less() -> Weight {
		(54_648_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
    // Storage: ParachainStaking CandidateInfo (r:1 w:1)
	#[rustfmt::skip]
    fn cancel_candidate_bond_less() -> Weight {
		(22_585_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
    // Storage: System Account (r:1 w:1)
    // Storage: ParachainStaking DelegatorState (r:1 w:1)
    // Storage: ParachainStaking CandidateInfo (r:1 w:1)
    // Storage: ParachainStaking TopDelegations (r:1 w:1)
    // Storage: ParachainStaking CandidatePool (r:1 w:1)
    // Storage: ParachainStaking Total (r:1 w:1)
	#[rustfmt::skip]
    fn delegate(x: u32, y: u32, ) -> Weight {
		(81_464_000 as Weight)
			// Standard Error: 7_000
			.saturating_add((273_000 as Weight).saturating_mul(x as Weight))
			// Standard Error: 2_000
			.saturating_add((156_000 as Weight).saturating_mul(y as Weight))
			.saturating_add(RocksDbWeight::get().reads(6 as Weight))
			.saturating_add(RocksDbWeight::get().writes(6 as Weight))
	}
    // Storage: ParachainStaking DelegatorState (r:1 w:1)
    // Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	#[rustfmt::skip]
    fn schedule_leave_delegators() -> Weight {
		(32_060_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
    // Storage: ParachainStaking DelegatorState (r:1 w:1)
    // Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
    // Storage: ParachainStaking CandidateInfo (r:1 w:1)
    // Storage: ParachainStaking TopDelegations (r:1 w:1)
    // Storage: ParachainStaking CandidatePool (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    // Storage: ParachainStaking Total (r:1 w:1)
	#[rustfmt::skip]
    fn execute_leave_delegators(x: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 44_000
			.saturating_add((39_035_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().reads((3 as Weight).saturating_mul(x as Weight)))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes((3 as Weight).saturating_mul(x as Weight)))
	}
    // Storage: ParachainStaking DelegatorState (r:1 w:1)
    // Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	#[rustfmt::skip]
    fn cancel_leave_delegators() -> Weight {
		(31_955_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
    // Storage: ParachainStaking DelegatorState (r:1 w:1)
    // Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	#[rustfmt::skip]
    fn schedule_revoke_delegation() -> Weight {
		(31_793_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
    // Storage: ParachainStaking DelegationScheduledRequests (r:1 w:0)
    // Storage: ParachainStaking DelegatorState (r:1 w:1)
    // Storage: ParachainStaking CandidateInfo (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    // Storage: ParachainStaking TopDelegations (r:1 w:1)
    // Storage: ParachainStaking CandidatePool (r:1 w:1)
    // Storage: ParachainStaking Total (r:1 w:1)
	#[rustfmt::skip]
    fn delegator_bond_more() -> Weight {
		(66_476_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(7 as Weight))
			.saturating_add(RocksDbWeight::get().writes(6 as Weight))
	}
    // Storage: ParachainStaking DelegatorState (r:1 w:1)
    // Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	#[rustfmt::skip]
    fn schedule_delegator_bond_less() -> Weight {
		(31_467_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
    // Storage: ParachainStaking DelegatorState (r:1 w:1)
    // Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
    // Storage: ParachainStaking CandidateInfo (r:1 w:1)
    // Storage: ParachainStaking TopDelegations (r:1 w:1)
    // Storage: ParachainStaking CandidatePool (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    // Storage: ParachainStaking Total (r:1 w:1)
	#[rustfmt::skip]
    fn execute_revoke_delegation() -> Weight {
		(82_319_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(7 as Weight))
			.saturating_add(RocksDbWeight::get().writes(7 as Weight))
	}
    // Storage: ParachainStaking DelegatorState (r:1 w:1)
    // Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
    // Storage: ParachainStaking CandidateInfo (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    // Storage: ParachainStaking TopDelegations (r:1 w:1)
    // Storage: ParachainStaking CandidatePool (r:1 w:1)
    // Storage: ParachainStaking Total (r:1 w:1)
	#[rustfmt::skip]
    fn execute_delegator_bond_less() -> Weight {
		(75_207_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(7 as Weight))
			.saturating_add(RocksDbWeight::get().writes(7 as Weight))
	}
    // Storage: ParachainStaking DelegatorState (r:1 w:1)
    // Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	#[rustfmt::skip]
    fn cancel_revoke_delegation() -> Weight {
		(29_497_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
    // Storage: ParachainStaking DelegatorState (r:1 w:1)
    // Storage: ParachainStaking DelegationScheduledRequests (r:1 w:1)
	#[rustfmt::skip]
    fn cancel_delegator_bond_less() -> Weight {
		(35_951_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
    // Storage: ParachainStaking Points (r:1 w:0)
    // Storage: ParachainStaking Staked (r:1 w:2)
    // Storage: ParachainStaking InflationConfig (r:1 w:0)
    // Storage: ParachainStaking ParachainBondInfo (r:1 w:0)
    // Storage: System Account (r:302 w:301)
    // Storage: ParachainStaking CollatorCommission (r:1 w:0)
    // Storage: ParachainStaking CandidatePool (r:1 w:0)
    // Storage: ParachainStaking TotalSelected (r:1 w:0)
    // Storage: ParachainStaking CandidateInfo (r:9 w:0)
    // Storage: ParachainStaking DelegationScheduledRequests (r:9 w:0)
    // Storage: ParachainStaking TopDelegations (r:9 w:0)
    // Storage: ParachainStaking Total (r:1 w:0)
    // Storage: ParachainStaking AwardedPts (r:2 w:1)
    // Storage: ParachainStaking AtStake (r:1 w:10)
    // Storage: MoonbeamOrbiters OrbiterPerRound (r:1 w:0)
    // Storage: MoonbeamOrbiters CurrentRound (r:0 w:1)
    // Storage: ParachainStaking SelectedCandidates (r:0 w:1)
    // Storage: ParachainStaking DelayedPayouts (r:0 w:1)
	#[rustfmt::skip]
    fn round_transition_on_initialize(x: u32, y: u32, ) -> Weight {
		(5_114_000 as Weight)
			// Standard Error: 796_000
			.saturating_add((52_172_000 as Weight).saturating_mul(x as Weight))
			// Standard Error: 2_000
			.saturating_add((156_000 as Weight).saturating_mul(y as Weight))
			.saturating_add(RocksDbWeight::get().reads(195 as Weight))
			.saturating_add(RocksDbWeight::get().reads((3 as Weight).saturating_mul(x as Weight)))
			.saturating_add(RocksDbWeight::get().writes(188 as Weight))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(x as Weight)))
	}
    // Storage: ParachainStaking DelayedPayouts (r:1 w:0)
    // Storage: ParachainStaking Points (r:1 w:0)
    // Storage: ParachainStaking AwardedPts (r:2 w:1)
    // Storage: ParachainStaking AtStake (r:1 w:1)
    // Storage: System Account (r:1 w:1)
    // Storage: MoonbeamOrbiters OrbiterPerRound (r:1 w:0)
	#[rustfmt::skip]
    fn pay_one_collator_reward(y: u32, ) -> Weight {
		(49_798_000 as Weight)
			// Standard Error: 9_000
			.saturating_add((16_304_000 as Weight).saturating_mul(y as Weight))
			.saturating_add(RocksDbWeight::get().reads(7 as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(y as Weight)))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(y as Weight)))
	}
	#[rustfmt::skip]
    fn base_on_initialize() -> Weight {
		(4_762_000 as Weight)
	}
}
