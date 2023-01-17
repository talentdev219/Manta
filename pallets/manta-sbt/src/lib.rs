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

//! # MantaSBT Module
//!
//! MantaSBT creates non-transferable nfts as unspendable UTXOs
//!
//! ## Overview
//!
//! Uses `pallet-uniques` to store NFT data. NFTs are created by the pallet account and Ownership is recorded as an UTXO

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(doc_cfg, feature(doc_cfg))]
#![forbid(rustdoc::broken_intra_doc_links)]

extern crate alloc;

use crate::types::{
    asset_value_encode, fp_decode, fp_encode, Asset, AssetValue, FullIncomingNote, ReceiverChunk,
    SenderChunk, TransferPost, Utxo, UtxoAccumulatorOutput, UtxoMerkleTreePath,
};
use alloc::{vec, vec::Vec};
use core::marker::PhantomData;
use frame_support::{
    pallet_prelude::*,
    traits::{
        tokens::nonfungibles::Mutate, Currency, ExistenceRequirement, ReservableCurrency,
        StorageVersion,
    },
    transactional, PalletId,
};
use frame_system::pallet_prelude::*;
use manta_pay::{
    config::{self, utxo::MerkleTreeConfiguration},
    manta_accounting::transfer::{
        self,
        canonical::TransferShape,
        receiver::{ReceiverLedger, ReceiverPostError},
        sender::{SenderLedger, SenderPostError},
        InvalidAuthorizationSignature, InvalidSinkAccount, InvalidSourceAccount, ProofSystemError,
        SinkPostingKey, SourcePostingKey, TransferLedger, TransferLedgerSuperPostingKey,
        TransferPostingKeyRef,
    },
    manta_crypto::merkle_tree::{self, forest::Configuration as _},
    manta_parameters::{self, Get as _},
    manta_util::codec::Decode as _,
    parameters::load_transfer_parameters,
};
use manta_util::{
    codec::{self, Encode},
    into_array_unchecked, Array,
};
use sp_runtime::{
    traits::{AccountIdConversion, One, Zero},
    ArithmeticError,
};

pub use crate::types::{Checkpoint, RawCheckpoint};
pub use pallet::*;
pub use types::PullResponse;
pub use weights::WeightInfo;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod test;

pub mod types;
pub mod weights;

#[cfg(feature = "runtime-benchmarks")]
pub mod benchmark;

#[cfg(feature = "rpc")]
pub mod rpc;

#[cfg(feature = "runtime")]
pub mod runtime;

/// SBT Asset Id
pub type SBTAssetId = u128;

/// This is needed because ItemId is generic and doesn't have Add trait implemented
pub trait ItemIdConvert<ItemId> {
    fn asset_id_to_item_id(asset_id: SBTAssetId) -> ItemId;
}

/// Type alias for currency balance.
pub type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

/// MantaSBT Pallet
#[frame_support::pallet]
pub mod pallet {
    use super::*;

    const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

