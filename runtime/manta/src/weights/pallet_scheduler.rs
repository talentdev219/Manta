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

//! Autogenerated weights for pallet_scheduler
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-05, STEPS: `50`, REPEAT: 40, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("manta-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/manta
// benchmark
// pallet
// --chain=manta-dev
// --steps=50
// --repeat=40
// --pallet=pallet_scheduler
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./scripts/benchmarking/frame-weights-output/pallet_scheduler.rs
// --template=.github/resources/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;
use manta_primitives::constants::RocksDbWeight;

/// Weight functions needed for pallet_scheduler.
pub trait WeightInfo {
    fn service_agendas_base() -> Weight;
    fn service_agenda_base(s: u32, ) -> Weight;
    fn service_task_base() -> Weight;
    fn service_task_fetched(s: u32, ) -> Weight;
    fn service_task_named() -> Weight;
    fn service_task_periodic() -> Weight;
    fn execute_dispatch_signed() -> Weight;
    fn execute_dispatch_unsigned() -> Weight;
    fn schedule(s: u32, ) -> Weight;
    fn cancel(s: u32, ) -> Weight;
    fn schedule_named(s: u32, ) -> Weight;
    fn cancel_named(s: u32, ) -> Weight;
}

/// Weights for pallet_scheduler using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_scheduler::WeightInfo for SubstrateWeight<T> {
	// Storage: Scheduler IncompleteSince (r:1 w:1)
	fn service_agendas_base() -> Weight {
		// Minimum execution time: 4_845 nanoseconds.
		Weight::from_ref_time(4_939_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Scheduler Agenda (r:1 w:1)
	/// The range of component `s` is `[0, 50]`.
	fn service_agenda_base(s: u32, ) -> Weight {
		// Minimum execution time: 8_208 nanoseconds.
		Weight::from_ref_time(8_529_104)
			// Standard Error: 2_697
			.saturating_add(Weight::from_ref_time(770_288).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	fn service_task_base() -> Weight {
		// Minimum execution time: 10_777 nanoseconds.
		Weight::from_ref_time(10_958_000)
	}
	// Storage: Preimage PreimageFor (r:1 w:1)
	// Storage: Preimage StatusFor (r:1 w:1)
	/// The range of component `s` is `[128, 4194304]`.
	fn service_task_fetched(s: u32, ) -> Weight {
		// Minimum execution time: 23_227 nanoseconds.
		Weight::from_ref_time(23_594_000)
			// Standard Error: 5
			.saturating_add(Weight::from_ref_time(1_795).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Scheduler Lookup (r:0 w:1)
	fn service_task_named() -> Weight {
		// Minimum execution time: 24_717 nanoseconds.
		Weight::from_ref_time(37_757_000)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	fn service_task_periodic() -> Weight {
		// Minimum execution time: 14_797 nanoseconds.
		Weight::from_ref_time(15_222_000)
	}
	fn execute_dispatch_signed() -> Weight {
		// Minimum execution time: 6_692 nanoseconds.
		Weight::from_ref_time(6_886_000)
	}
	fn execute_dispatch_unsigned() -> Weight {
		// Minimum execution time: 4_840 nanoseconds.
		Weight::from_ref_time(4_946_000)
	}
	// Storage: Scheduler Agenda (r:1 w:1)
	/// The range of component `s` is `[0, 49]`.
	fn schedule(s: u32, ) -> Weight {
		// Minimum execution time: 18_608 nanoseconds.
		Weight::from_ref_time(23_175_287)
			// Standard Error: 3_173
			.saturating_add(Weight::from_ref_time(846_144).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Scheduler Agenda (r:1 w:1)
	// Storage: Scheduler Lookup (r:0 w:1)
	/// The range of component `s` is `[1, 50]`.
	fn cancel(s: u32, ) -> Weight {
		// Minimum execution time: 23_332 nanoseconds.
		Weight::from_ref_time(24_383_594)
			// Standard Error: 4_101
			.saturating_add(Weight::from_ref_time(1_412_848).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Scheduler Lookup (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	/// The range of component `s` is `[0, 49]`.
	fn schedule_named(s: u32, ) -> Weight {
		// Minimum execution time: 21_762 nanoseconds.
		Weight::from_ref_time(26_872_288)
			// Standard Error: 3_676
			.saturating_add(Weight::from_ref_time(883_404).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Scheduler Lookup (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	/// The range of component `s` is `[1, 50]`.
	fn cancel_named(s: u32, ) -> Weight {
		// Minimum execution time: 23_694 nanoseconds.
		Weight::from_ref_time(26_729_328)
			// Standard Error: 3_595
			.saturating_add(Weight::from_ref_time(1_422_442).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Scheduler IncompleteSince (r:1 w:1)
	fn service_agendas_base() -> Weight {
		// Minimum execution time: 4_845 nanoseconds.
		Weight::from_ref_time(4_939_000)
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: Scheduler Agenda (r:1 w:1)
	/// The range of component `s` is `[0, 50]`.
	fn service_agenda_base(s: u32, ) -> Weight {
		// Minimum execution time: 8_208 nanoseconds.
		Weight::from_ref_time(8_529_104)
			// Standard Error: 2_697
			.saturating_add(Weight::from_ref_time(770_288).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	fn service_task_base() -> Weight {
		// Minimum execution time: 10_777 nanoseconds.
		Weight::from_ref_time(10_958_000)
	}
	// Storage: Preimage PreimageFor (r:1 w:1)
	// Storage: Preimage StatusFor (r:1 w:1)
	/// The range of component `s` is `[128, 4194304]`.
	fn service_task_fetched(s: u32, ) -> Weight {
		// Minimum execution time: 23_227 nanoseconds.
		Weight::from_ref_time(23_594_000)
			// Standard Error: 5
			.saturating_add(Weight::from_ref_time(1_795).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(2))
	}
	// Storage: Scheduler Lookup (r:0 w:1)
	fn service_task_named() -> Weight {
		// Minimum execution time: 24_717 nanoseconds.
		Weight::from_ref_time(37_757_000)
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	fn service_task_periodic() -> Weight {
		// Minimum execution time: 14_797 nanoseconds.
		Weight::from_ref_time(15_222_000)
	}
	fn execute_dispatch_signed() -> Weight {
		// Minimum execution time: 6_692 nanoseconds.
		Weight::from_ref_time(6_886_000)
	}
	fn execute_dispatch_unsigned() -> Weight {
		// Minimum execution time: 4_840 nanoseconds.
		Weight::from_ref_time(4_946_000)
	}
	// Storage: Scheduler Agenda (r:1 w:1)
	/// The range of component `s` is `[0, 49]`.
	fn schedule(s: u32, ) -> Weight {
		// Minimum execution time: 18_608 nanoseconds.
		Weight::from_ref_time(23_175_287)
			// Standard Error: 3_173
			.saturating_add(Weight::from_ref_time(846_144).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: Scheduler Agenda (r:1 w:1)
	// Storage: Scheduler Lookup (r:0 w:1)
	/// The range of component `s` is `[1, 50]`.
	fn cancel(s: u32, ) -> Weight {
		// Minimum execution time: 23_332 nanoseconds.
		Weight::from_ref_time(24_383_594)
			// Standard Error: 4_101
			.saturating_add(Weight::from_ref_time(1_412_848).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(2))
	}
	// Storage: Scheduler Lookup (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	/// The range of component `s` is `[0, 49]`.
	fn schedule_named(s: u32, ) -> Weight {
		// Minimum execution time: 21_762 nanoseconds.
		Weight::from_ref_time(26_872_288)
			// Standard Error: 3_676
			.saturating_add(Weight::from_ref_time(883_404).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(2))
	}
	// Storage: Scheduler Lookup (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	/// The range of component `s` is `[1, 50]`.
	fn cancel_named(s: u32, ) -> Weight {
		// Minimum execution time: 23_694 nanoseconds.
		Weight::from_ref_time(26_729_328)
			// Standard Error: 3_595
			.saturating_add(Weight::from_ref_time(1_422_442).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(2))
	}
}
