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

//! Autogenerated weights for pallet_utility
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-10-09, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("calamari-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/manta
// benchmark
// pallet
// --chain=calamari-dev
// --steps=50
// --repeat=20
// --pallet=pallet_utility
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./scripts/benchmarking/frame-weights-output/pallet_utility.rs
// --template=.github/resources/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_utility.
pub trait WeightInfo {
    fn batch(c: u32, ) -> Weight;
    fn as_derivative() -> Weight;
    fn batch_all(c: u32, ) -> Weight;
    fn dispatch_as() -> Weight;
    fn force_batch(c: u32, ) -> Weight;
}

/// Weights for pallet_utility using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_utility::WeightInfo for SubstrateWeight<T> {
    fn batch(c: u32, ) -> Weight {
        (32_826_000 as Weight)
            // Standard Error: 5_000
            .saturating_add((4_779_000 as Weight).saturating_mul(c as Weight))
    }
    fn as_derivative() -> Weight {
        (5_956_000 as Weight)
    }
    fn batch_all(c: u32, ) -> Weight {
        (47_458_000 as Weight)
            // Standard Error: 6_000
            .saturating_add((4_939_000 as Weight).saturating_mul(c as Weight))
    }
    fn dispatch_as() -> Weight {
        (13_727_000 as Weight)
    }
    fn force_batch(c: u32, ) -> Weight {
        (39_342_000 as Weight)
            // Standard Error: 6_000
            .saturating_add((4_743_000 as Weight).saturating_mul(c as Weight))
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
    fn batch(c: u32, ) -> Weight {
        (32_826_000 as Weight)
            // Standard Error: 5_000
            .saturating_add((4_779_000 as Weight).saturating_mul(c as Weight))
    }
    fn as_derivative() -> Weight {
        (5_956_000 as Weight)
    }
    fn batch_all(c: u32, ) -> Weight {
        (47_458_000 as Weight)
            // Standard Error: 6_000
            .saturating_add((4_939_000 as Weight).saturating_mul(c as Weight))
    }
    fn dispatch_as() -> Weight {
        (13_727_000 as Weight)
    }
    fn force_batch(c: u32, ) -> Weight {
        (39_342_000 as Weight)
            // Standard Error: 6_000
            .saturating_add((4_743_000 as Weight).saturating_mul(c as Weight))
    }
}