    /// Pallet
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::storage_version(STORAGE_VERSION)]
    pub struct Pallet<T>(_);

    /// The module configuration trait.
    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_uniques::Config {
        /// The overarching event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;

        /// SBT CollectionId
        type PalletCollectionId: Get<Self::CollectionId>;

        /// Returns SBT serial number and increments value
        type ConvertItemId: ItemIdConvert<Self::ItemId>;

        /// The currency mechanism.
        type Currency: ReservableCurrency<Self::AccountId>;

        /// Pallet ID
        type PalletId: Get<PalletId>;

        /// Number of mints reserved per `reserve_sbt` call
        #[pallet::constant]
        type MintsPerReserve: Get<u16>;

        type ReservePrice: Get<BalanceOf<Self>>;
    }

    #[pallet::storage]
    pub type ItemIdCounter<T: Config> = StorageValue<_, SBTAssetId, ValueQuery>;

    #[pallet::storage]
    pub type ReservedIds<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, (SBTAssetId, SBTAssetId), OptionQuery>;

    /// UTXO Set
    #[pallet::storage]
    pub(super) type UtxoSet<T: Config> = StorageMap<_, Twox64Concat, Utxo, (), ValueQuery>;

    /// UTXOs and Incoming Notes Grouped by Shard
    #[pallet::storage]
    pub(super) type Shards<T: Config> = StorageDoubleMap<
        _,
        Twox64Concat,
        u8,
        Twox64Concat,
        u64,
        (Utxo, FullIncomingNote),
        ValueQuery,
    >;

    /// Shard Merkle Tree Paths
    #[pallet::storage]
    pub(super) type ShardTrees<T: Config> =
        StorageMap<_, Twox64Concat, u8, UtxoMerkleTreePath, ValueQuery>;

    /// Outputs of Utxo Accumulator
    #[pallet::storage]
    pub(super) type UtxoAccumulatorOutputs<T: Config> =
        StorageMap<_, Twox64Concat, UtxoAccumulatorOutput, (), ValueQuery>;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Transforms some public assets into private ones using `post`, withdrawing the public
        /// assets from the `origin` account.
        #[pallet::call_index(0)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::to_private())]
        #[transactional]
        pub fn to_private(
            origin: OriginFor<T>,
            post: TransferPost,
            metadata: BoundedVec<u8, <T as pallet_uniques::Config>::StringLimit>,
        ) -> DispatchResultWithPostInfo {
            let origin = ensure_signed(origin)?;

            let (start_id, end_id) =
                ReservedIds::<T>::get(&origin).ok_or(Error::<T>::NotReserved)?;
            // Ensure asset id is correct
            ensure!(
                post.asset_id == Some(Pallet::<T>::field_from_id(start_id)),
                Error::<T>::InvalidAssetId
            );

            pallet_uniques::Pallet::<T>::mint_into(
                &T::PalletCollectionId::get(),
                &T::ConvertItemId::asset_id_to_item_id(start_id),
                &Self::account_id(),
            )?;

            pallet_uniques::Pallet::<T>::set_metadata(
                frame_system::RawOrigin::Root.into(),
                T::PalletCollectionId::get(),
                T::ConvertItemId::asset_id_to_item_id(start_id),
                metadata,
                true,
            )?;

            let increment_start_id = start_id.saturating_add(One::one());
            if increment_start_id == end_id {
                ReservedIds::<T>::remove(&origin)
            } else {
                ReservedIds::<T>::insert(&origin, (increment_start_id, end_id))
            }

            // Only one UTXO allowed to be inserted per transaction
            ensure!(
                post.receiver_posts.len() == 1_usize,
                Error::<T>::DuplicateRegister
            );

            Self::post_transaction(vec![origin], post)
        }

        #[pallet::call_index(1)]
        #[pallet::weight(0)]
        #[transactional]
        pub fn reserve_sbt(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;

            <T as pallet::Config>::Currency::transfer(
                &who,
                &Self::account_id(),
                T::ReservePrice::get(),
                ExistenceRequirement::KeepAlive,
            )?;
            let first_id = ItemIdCounter::<T>::get();
            let stop_id = first_id
                .checked_add(T::MintsPerReserve::get().into())
                .ok_or(ArithmeticError::Overflow)?;
            ItemIdCounter::<T>::set(stop_id);

            ReservedIds::<T>::insert(&who, (first_id, stop_id));
            Self::deposit_event(Event::<T>::SBTReserved {
                who,
                start_id: first_id,
                stop_id,
            });
            Ok(())
        }
    }

    /// Event
    #[pallet::event]
    #[pallet::generate_deposit(fn deposit_event)]
    pub enum Event<T: Config> {
        /// To Private Event
        ToPrivate {
            /// Asset Converted
            asset: Asset,

            /// Source Account
            source: T::AccountId,
        },
        /// Reserve
        SBTReserved {
            /// Public Account reserving sbt mints
            who: T::AccountId,
            /// Start of reserved ids
            start_id: SBTAssetId,
            /// end of reserved ids
            stop_id: SBTAssetId,
        },
    }

    #[pallet::genesis_config]
    pub struct GenesisConfig<T>(PhantomData<T>);

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self(std::marker::PhantomData::<T>)
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        #[inline]
        fn build(&self) {
            pallet_uniques::Pallet::<T>::do_create_collection(
                T::PalletCollectionId::get(),
                Pallet::<T>::account_id(),
                Pallet::<T>::account_id(),
                Zero::zero(),
                true,
                pallet_uniques::Event::<T>::ForceCreated {
                    collection: T::PalletCollectionId::get(),
                    owner: Pallet::<T>::account_id(),
                },
            )
            .expect("create SBT collection on genesis failed");
        }
    }

    /// Error
    #[pallet::error]
    pub enum Error<T> {
        /// Invalid Serialized Form
        ///
        /// The transfer could not be interpreted because of an issue during deserialization.
        InvalidSerializedForm,

        /// Invalid Asset Id
        ///
        /// The asset id of the transfer could not be converted correctly to the standard format.
        InvalidAssetId,

        /// Invalid Shape
        ///
        /// The transfer had an invalid shape.
        InvalidShape,

        /// Invalid Authorization Signature
        InvalidAuthorizationSignature,

        /// Asset Spent
        ///
        /// An asset present in this transfer has already been spent.
        AssetSpent,

        /// Invalid UTXO Accumulator Output
        ///
        /// The sender was constructed on an invalid version of the ledger state.
        InvalidUtxoAccumulatorOutput,

        /// Asset Registered
        ///
        /// An asset present in this transfer has already been registered to the ledger.
        AssetRegistered,

        /// Duplicate Spend
        ///
        /// There were multiple spend entries for the same underlying asset in this transfer.
        DuplicateSpend,

        /// Duplicate Register
        ///
        /// There were multiple register entries for the same underlying asset in this transfer.
        DuplicateRegister,

        /// Invalid Proof
        ///
        /// The submitted proof did not pass validation, or errored during validation.
        InvalidProof,

        /// Invalid Source Account
        ///
        /// At least one of the source accounts is invalid.
        InvalidSourceAccount,

        /// Invalid Sink Account
        ///
        /// At least one of the sink accounts in invalid.
        InvalidSinkAccount,

        /// Internal Ledger Error
        ///
        /// This is caused by some internal error in the ledger and should never occur.
        InternalLedgerError,

        /// Encode Error
        EncodeError,

        /// No Sender Ledger in SBT, Private Transfers are disabled
        NoSenderLedger,

        /// Need to first reserve SBT before minting
        NotReserved,
    }

    impl<T> From<InvalidAuthorizationSignature> for Error<T>
    where
        T: Config,
    {
        #[inline]
        fn from(_: InvalidAuthorizationSignature) -> Self {
            Self::InvalidAuthorizationSignature
        }
    }

    impl<T> From<InvalidSourceAccount<config::Config, T::AccountId>> for Error<T>
    where
        T: Config,
    {
        #[inline]
        fn from(_: InvalidSourceAccount<config::Config, T::AccountId>) -> Self {
            Self::InvalidSourceAccount
        }
    }

    impl<T> From<InvalidSinkAccount<config::Config, T::AccountId>> for Error<T>
    where
        T: Config,
    {
        #[inline]
        fn from(_: InvalidSinkAccount<config::Config, T::AccountId>) -> Self {
            Self::InvalidSinkAccount
        }
    }

    impl<T> From<SenderPostError<SenderLedgerError>> for Error<T> {
        #[inline]
        fn from(err: SenderPostError<SenderLedgerError>) -> Self {
            match err {
                SenderPostError::AssetSpent => Self::NoSenderLedger,
                SenderPostError::InvalidUtxoAccumulatorOutput => Self::NoSenderLedger,
                SenderPostError::UnexpectedError(_) => Self::NoSenderLedger,
            }
        }
    }

    impl<T> From<ReceiverPostError<ReceiverLedgerError>> for Error<T> {
        #[inline]
        fn from(err: ReceiverPostError<ReceiverLedgerError>) -> Self {
            match err {
                ReceiverPostError::AssetRegistered => Self::AssetRegistered,
                ReceiverPostError::UnexpectedError(_) => Self::InternalLedgerError,
            }
        }
    }

    impl<T> From<SenderLedgerError> for TransferLedgerError<T>
    where
        T: Config,
    {
        #[inline]
        fn from(err: SenderLedgerError) -> Self {
            TransferLedgerError::SenderLedgerError(err)
        }
    }

    impl<T> From<ReceiverLedgerError> for TransferLedgerError<T>
    where
        T: Config,
    {
        #[inline]
        fn from(err: ReceiverLedgerError) -> Self {
            TransferLedgerError::ReceiverLedgerError(err)
        }
    }

    /// Transfer Post Error
    pub type TransferPostError<T> = transfer::TransferPostError<
        config::Config,
        <T as frame_system::Config>::AccountId,
        SenderLedgerError,
        ReceiverLedgerError,
        TransferLedgerError<T>,
    >;

    impl<T> From<TransferPostError<T>> for Error<T>
    where
        T: Config,
    {
        #[inline]
        fn from(err: TransferPostError<T>) -> Self {
            match err {
                TransferPostError::<T>::InvalidShape => Self::InvalidShape,
                TransferPostError::<T>::InvalidAuthorizationSignature(err) => err.into(),
                TransferPostError::<T>::InvalidSourceAccount(err) => err.into(),
                TransferPostError::<T>::InvalidSinkAccount(err) => err.into(),
                TransferPostError::<T>::Sender(err) => err.into(),
                TransferPostError::<T>::Receiver(err) => err.into(),
                TransferPostError::<T>::DuplicateMint => Self::DuplicateRegister,
                TransferPostError::<T>::DuplicateSpend => Self::DuplicateSpend,
                TransferPostError::<T>::InvalidProof => Self::InvalidProof,
                TransferPostError::<T>::UnexpectedError(_) => Self::InternalLedgerError,
            }
        }
    }

    impl<T> Pallet<T>
    where
        T: Config,
    {
        /// Maximum Number of Updates per Shard (based on benchmark result)
        const PULL_MAX_RECEIVER_UPDATE_SIZE: u64 = 32768;

        /// Pulls receiver data from the ledger starting at the `receiver_indices`.
        /// The pull algorithm is greedy. It tries to pull as many as possible from each shard
        /// before moving to the next shard.
        #[inline]
        fn pull_receivers(
            receiver_indices: [usize; MerkleTreeConfiguration::FOREST_WIDTH],
            max_update_request: u64,
        ) -> (bool, ReceiverChunk) {
            let mut more_receivers = false;
            let mut receivers = Vec::new();
            let mut receivers_pulled: u64 = 0;
            let max_update = if max_update_request > Self::PULL_MAX_RECEIVER_UPDATE_SIZE {
                Self::PULL_MAX_RECEIVER_UPDATE_SIZE
            } else {
                max_update_request
            };

            for (shard_index, utxo_index) in receiver_indices.into_iter().enumerate() {
                more_receivers |= Self::pull_receivers_for_shard(
                    shard_index as u8,
                    utxo_index,
                    max_update,
                    &mut receivers,
                    &mut receivers_pulled,
                );
                // NOTE: If max capacity is reached and there is more to pull, then we return.
                if receivers_pulled == max_update && more_receivers {
                    break;
                }
            }
            (more_receivers, receivers)
        }

        /// Pulls receiver data from the shard at `shard_index` starting at the `receiver_index`,
        /// pushing the results back to `receivers`.
        #[inline]
        fn pull_receivers_for_shard(
            shard_index: u8,
            receiver_index: usize,
            max_update: u64,
            receivers: &mut ReceiverChunk,
            receivers_pulled: &mut u64,
        ) -> bool {
            let max_receiver_index = (receiver_index as u64) + max_update;
            for idx in (receiver_index as u64)..max_receiver_index {
                if *receivers_pulled == max_update {
                    return Shards::<T>::contains_key(shard_index, idx);
                }
                match Shards::<T>::try_get(shard_index, idx) {
                    Ok(next) => {
                        *receivers_pulled += 1;
                        receivers.push(next);
                    }
                    _ => return false,
                }
            }
            Shards::<T>::contains_key(shard_index, max_receiver_index)
        }

        /// Pulls sender data from the ledger starting at the `sender_index`.
        #[inline]
        fn pull_senders() -> (bool, SenderChunk) {
            (false, vec![])
        }

        /// Returns the diff of ledger state since the given `checkpoint`, `max_receivers`, and
        /// `max_senders`.
        #[inline]
        pub fn pull_ledger_diff(
            checkpoint: Checkpoint,
            max_receivers: u64,
            _max_senders: u64,
        ) -> PullResponse {
            let (more_receivers, receivers) =
                Self::pull_receivers(*checkpoint.receiver_index, max_receivers);
            let (_more_senders, senders) = Self::pull_senders();
            let senders_receivers_total = (0..=255)
                .map(|i| ShardTrees::<T>::get(i).current_path.leaf_index as u128)
                .sum::<u128>();
            PullResponse {
                should_continue: more_receivers,
                receivers,
                senders,
                senders_receivers_total: asset_value_encode(senders_receivers_total),
            }
        }

        /// Returns the account ID of this pallet.
        #[inline]
        pub fn account_id() -> T::AccountId {
            T::PalletId::get().into_account_truncating()
        }

        /// Posts the transaction encoded in `post` to the ledger, using `sources` and `sinks` as
        /// the public deposit and public withdraw accounts respectively.
        #[inline]
        fn post_transaction(
            sources: Vec<T::AccountId>,
            post: TransferPost,
        ) -> DispatchResultWithPostInfo {
            Self::deposit_event(
                config::TransferPost::try_from(post)
                    .map_err(|_| Error::<T>::InvalidSerializedForm)?
                    .post(
                        &load_transfer_parameters(),
                        &mut Ledger(PhantomData),
                        &(),
                        sources,
                        Vec::new(),
                    )
                    .map_err(Error::<T>::from)?
                    .convert(),
            );
            Ok(().into())
        }

        ///
        #[inline]
        pub fn id_from_field(id: [u8; 32]) -> Option<SBTAssetId> {
            if 0u128.to_le_bytes() == id[16..32] {
                Some(u128::from_le_bytes(
                    Array::from_iter(id[0..16].iter().copied()).into(),
                ))
            } else {
                None
            }
        }

        ///
        #[inline]
        pub fn field_from_id(id: SBTAssetId) -> [u8; 32] {
            into_array_unchecked([id.to_le_bytes(), [0; 16]].concat())
        }
    }
}

