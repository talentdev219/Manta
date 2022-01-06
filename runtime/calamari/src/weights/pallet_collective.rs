// Copyright 2020-2022 Manta Network.
// This file is part of Manta.

// Manta is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Manta is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Manta.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for pallet_collective
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
<<<<<<< HEAD
//! DATE: 2022-01-05, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
=======
//! DATE: 2021-12-27, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
>>>>>>> manta
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("calamari-dev"), DB CACHE: 128

// Executed Command:
// manta
// benchmark
// --chain=calamari-dev
// --pallet=pallet_collective
// --extrinsic=*
// --execution=Wasm
// --wasm-execution=Compiled
// --heap-pages=4096
// --repeat=20
// --steps=50
// --template=.github/resources/frame-weight-template.hbs
// --output=pallet_collective.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_collective.
pub trait WeightInfo {
	fn set_members(m: u32, n: u32, p: u32, ) -> Weight;
	fn execute(b: u32, m: u32, ) -> Weight;
	fn propose_execute(b: u32, m: u32, ) -> Weight;
	fn propose_proposed(b: u32, m: u32, p: u32, ) -> Weight;
	fn vote(m: u32, ) -> Weight;
	fn close_early_disapproved(m: u32, p: u32, ) -> Weight;
	fn close_early_approved(b: u32, m: u32, p: u32, ) -> Weight;
	fn close_disapproved(m: u32, p: u32, ) -> Weight;
	fn close_approved(b: u32, m: u32, p: u32, ) -> Weight;
	fn disapprove_proposal(p: u32, ) -> Weight;
}

