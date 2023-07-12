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

//! Autogenerated weights for pallet_asset_manager
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-10-08, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("calamari-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/manta
// benchmark
// pallet
// --chain=calamari-dev
// --steps=50
// --repeat=20
// --pallet=pallet_asset_manager
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./scripts/benchmarking/frame-weights-output/pallet_asset_manager.rs
// --template=.github/resources/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_asset_manager.
pub trait WeightInfo {
    fn register_asset() -> Weight;
    fn set_units_per_second() -> Weight;
    fn update_asset_location() -> Weight;
    fn update_asset_metadata() -> Weight;
    fn mint_asset() -> Weight;
    fn set_min_xcm_fee() -> Weight;
    fn update_outgoing_filtered_assets() -> Weight;
    fn register_lp_asset() -> Weight;
}

/// Weights for pallet_asset_manager using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    // Storage: AssetManager LocationAssetId (r:1 w:1)
    // Storage: AssetManager NextAssetId (r:1 w:1)
    // Storage: Assets Asset (r:1 w:1)
    // Storage: Assets Metadata (r:1 w:1)
    // Storage: AssetManager AssetIdMetadata (r:0 w:1)
    // Storage: AssetManager AssetIdLocation (r:0 w:1)
    fn register_asset() -> Weight {
        Weight::from_ref_time(43_430_000)
            .saturating_add(T::DbWeight::get().reads(4_u64))
            .saturating_add(T::DbWeight::get().writes(6_u64))
    }
    // Storage: AssetManager AssetIdLocation (r:1 w:0)
    // Storage: AssetManager UnitsPerSecond (r:0 w:1)
    fn set_units_per_second() -> Weight {
        Weight::from_ref_time(56_974_000)
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    // Storage: AssetManager AssetIdLocation (r:1 w:1)
    // Storage: AssetManager LocationAssetId (r:1 w:2)
    // Storage: AssetManager AllowedDestParaIds (r:1 w:1)
    fn update_asset_location() -> Weight {
        Weight::from_ref_time(72_974_000)
            .saturating_add(T::DbWeight::get().reads(3_u64))
            .saturating_add(T::DbWeight::get().writes(4_u64))
    }
    // Storage: AssetManager AssetIdLocation (r:1 w:0)
    // Storage: Assets Asset (r:1 w:0)
    // Storage: Assets Metadata (r:1 w:1)
    // Storage: AssetManager AssetIdMetadata (r:0 w:1)
    fn update_asset_metadata() -> Weight {
        Weight::from_ref_time(73_985_000)
            .saturating_add(T::DbWeight::get().reads(3_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    // Storage: AssetManager AssetIdLocation (r:1 w:0)
    // Storage: Assets Asset (r:1 w:1)
    // Storage: Assets Account (r:1 w:1)
    fn mint_asset() -> Weight {
        Weight::from_ref_time(85_480_000)
            .saturating_add(T::DbWeight::get().reads(3_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    // Storage: AssetManager MinXcmFee (r:0 w:1)
    fn set_min_xcm_fee() -> Weight {
        Weight::from_ref_time(49_509_000)
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    fn update_outgoing_filtered_assets() -> Weight {
        Weight::from_ref_time(49_509_000)
            .saturating_add(RocksDbWeight::get().writes(1_u64))
    }
    // Storage: AssetManager AssetIdLocation (r:2 w:0)
    // Storage: AssetManager AssetIdPairToLp (r:1 w:1)
    // Storage: AssetManager NextAssetId (r:1 w:1)
    // Storage: Assets Asset (r:1 w:1)
    // Storage: Assets Metadata (r:1 w:1)
    // Storage: AssetManager AssetIdMetadata (r:0 w:1)
    // Storage: AssetManager LpToAssetIdPair (r:0 w:1)
    fn register_lp_asset() -> Weight {
        // Minimum execution time: 546_000 nanoseconds.
        Weight::from_ref_time(546_000_000)
            .saturating_add(T::DbWeight::get().reads(6))
            .saturating_add(T::DbWeight::get().writes(6))
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
    // Storage: AssetManager LocationAssetId (r:1 w:1)
    // Storage: AssetManager NextAssetId (r:1 w:1)
    // Storage: Assets Asset (r:1 w:1)
    // Storage: Assets Metadata (r:1 w:1)
    // Storage: AssetManager AssetIdMetadata (r:0 w:1)
    // Storage: AssetManager AssetIdLocation (r:0 w:1)
    fn register_asset() -> Weight {
        Weight::from_ref_time(43_430_000)
            .saturating_add(RocksDbWeight::get().reads(4_u64))
            .saturating_add(RocksDbWeight::get().writes(6_u64))
    }
    // Storage: AssetManager AssetIdLocation (r:1 w:0)
    // Storage: AssetManager UnitsPerSecond (r:0 w:1)
    fn set_units_per_second() -> Weight {
        Weight::from_ref_time(56_974_000)
            .saturating_add(RocksDbWeight::get().reads(1_u64))
            .saturating_add(RocksDbWeight::get().writes(1_u64))
    }
    // Storage: AssetManager AssetIdLocation (r:1 w:1)
    // Storage: AssetManager LocationAssetId (r:1 w:2)
    // Storage: AssetManager AllowedDestParaIds (r:1 w:1)
    fn update_asset_location() -> Weight {
        Weight::from_ref_time(72_974_000)
            .saturating_add(RocksDbWeight::get().reads(3_u64))
            .saturating_add(RocksDbWeight::get().writes(4_u64))
    }
    // Storage: AssetManager AssetIdLocation (r:1 w:0)
    // Storage: Assets Asset (r:1 w:0)
    // Storage: Assets Metadata (r:1 w:1)
    // Storage: AssetManager AssetIdMetadata (r:0 w:1)
    fn update_asset_metadata() -> Weight {
        Weight::from_ref_time(73_985_000)
            .saturating_add(RocksDbWeight::get().reads(3_u64))
            .saturating_add(RocksDbWeight::get().writes(2_u64))
    }
    // Storage: AssetManager AssetIdLocation (r:1 w:0)
    // Storage: Assets Asset (r:1 w:1)
    // Storage: Assets Account (r:1 w:1)
    fn mint_asset() -> Weight {
        Weight::from_ref_time(85_480_000)
            .saturating_add(RocksDbWeight::get().reads(3_u64))
            .saturating_add(RocksDbWeight::get().writes(2_u64))
    }
    // Storage: AssetManager MinXcmFee (r:0 w:1)
    fn set_min_xcm_fee() -> Weight {
        Weight::from_ref_time(49_509_000)
            .saturating_add(RocksDbWeight::get().writes(1_u64))
    }
    fn update_outgoing_filtered_assets() -> Weight {
        Weight::from_ref_time(49_509_000)
            .saturating_add(RocksDbWeight::get().writes(1_u64))
    }
    // Storage: AssetManager AssetIdLocation (r:2 w:0)
    // Storage: AssetManager AssetIdPairToLp (r:1 w:1)
    // Storage: AssetManager NextAssetId (r:1 w:1)
    // Storage: Assets Asset (r:1 w:1)
    // Storage: Assets Metadata (r:1 w:1)
    // Storage: AssetManager AssetIdMetadata (r:0 w:1)
    // Storage: AssetManager LpToAssetIdPair (r:0 w:1)
    fn register_lp_asset() -> Weight {
        // Minimum execution time: 546_000 nanoseconds.
        Weight::from_ref_time(546_000_000)
            .saturating_add(RocksDbWeight::get().reads(6))
            .saturating_add(RocksDbWeight::get().writes(6))
    }
}