/// Preprocessed Event
enum PreprocessedEvent<T>
where
    T: Config,
{
    /// To Private Event
    ToPrivate {
        /// Asset Minted
        asset: Asset,

        /// Source Account
        source: T::AccountId,
    },
}

impl<T> PreprocessedEvent<T>
where
    T: Config,
{
    /// Converts a [`PreprocessedEvent`] with into an [`Event`] using the given `origin` for
    /// [`PreprocessedEvent::PrivateTransfer`].
    #[inline]
    fn convert(self) -> Event<T> {
        match self {
            Self::ToPrivate { asset, source } => Event::ToPrivate { asset, source },
        }
    }
}

/// Ledger
struct Ledger<T>(PhantomData<T>)
where
    T: Config;

/// Wrap Type
#[derive(Clone, Copy)]
pub struct Wrap<T>(T);

impl<T> AsRef<T> for Wrap<T> {
    #[inline]
    fn as_ref(&self) -> &T {
        &self.0
    }
}

/// Wrap Pair Type
#[derive(Clone, Copy)]
pub struct WrapPair<L, R>(L, R);

impl<L, R> AsRef<R> for WrapPair<L, R> {
    #[inline]
    fn as_ref(&self) -> &R {
        &self.1
    }
}

/// Sender Ledger Error
pub enum SenderLedgerError {
    /// Sender Ledger is purposly not implementented for SBT ledger
    NoSenderLedger,
}