/// Weights for pallet_collective using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_collective::WeightInfo for SubstrateWeight<T> {
	// Storage: Council Members (r:1 w:1)
	// Storage: Council Proposals (r:1 w:0)
	// Storage: Council Voting (r:100 w:100)
	// Storage: Council Prime (r:0 w:1)
	fn set_members(m: u32, n: u32, p: u32, ) -> Weight {
		(0 as Weight)
<<<<<<< HEAD
			// Standard Error: 10_000
			.saturating_add((20_710_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 10_000
			.saturating_add((136_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 10_000
			.saturating_add((27_054_000 as Weight).saturating_mul(p as Weight))
=======
			// Standard Error: 9_000
			.saturating_add((26_671_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 9_000
			.saturating_add((183_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 9_000
			.saturating_add((33_265_000 as Weight).saturating_mul(p as Weight))
>>>>>>> manta
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(p as Weight)))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(p as Weight)))
	}
	// Storage: Council Members (r:1 w:0)
	fn execute(b: u32, m: u32, ) -> Weight {
<<<<<<< HEAD
		(24_675_000 as Weight)
			// Standard Error: 0
			.saturating_add((3_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 0
			.saturating_add((107_000 as Weight).saturating_mul(m as Weight))
=======
		(26_335_000 as Weight)
			// Standard Error: 0
			.saturating_add((3_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 0
			.saturating_add((158_000 as Weight).saturating_mul(m as Weight))
>>>>>>> manta
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
	// Storage: Council Members (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:0)
	fn propose_execute(b: u32, m: u32, ) -> Weight {
<<<<<<< HEAD
		(30_263_000 as Weight)
			// Standard Error: 0
			.saturating_add((3_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 0
			.saturating_add((201_000 as Weight).saturating_mul(m as Weight))
=======
		(31_748_000 as Weight)
			// Standard Error: 0
			.saturating_add((4_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 1_000
			.saturating_add((304_000 as Weight).saturating_mul(m as Weight))
>>>>>>> manta
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
	}
	// Storage: Council Members (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:1)
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council ProposalCount (r:1 w:1)
	// Storage: Council Voting (r:0 w:1)
	fn propose_proposed(b: u32, m: u32, p: u32, ) -> Weight {
<<<<<<< HEAD
		(41_207_000 as Weight)
			// Standard Error: 0
			.saturating_add((12_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 2_000
			.saturating_add((123_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 2_000
			.saturating_add((488_000 as Weight).saturating_mul(p as Weight))
=======
		(44_163_000 as Weight)
			// Standard Error: 0
			.saturating_add((10_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 2_000
			.saturating_add((188_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 2_000
			.saturating_add((501_000 as Weight).saturating_mul(p as Weight))
>>>>>>> manta
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Voting (r:1 w:1)
	fn vote(m: u32, ) -> Weight {
<<<<<<< HEAD
		(48_800_000 as Weight)
			// Standard Error: 3_000
			.saturating_add((218_000 as Weight).saturating_mul(m as Weight))
=======
		(48_205_000 as Weight)
			// Standard Error: 4_000
			.saturating_add((319_000 as Weight).saturating_mul(m as Weight))
>>>>>>> manta
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council ProposalOf (r:0 w:1)
	fn close_early_disapproved(m: u32, p: u32, ) -> Weight {
<<<<<<< HEAD
		(52_143_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((209_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 2_000
			.saturating_add((412_000 as Weight).saturating_mul(p as Weight))
=======
		(52_309_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((326_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 2_000
			.saturating_add((444_000 as Weight).saturating_mul(p as Weight))
>>>>>>> manta
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:1)
	// Storage: Council Proposals (r:1 w:1)
	fn close_early_approved(b: u32, m: u32, p: u32, ) -> Weight {
<<<<<<< HEAD
		(62_558_000 as Weight)
			// Standard Error: 0
			.saturating_add((9_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 2_000
			.saturating_add((218_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 2_000
			.saturating_add((484_000 as Weight).saturating_mul(p as Weight))
=======
		(62_639_000 as Weight)
			// Standard Error: 0
			.saturating_add((9_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 2_000
			.saturating_add((356_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 2_000
			.saturating_add((513_000 as Weight).saturating_mul(p as Weight))
>>>>>>> manta
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Prime (r:1 w:0)
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council ProposalOf (r:0 w:1)
	fn close_disapproved(m: u32, p: u32, ) -> Weight {
<<<<<<< HEAD
		(58_410_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((205_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 2_000
			.saturating_add((401_000 as Weight).saturating_mul(p as Weight))
=======
		(60_930_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((304_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 2_000
			.saturating_add((430_000 as Weight).saturating_mul(p as Weight))
>>>>>>> manta
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Prime (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:1)
	// Storage: Council Proposals (r:1 w:1)
	fn close_approved(b: u32, m: u32, p: u32, ) -> Weight {
<<<<<<< HEAD
		(63_512_000 as Weight)
			// Standard Error: 0
			.saturating_add((9_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 2_000
			.saturating_add((253_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 2_000
			.saturating_add((493_000 as Weight).saturating_mul(p as Weight))
=======
		(68_194_000 as Weight)
			// Standard Error: 0
			.saturating_add((9_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 2_000
			.saturating_add((353_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 2_000
			.saturating_add((512_000 as Weight).saturating_mul(p as Weight))
>>>>>>> manta
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council Voting (r:0 w:1)
	// Storage: Council ProposalOf (r:0 w:1)
	fn disapprove_proposal(p: u32, ) -> Weight {
<<<<<<< HEAD
		(30_477_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((490_000 as Weight).saturating_mul(p as Weight))
=======
		(31_711_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((509_000 as Weight).saturating_mul(p as Weight))
>>>>>>> manta
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Council Members (r:1 w:1)
	// Storage: Council Proposals (r:1 w:0)
	// Storage: Council Voting (r:100 w:100)
	// Storage: Council Prime (r:0 w:1)
	fn set_members(m: u32, n: u32, p: u32, ) -> Weight {
		(0 as Weight)
<<<<<<< HEAD
			// Standard Error: 10_000
			.saturating_add((20_710_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 10_000
			.saturating_add((136_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 10_000
			.saturating_add((27_054_000 as Weight).saturating_mul(p as Weight))
=======
			// Standard Error: 9_000
			.saturating_add((26_671_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 9_000
			.saturating_add((183_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 9_000
			.saturating_add((33_265_000 as Weight).saturating_mul(p as Weight))
>>>>>>> manta
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(p as Weight)))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(p as Weight)))
	}
	// Storage: Council Members (r:1 w:0)
	fn execute(b: u32, m: u32, ) -> Weight {
<<<<<<< HEAD
		(24_675_000 as Weight)
			// Standard Error: 0
			.saturating_add((3_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 0
			.saturating_add((107_000 as Weight).saturating_mul(m as Weight))
=======
		(26_335_000 as Weight)
			// Standard Error: 0
			.saturating_add((3_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 0
			.saturating_add((158_000 as Weight).saturating_mul(m as Weight))
>>>>>>> manta
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
	}
	// Storage: Council Members (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:0)
	fn propose_execute(b: u32, m: u32, ) -> Weight {
<<<<<<< HEAD
		(30_263_000 as Weight)
			// Standard Error: 0
			.saturating_add((3_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 0
			.saturating_add((201_000 as Weight).saturating_mul(m as Weight))
=======
		(31_748_000 as Weight)
			// Standard Error: 0
			.saturating_add((4_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 1_000
			.saturating_add((304_000 as Weight).saturating_mul(m as Weight))
>>>>>>> manta
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
	}
	// Storage: Council Members (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:1)
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council ProposalCount (r:1 w:1)
	// Storage: Council Voting (r:0 w:1)
	fn propose_proposed(b: u32, m: u32, p: u32, ) -> Weight {
<<<<<<< HEAD
		(41_207_000 as Weight)
			// Standard Error: 0
			.saturating_add((12_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 2_000
			.saturating_add((123_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 2_000
			.saturating_add((488_000 as Weight).saturating_mul(p as Weight))
=======
		(44_163_000 as Weight)
			// Standard Error: 0
			.saturating_add((10_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 2_000
			.saturating_add((188_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 2_000
			.saturating_add((501_000 as Weight).saturating_mul(p as Weight))
>>>>>>> manta
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Voting (r:1 w:1)
	fn vote(m: u32, ) -> Weight {
<<<<<<< HEAD
		(48_800_000 as Weight)
			// Standard Error: 3_000
			.saturating_add((218_000 as Weight).saturating_mul(m as Weight))
=======
		(48_205_000 as Weight)
			// Standard Error: 4_000
			.saturating_add((319_000 as Weight).saturating_mul(m as Weight))
>>>>>>> manta
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council ProposalOf (r:0 w:1)
	fn close_early_disapproved(m: u32, p: u32, ) -> Weight {
<<<<<<< HEAD
		(52_143_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((209_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 2_000
			.saturating_add((412_000 as Weight).saturating_mul(p as Weight))
=======
		(52_309_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((326_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 2_000
			.saturating_add((444_000 as Weight).saturating_mul(p as Weight))
>>>>>>> manta
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:1)
	// Storage: Council Proposals (r:1 w:1)
	fn close_early_approved(b: u32, m: u32, p: u32, ) -> Weight {
<<<<<<< HEAD
		(62_558_000 as Weight)
			// Standard Error: 0
			.saturating_add((9_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 2_000
			.saturating_add((218_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 2_000
			.saturating_add((484_000 as Weight).saturating_mul(p as Weight))
=======
		(62_639_000 as Weight)
			// Standard Error: 0
			.saturating_add((9_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 2_000
			.saturating_add((356_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 2_000
			.saturating_add((513_000 as Weight).saturating_mul(p as Weight))
>>>>>>> manta
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Prime (r:1 w:0)
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council ProposalOf (r:0 w:1)
	fn close_disapproved(m: u32, p: u32, ) -> Weight {
<<<<<<< HEAD
		(58_410_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((205_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 2_000
			.saturating_add((401_000 as Weight).saturating_mul(p as Weight))
=======
		(60_930_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((304_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 2_000
			.saturating_add((430_000 as Weight).saturating_mul(p as Weight))
>>>>>>> manta
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Prime (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:1)
	// Storage: Council Proposals (r:1 w:1)
	fn close_approved(b: u32, m: u32, p: u32, ) -> Weight {
<<<<<<< HEAD
		(63_512_000 as Weight)
			// Standard Error: 0
			.saturating_add((9_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 2_000
			.saturating_add((253_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 2_000
			.saturating_add((493_000 as Weight).saturating_mul(p as Weight))
=======
		(68_194_000 as Weight)
			// Standard Error: 0
			.saturating_add((9_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 2_000
			.saturating_add((353_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 2_000
			.saturating_add((512_000 as Weight).saturating_mul(p as Weight))
>>>>>>> manta
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council Voting (r:0 w:1)
	// Storage: Council ProposalOf (r:0 w:1)
	fn disapprove_proposal(p: u32, ) -> Weight {
<<<<<<< HEAD
		(30_477_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((490_000 as Weight).saturating_mul(p as Weight))
=======
		(31_711_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((509_000 as Weight).saturating_mul(p as Weight))
>>>>>>> manta
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
}
