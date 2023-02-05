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

use super::{
    weights, xcm_config::SelfReserve, AssetManager, Assets, Balances, Event,
    NativeTokenExistentialDeposit, Origin, Runtime,
};

use manta_primitives::{
    assets::{
        AssetConfig, AssetIdType, AssetLocation, AssetMetadata, AssetRegistry, BalanceType,
        FungibleAssetRegistryMetadata, FungibleAssetStorageMetadata, LocationType,
        NativeAndNonNative,
    },
    constants::{ASSET_MANAGER_PALLET_ID, DOLPHIN_DECIMAL, MANTA_PAY_PALLET_ID},
    types::{AccountId, Balance, DolphinAssetId},
};

use frame_support::{pallet_prelude::*, parameter_types, PalletId};
use frame_system::EnsureRoot;
use xcm::VersionedMultiLocation;

parameter_types! {
    pub const AssetDeposit: Balance = 0; // Does not really matter as this will be only called by root
    pub const AssetAccountDeposit: Balance = 0;
    pub const ApprovalDeposit: Balance = 0;
    pub const AssetsStringLimit: u32 = 50;
    pub const MetadataDepositBase: Balance = 0;
    pub const MetadataDepositPerByte: Balance = 0;
}

impl pallet_assets::Config for Runtime {
    type Event = Event;
    type Balance = Balance;
    type AssetId = DolphinAssetId;
    type Currency = Balances;
    type ForceOrigin = EnsureRoot<AccountId>;
    type AssetDeposit = AssetDeposit;
    type AssetAccountDeposit = AssetAccountDeposit;
    type MetadataDepositBase = MetadataDepositBase;
    type MetadataDepositPerByte = MetadataDepositPerByte;
    type ApprovalDeposit = ApprovalDeposit;
    type StringLimit = AssetsStringLimit;
    type Freezer = ();
    type Extra = ();
    type WeightInfo = weights::pallet_assets::SubstrateWeight<Runtime>;
}

pub struct MantaAssetRegistry;
impl BalanceType for MantaAssetRegistry {
    type Balance = Balance;
}
impl AssetIdType for MantaAssetRegistry {
    type AssetId = DolphinAssetId;
}
impl AssetRegistry for MantaAssetRegistry {
    type Metadata = AssetMetadata<Balance>;
    type Error = sp_runtime::DispatchError;

    fn create_asset(asset_id: DolphinAssetId, metadata: AssetMetadata<Balance>) -> DispatchResult {
        match metadata {
            AssetMetadata::FT(meta) => {
                Assets::force_create(
                    Origin::root(),
                    asset_id,
                    sp_runtime::MultiAddress::Id(AssetManager::account_id()),
                    meta.is_sufficient,
                    meta.min_balance,
                )?;

                Assets::force_set_metadata(
                    Origin::root(),
                    asset_id,
                    meta.metadata.name,
                    meta.metadata.symbol,
                    meta.metadata.decimals,
                    meta.metadata.is_frozen,
                )?;

                Assets::force_asset_status(
                    Origin::root(),
                    asset_id,
                    AssetManager::account_id().into(),
                    AssetManager::account_id().into(),
                    AssetManager::account_id().into(),
                    AssetManager::account_id().into(),
                    meta.min_balance,
                    meta.is_sufficient,
                    meta.metadata.is_frozen,
                )
            }
            AssetMetadata::SBT(_) => Ok(()),
        }
    }

    fn update_asset_metadata(
        asset_id: &DolphinAssetId,
        metadata: AssetMetadata<Balance>,
    ) -> DispatchResult {
        match metadata {
            AssetMetadata::FT(meta) => Assets::force_set_metadata(
                Origin::root(),
                *asset_id,
                meta.metadata.name,
                meta.metadata.symbol,
                meta.metadata.decimals,
                meta.metadata.is_frozen,
            ),
            AssetMetadata::SBT(_) => Ok(()),
        }
    }
}

parameter_types! {
    pub const StartNonNativeAssetId: DolphinAssetId = 8;
    pub const NativeAssetId: DolphinAssetId = 1;
    pub NativeAssetLocation: AssetLocation = AssetLocation(
        VersionedMultiLocation::V1(SelfReserve::get()));
    pub NativeAssetMetadata: FungibleAssetRegistryMetadata<Balance> = FungibleAssetRegistryMetadata {
        metadata: FungibleAssetStorageMetadata {
            name: b"Dolphin".to_vec(),
            symbol: b"DOL".to_vec(),
            decimals: DOLPHIN_DECIMAL,
            is_frozen: false,
        },
        min_balance: NativeTokenExistentialDeposit::get(),
        is_sufficient: true,
    };
    pub const AssetManagerPalletId: PalletId = ASSET_MANAGER_PALLET_ID;
}

pub type DolphinConcreteFungibleLedger =
    NativeAndNonNative<Runtime, DolphinAssetConfig, Balances, Assets>;

/// AssetConfig implementations for this runtime
#[derive(Clone, Eq, PartialEq)]
pub struct DolphinAssetConfig;
impl LocationType for DolphinAssetConfig {
    type Location = AssetLocation;
}
impl BalanceType for DolphinAssetConfig {
    type Balance = Balance;
}
impl AssetIdType for DolphinAssetConfig {
    type AssetId = DolphinAssetId;
}
impl AssetConfig<Runtime> for DolphinAssetConfig {
    type StartNonNativeAssetId = StartNonNativeAssetId;
    type NativeAssetId = NativeAssetId;
    type NativeAssetLocation = NativeAssetLocation;
    type NativeAssetMetadata = NativeAssetMetadata;
    type AssetRegistry = MantaAssetRegistry;
    type FungibleLedger = DolphinConcreteFungibleLedger;
}

impl pallet_asset_manager::Config for Runtime {
    type Event = Event;
    type AssetId = DolphinAssetId;
    type Balance = Balance;
    type Location = AssetLocation;
    type AssetConfig = DolphinAssetConfig;
    type ModifierOrigin = EnsureRoot<AccountId>;
    type PalletId = AssetManagerPalletId;
    type WeightInfo = weights::pallet_asset_manager::SubstrateWeight<Runtime>;
}

parameter_types! {
    pub const MantaPayPalletId: PalletId = MANTA_PAY_PALLET_ID;
}

impl pallet_manta_pay::Config for Runtime {
    type Event = Event;
    type WeightInfo = weights::pallet_manta_pay::SubstrateWeight<Runtime>;
    type AssetConfig = DolphinAssetConfig;
    type PalletId = MantaPayPalletId;
    type Suspender = ();
}