impl From<SenderLedgerError> for SenderPostError<SenderLedgerError> {
    #[inline]
    fn from(value: SenderLedgerError) -> Self {
        match value {
            SenderLedgerError::NoSenderLedger => {
                Self::UnexpectedError(SenderLedgerError::NoSenderLedger)
            }
        }
    }
}

impl<T> SenderLedger<config::Parameters> for Ledger<T>
where
    T: Config,
{
    type SuperPostingKey = (Wrap<()>, ());
    type ValidUtxoAccumulatorOutput = Wrap<config::UtxoAccumulatorOutput>;
    type ValidNullifier = Wrap<config::Nullifier>;
    type Error = SenderLedgerError;

    #[inline]
    fn is_unspent(
        &self,
        _nullifier: config::Nullifier,
    ) -> Result<Self::ValidNullifier, Self::Error> {
        Err(SenderLedgerError::NoSenderLedger)
    }

    #[inline]
    fn has_matching_utxo_accumulator_output(
        &self,
        _output: config::UtxoAccumulatorOutput,
    ) -> Result<Self::ValidUtxoAccumulatorOutput, Self::Error> {
        Err(SenderLedgerError::NoSenderLedger)
    }

    #[inline]
    fn spend_all<I>(
        &mut self,
        _super_key: &Self::SuperPostingKey,
        _iter: I,
    ) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = (Self::ValidUtxoAccumulatorOutput, Self::ValidNullifier)>,
    {
        Err(SenderLedgerError::NoSenderLedger)
    }
}

/// Merkle Tree Parameters Decode Error Type
pub type MTParametersError = codec::DecodeError<
    <&'static [u8] as codec::Read>::Error,
    <config::UtxoAccumulatorModel as codec::Decode>::Error,
>;

/// Utxo Accumulator Item Hash Decode Error Type
pub type UtxoItemHashError = codec::DecodeError<
    <&'static [u8] as codec::Read>::Error,
    <config::utxo::UtxoAccumulatorItemHash as codec::Decode>::Error,
>;

/// Receiver Ledger Error
pub enum ReceiverLedgerError {
    /// Utxo Decoding Error
    UtxoDecodeError(scale_codec::Error),

    /// Wrong Checksum Error
    ChecksumError,

    /// Merkle Tree Parameters Decoding Error
    MTParametersDecodeError(MTParametersError),

    /// Utxo Accumulator Item Hash Decoding Error
    UtxoAccumulatorItemHashDecodeError(UtxoItemHashError),

    /// Merkle Tree Out of Capacity Error
    MerkleTreeCapacityError,

    /// Field Element Encoding Error
    FpEncodeError(scale_codec::Error),

    /// Field Element Encoding Error
    FpDecodeError(scale_codec::Error),

    /// Path Decoding Error
    PathDecodeError(scale_codec::Error),

    /// Full Incoming Note Decoding Error
    FullNoteDecodeError(scale_codec::Error),

    /// Asset Registered Error
    ///
    /// The asset has already been registered with the ledger.
    AssetRegistered,
}

impl From<ReceiverLedgerError> for ReceiverPostError<ReceiverLedgerError> {
    #[inline]
    fn from(value: ReceiverLedgerError) -> Self {
        if let ReceiverLedgerError::AssetRegistered = value {
            Self::AssetRegistered
        } else {
            Self::UnexpectedError(value)
        }
    }
}

impl<T> ReceiverLedger<config::Parameters> for Ledger<T>
where
    T: Config,
{
    type SuperPostingKey = (Wrap<()>, ());
    type ValidUtxo = Wrap<config::Utxo>;
    type Error = ReceiverLedgerError;

    #[inline]
    fn is_not_registered(&self, utxo: config::Utxo) -> Result<Self::ValidUtxo, Self::Error> {
        if UtxoSet::<T>::contains_key(
            Utxo::try_from(utxo).map_err(ReceiverLedgerError::UtxoDecodeError)?,
        ) {
            Err(ReceiverLedgerError::AssetRegistered)
        } else {
            Ok(Wrap(utxo))
        }
    }

    #[inline]
    fn register_all<I>(
        &mut self,
        super_key: &Self::SuperPostingKey,
        iter: I,
    ) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = (Self::ValidUtxo, config::Note)>,
    {
        let _ = super_key;
        let utxo_accumulator_model = config::UtxoAccumulatorModel::decode(
            manta_parameters::pay::parameters::UtxoAccumulatorModel::get()
                .ok_or(ReceiverLedgerError::ChecksumError)?,
        )
        .map_err(ReceiverLedgerError::MTParametersDecodeError)?;
        let utxo_accumulator_item_hash = config::utxo::UtxoAccumulatorItemHash::decode(
            manta_parameters::pay::parameters::UtxoAccumulatorItemHash::get()
                .ok_or(ReceiverLedgerError::ChecksumError)?,
        )
        .map_err(ReceiverLedgerError::UtxoAccumulatorItemHashDecodeError)?;
        let mut shard_indices = iter
            .into_iter()
            .map(|(utxo, note)| {
                (
                    MerkleTreeConfiguration::tree_index(
                        &utxo.0.item_hash(&utxo_accumulator_item_hash, &mut ()),
                    ),
                    utxo.0,
                    note,
                )
            })
            .collect::<Vec<_>>();
        shard_indices.sort_by_key(|(s, _, _)| *s);
        let mut shard_insertions = Vec::<(_, Vec<_>)>::new();
        for (shard_index, utxo, note) in shard_indices {
            match shard_insertions.last_mut() {
                Some((index, pairs)) if shard_index == *index => pairs.push((utxo, note)),
                _ => shard_insertions.push((shard_index, vec![(utxo, note)])),
            }
        }
        for (shard_index, insertions) in shard_insertions {
            let mut tree = ShardTrees::<T>::get(shard_index);
            let cloned_tree = tree.clone();
            let mut next_root = Option::<config::UtxoAccumulatorOutput>::None;
            let mut current_path = cloned_tree
                .current_path
                .try_into()
                .map_err(ReceiverLedgerError::PathDecodeError)?;
            let mut leaf_digest = tree
                .leaf_digest
                .map(|x| fp_decode(x.to_vec()).map_err(ReceiverLedgerError::FpDecodeError))
                .map_or(Ok(None), |r| r.map(Some))?;
            for (utxo, note) in insertions {
                next_root = Some(
                    merkle_tree::single_path::raw::insert(
                        &utxo_accumulator_model,
                        &mut leaf_digest,
                        &mut current_path,
                        utxo.item_hash(&utxo_accumulator_item_hash, &mut ()),
                    )
                    .ok_or(ReceiverLedgerError::MerkleTreeCapacityError)?,
                );
                let next_index = current_path.leaf_index().0 as u64;
                let utxo = Utxo::try_from(utxo).map_err(ReceiverLedgerError::UtxoDecodeError)?;
                UtxoSet::<T>::insert(utxo, ());
                Shards::<T>::insert(
                    shard_index,
                    next_index,
                    (
                        utxo,
                        FullIncomingNote::try_from(note)
                            .map_err(ReceiverLedgerError::FullNoteDecodeError)?,
                    ),
                );
            }
            tree.current_path = current_path
                .try_into()
                .map_err(ReceiverLedgerError::PathDecodeError)?;
            tree.leaf_digest = leaf_digest
                .map(|x| fp_encode(x).map_err(ReceiverLedgerError::FpEncodeError))
                .map_or(Ok(None), |r| r.map(Some))?;
            if let Some(next_root) = next_root {
                ShardTrees::<T>::insert(shard_index, tree);
                UtxoAccumulatorOutputs::<T>::insert(
                    fp_encode(next_root).map_err(ReceiverLedgerError::FpEncodeError)?,
                    (),
                );
            }
        }
        Ok(())
    }
}

/// Verification Context Decode Error Type
pub type VerifyingContextError = codec::DecodeError<
    <&'static [u8] as codec::Read>::Error,
    <config::VerifyingContext as codec::Decode>::Error,
>;

/// Transfer Ledger Error
pub enum TransferLedgerError<T>
where
    T: Config,
{
    /// Wrong Checksum Error
    ChecksumError,

    /// Verifying Context Decoding Error
    VerifiyingContextDecodeError(VerifyingContextError),

    /// Field Element Encoding Error
    FpEncodeError(scale_codec::Error),

    /// Unknown Asset Error
    UnknownAsset,

    /// Sender Ledger Error
    SenderLedgerError(SenderLedgerError),

    /// Receiver Ledger Error
    ReceiverLedgerError(ReceiverLedgerError),

    /// Invalid Transfer Shape
    InvalidTransferShape,

    /// Proof System Error
    ProofSystemError(ProofSystemError<config::Config>),

    /// Invalid Transfer Proof Error
    ///
    /// Validity of the transfer could not be proved by the ledger.
    InvalidProof,

    /// Type Marker Parameter
    Marker(PhantomData<T>),
}

impl<T> From<TransferLedgerError<T>> for TransferPostError<T>
where
    T: Config,
{
    #[inline]
    fn from(value: TransferLedgerError<T>) -> Self {
        match value {
            TransferLedgerError::InvalidProof => Self::InvalidProof,
            TransferLedgerError::InvalidTransferShape => Self::InvalidShape,
            TransferLedgerError::SenderLedgerError(err) => Self::Sender(err.into()),
            TransferLedgerError::ReceiverLedgerError(err) => Self::Receiver(err.into()),
            TransferLedgerError::UnknownAsset => Self::InvalidProof,
            err => Self::UnexpectedError(err),
        }
    }
}

impl<T> TransferLedger<config::Config> for Ledger<T>
where
    T: Config,
{
    type SuperPostingKey = ();
    type AccountId = T::AccountId;
    type Event = PreprocessedEvent<T>;
    type ValidSourceAccount = WrapPair<Self::AccountId, AssetValue>;
    type ValidSinkAccount = WrapPair<Self::AccountId, AssetValue>;
    type ValidProof = Wrap<()>;
    type Error = TransferLedgerError<T>;

    #[inline]
    fn check_source_accounts<I>(
        &self,
        _asset_id: &config::AssetId,
        sources: I,
    ) -> Result<Vec<Self::ValidSourceAccount>, InvalidSourceAccount<config::Config, Self::AccountId>>
    where
        I: Iterator<Item = (Self::AccountId, config::AssetValue)>,
    {
        Ok(sources
            .map(move |(account_id, withdraw)| WrapPair(account_id, withdraw))
            .collect())
    }

    #[inline]
    fn check_sink_accounts<I>(
        &self,
        _asset_id: &config::AssetId,
        _sinks: I,
    ) -> Result<Vec<Self::ValidSinkAccount>, InvalidSinkAccount<config::Config, Self::AccountId>>
    where
        I: Iterator<Item = (Self::AccountId, config::AssetValue)>,
    {
        // No Sinks in this pallet
        Ok(Vec::new())
    }

    #[inline]
    fn is_valid(
        &self,
        posting_key: TransferPostingKeyRef<config::Config, Self>,
    ) -> Result<(Self::ValidProof, Self::Event), TransferLedgerError<T>> {
        let transfer_shape = TransferShape::from_posting_key_ref(&posting_key);
        let (mut verifying_context, event) =
            match transfer_shape.ok_or(TransferLedgerError::InvalidTransferShape)? {
                TransferShape::ToPrivate => {
                    if let Some(asset_id) = posting_key.asset_id.or(None) {
                        let asset_id =
                            fp_encode(asset_id).map_err(TransferLedgerError::FpEncodeError)?;
                        (
                            manta_parameters::pay::verifying::ToPrivate::get()
                                .ok_or(TransferLedgerError::ChecksumError)?,
                            PreprocessedEvent::<T>::ToPrivate {
                                asset: Asset::new(
                                    asset_id,
                                    asset_value_encode(posting_key.sources[0].1),
                                ),
                                source: posting_key.sources[0].0.clone(),
                            },
                        )
                    } else {
                        return Err(TransferLedgerError::UnknownAsset);
                    }
                }
                TransferShape::PrivateTransfer => {
                    return Err(TransferLedgerError::SenderLedgerError(
                        SenderLedgerError::NoSenderLedger,
                    ))
                }
                TransferShape::ToPublic => {
                    return Err(TransferLedgerError::SenderLedgerError(
                        SenderLedgerError::NoSenderLedger,
                    ))
                }
            };
        let verification = posting_key
            .has_valid_proof(
                &config::VerifyingContext::decode(&mut verifying_context)
                    .map_err(TransferLedgerError::VerifiyingContextDecodeError)?,
            )
            .map_err(TransferLedgerError::ProofSystemError)?;
        if verification {
            Ok((Wrap(()), event))
        } else {
            Err(TransferLedgerError::InvalidProof)
        }
    }

    #[inline]
    fn update_public_balances(
        &mut self,
        _super_key: &TransferLedgerSuperPostingKey<config::Config, Self>,
        _asset_id: config::AssetId,
        _sources: Vec<SourcePostingKey<config::Config, Self>>,
        _sinks: Vec<SinkPostingKey<config::Config, Self>>,
        _proof: Self::ValidProof,
    ) -> Result<(), TransferLedgerError<T>> {
        Ok(())
    }
}
