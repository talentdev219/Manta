use cumulus_client_consensus_aura::{
	build_aura_consensus, BuildAuraConsensusParams, SlotProportion,
};
use cumulus_client_consensus_common::{
	ParachainBlockImport, ParachainCandidate, ParachainConsensus,
};
use cumulus_client_consensus_relay_chain::Verifier as RelayChainVerifier;
use cumulus_client_network::build_block_announce_validator;
use cumulus_client_service::{
	prepare_node_config, start_collator, start_full_node, StartCollatorParams, StartFullNodeParams,
};
use cumulus_primitives_core::{
	relay_chain::v1::{Hash as PHash, PersistedValidationData},
	ParaId,
};
use futures::lock::Mutex;
use sc_client_api::ExecutorProvider;
use sc_consensus::{
	import_queue::{BasicQueue, Verifier as VerifierT},
	BlockImportParams,
};
use sc_executor::native_executor_instance;
use sc_network::NetworkService;
use sc_service::{Configuration, PartialComponents, Role, TFullBackend, TFullClient, TaskManager};
use sc_telemetry::{Telemetry, TelemetryHandle, TelemetryWorker, TelemetryWorkerHandle};
use sp_api::{ApiExt, ConstructRuntimeApi};
use sp_consensus::{CacheKeyId, SlotData};
use sp_consensus_aura::{sr25519::AuthorityId as AuraId, AuraApi};
use sp_keystore::SyncCryptoStorePtr;
use sp_runtime::{
	generic::BlockId,
	traits::{BlakeTwo256, Header as HeaderT},
};
use std::sync::Arc;
use substrate_prometheus_endpoint::Registry;

pub use sc_executor::NativeExecutor;

type BlockNumber = u32;
type Header = sp_runtime::generic::Header<BlockNumber, sp_runtime::traits::BlakeTwo256>;
pub type Block = sp_runtime::generic::Block<Header, sp_runtime::OpaqueExtrinsic>;
type Hash = sp_core::H256;

// Native executor instance.
// Native Manta Parachain executor instance.
#[cfg(feature = "manta-pc")]
native_executor_instance!(
	pub MantaPCRuntimeExecutor,
	manta_pc_runtime::api::dispatch,
	manta_pc_runtime::native_version,
	frame_benchmarking::benchmarking::HostFunctions,
);

// Native Calamari Parachain executor instance.
#[cfg(feature = "calamari")]
native_executor_instance!(
	pub CalamariRuntimeExecutor,
	calamari_runtime::api::dispatch,
	calamari_runtime::native_version,
	frame_benchmarking::benchmarking::HostFunctions,
);

/// Starts a `ServiceBuilder` for a full service.
///
/// Use this macro if you don't actually need the full service, but just the builder in order to
/// be able to perform chain operations.
pub fn new_partial<RuntimeApi, Executor, BIQ>(
	config: &Configuration,
	build_import_queue: BIQ,
) -> Result<
	PartialComponents<
		TFullClient<Block, RuntimeApi, Executor>,
		TFullBackend<Block>,
		(),
		sc_consensus::DefaultImportQueue<Block, TFullClient<Block, RuntimeApi, Executor>>,
		sc_transaction_pool::FullPool<Block, TFullClient<Block, RuntimeApi, Executor>>,
		(Option<Telemetry>, Option<TelemetryWorkerHandle>),
	>,
	sc_service::Error,
>
where
	RuntimeApi: ConstructRuntimeApi<Block, TFullClient<Block, RuntimeApi, Executor>>
		+ Send
		+ Sync
		+ 'static,
	RuntimeApi::RuntimeApi: sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block>
		+ sp_api::Metadata<Block>
		+ sp_session::SessionKeys<Block>
		+ sp_api::ApiExt<
			Block,
			StateBackend = sc_client_api::StateBackendFor<TFullBackend<Block>, Block>,
		> + sp_offchain::OffchainWorkerApi<Block>
		+ sp_block_builder::BlockBuilder<Block>,
	sc_client_api::StateBackendFor<TFullBackend<Block>, Block>: sp_api::StateBackend<BlakeTwo256>,
	Executor: sc_executor::NativeExecutionDispatch + 'static,
	BIQ: FnOnce(
		Arc<TFullClient<Block, RuntimeApi, Executor>>,
		&Configuration,
		Option<TelemetryHandle>,
		&TaskManager,
	) -> Result<
		sc_consensus::DefaultImportQueue<Block, TFullClient<Block, RuntimeApi, Executor>>,
		sc_service::Error,
	>,
{
	let telemetry = config
		.telemetry_endpoints
		.clone()
		.filter(|x| !x.is_empty())
		.map(|endpoints| -> Result<_, sc_telemetry::Error> {
			let worker = TelemetryWorker::new(16)?;
			let telemetry = worker.handle().new_telemetry(endpoints);
			Ok((worker, telemetry))
		})
		.transpose()?;

	let (client, backend, keystore_container, task_manager) =
		sc_service::new_full_parts::<Block, RuntimeApi, Executor>(
			config,
			telemetry.as_ref().map(|(_, telemetry)| telemetry.handle()),
		)?;
	let client = Arc::new(client);

	let telemetry_worker_handle = telemetry.as_ref().map(|(worker, _)| worker.handle());

	let telemetry = telemetry.map(|(worker, telemetry)| {
		task_manager.spawn_handle().spawn("telemetry", worker.run());
		telemetry
	});

	let transaction_pool = sc_transaction_pool::BasicPool::new_full(
		config.transaction_pool.clone(),
		config.role.is_authority().into(),
		config.prometheus_registry(),
		task_manager.spawn_essential_handle(),
		client.clone(),
	);

	let import_queue = build_import_queue(
		client.clone(),
		config,
		telemetry.as_ref().map(|telemetry| telemetry.handle()),
		&task_manager,
	)?;

	let params = PartialComponents {
		backend,
		client,
		import_queue,
		keystore_container,
		task_manager,
		transaction_pool,
		select_chain: (),
		other: (telemetry, telemetry_worker_handle),
	};

	Ok(params)
}

/// Start a node with the given parachain `Configuration` and relay chain `Configuration`.
///
/// This is the actual implementation that is abstract over the executor and the runtime api.
#[sc_tracing::logging::prefix_logs_with("Parachain")]
async fn start_node_impl<RuntimeApi, Executor, RB, BIQ, BIC>(
	parachain_config: Configuration,
	polkadot_config: Configuration,
	id: ParaId,
	_rpc_ext_builder: RB,
	build_import_queue: BIQ,
	build_consensus: BIC,
) -> sc_service::error::Result<(TaskManager, Arc<TFullClient<Block, RuntimeApi, Executor>>)>
where
	RuntimeApi: ConstructRuntimeApi<Block, TFullClient<Block, RuntimeApi, Executor>>
		+ Send
		+ Sync
		+ 'static,
	RuntimeApi::RuntimeApi: sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block>
		+ sp_api::Metadata<Block>
		+ sp_session::SessionKeys<Block>
		+ sp_api::ApiExt<
			Block,
			StateBackend = sc_client_api::StateBackendFor<TFullBackend<Block>, Block>,
		> + sp_offchain::OffchainWorkerApi<Block>
		+ sp_block_builder::BlockBuilder<Block>
		+ cumulus_primitives_core::CollectCollationInfo<Block>
		+ pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<
			Block,
			manta_primitives::Balance,
		>,
	sc_client_api::StateBackendFor<TFullBackend<Block>, Block>: sp_api::StateBackend<BlakeTwo256>,
	Executor: sc_executor::NativeExecutionDispatch + 'static,
	RB: Fn(
			Arc<TFullClient<Block, RuntimeApi, Executor>>,
		) -> Result<jsonrpc_core::IoHandler<sc_rpc::Metadata>, sc_service::Error>
		+ Send
		+ 'static,
	BIQ: FnOnce(
		Arc<TFullClient<Block, RuntimeApi, Executor>>,
		&Configuration,
		Option<TelemetryHandle>,
		&TaskManager,
	) -> Result<
		sc_consensus::DefaultImportQueue<Block, TFullClient<Block, RuntimeApi, Executor>>,
		sc_service::Error,
	>,
	BIC: FnOnce(
		Arc<TFullClient<Block, RuntimeApi, Executor>>,
		Option<&Registry>,
		Option<TelemetryHandle>,
		&TaskManager,
		&polkadot_service::NewFull<polkadot_service::Client>,
		Arc<sc_transaction_pool::FullPool<Block, TFullClient<Block, RuntimeApi, Executor>>>,
		Arc<NetworkService<Block, Hash>>,
		SyncCryptoStorePtr,
		bool,
	) -> Result<Box<dyn ParachainConsensus<Block>>, sc_service::Error>,
{
	if matches!(parachain_config.role, Role::Light) {
		return Err("Light client not supported!".into());
	}

	let parachain_config = prepare_node_config(parachain_config);

	let params = new_partial::<RuntimeApi, Executor, BIQ>(&parachain_config, build_import_queue)?;
	let (mut telemetry, telemetry_worker_handle) = params.other;

	let relay_chain_full_node =
		cumulus_client_service::build_polkadot_full_node(polkadot_config, telemetry_worker_handle)
			.map_err(|e| match e {
				polkadot_service::Error::Sub(x) => x,
				s => format!("{}", s).into(),
			})?;

	let client = params.client.clone();
	let backend = params.backend.clone();
	let block_announce_validator = build_block_announce_validator(
		relay_chain_full_node.client.clone(),
		id,
		Box::new(relay_chain_full_node.network.clone()),
		relay_chain_full_node.backend.clone(),
	);

	let force_authoring = parachain_config.force_authoring;
	let validator = parachain_config.role.is_authority();
	let prometheus_registry = parachain_config.prometheus_registry().cloned();
	let transaction_pool = params.transaction_pool.clone();
	let mut task_manager = params.task_manager;
	let import_queue = cumulus_client_service::SharedImportQueue::new(params.import_queue);
	let (network, system_rpc_tx, start_network) =
		sc_service::build_network(sc_service::BuildNetworkParams {
			config: &parachain_config,
			client: client.clone(),
			transaction_pool: transaction_pool.clone(),
			spawn_handle: task_manager.spawn_handle(),
			import_queue: import_queue.clone(),
			on_demand: None,
			block_announce_validator_builder: Some(Box::new(|_| block_announce_validator)),
			warp_sync: None,
		})?;

	let rpc_client = client.clone();
	let rpc_extensions_builder = Box::new(move |_, _| {
		use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApi};

		let mut io = jsonrpc_core::IoHandler::default();
		io.extend_with(TransactionPaymentApi::to_delegate(TransactionPayment::new(
			rpc_client.clone(),
		)));
		Ok(io)
	});

	sc_service::spawn_tasks(sc_service::SpawnTasksParams {
		on_demand: None,
		remote_blockchain: None,
		rpc_extensions_builder,
		client: client.clone(),
		transaction_pool: transaction_pool.clone(),
		task_manager: &mut task_manager,
		config: parachain_config,
		keystore: params.keystore_container.sync_keystore(),
		backend: backend.clone(),
		network: network.clone(),
		system_rpc_tx,
		telemetry: telemetry.as_mut(),
	})?;

	let announce_block = {
		let network = network.clone();
		Arc::new(move |hash, data| network.announce_block(hash, data))
	};

	if validator {
		let parachain_consensus = build_consensus(
			client.clone(),
			prometheus_registry.as_ref(),
			telemetry.as_ref().map(|t| t.handle()),
			&task_manager,
			&relay_chain_full_node,
			transaction_pool,
			network,
			params.keystore_container.sync_keystore(),
			force_authoring,
		)?;

		let spawner = task_manager.spawn_handle();

		let params = StartCollatorParams {
			para_id: id,
			block_status: client.clone(),
			announce_block,
			client: client.clone(),
			task_manager: &mut task_manager,
			relay_chain_full_node,
			spawner,
			parachain_consensus,
			import_queue,
		};

		start_collator(params).await?;
	} else {
		let params = StartFullNodeParams {
			client: client.clone(),
			announce_block,
			task_manager: &mut task_manager,
			para_id: id,
			relay_chain_full_node,
		};

		start_full_node(params)?;
	}

	start_network.start_network();

	Ok((task_manager, client))
}

enum BuildOnAccess<R> {
	Uninitialized(Option<Box<dyn FnOnce() -> R + Send + Sync>>),
	Initialized(R),
}

impl<R> BuildOnAccess<R> {
	fn get_mut(&mut self) -> &mut R {
		loop {
			match self {
				Self::Uninitialized(f) => {
					*self = Self::Initialized((f.take().unwrap())());
				}
				Self::Initialized(ref mut r) => return r,
			}
		}
	}
}

/// Special [`ParachainConsensus`] implementation that waits for the upgrade from
/// shell to a parachain runtime that implements Aura.
struct WaitForAuraConsensus<Client> {
	client: Arc<Client>,
	aura_consensus: Arc<Mutex<BuildOnAccess<Box<dyn ParachainConsensus<Block>>>>>,
	relay_chain_consensus: Arc<Mutex<Box<dyn ParachainConsensus<Block>>>>,
}

impl<Client> Clone for WaitForAuraConsensus<Client> {
	fn clone(&self) -> Self {
		Self {
			client: self.client.clone(),
			aura_consensus: self.aura_consensus.clone(),
			relay_chain_consensus: self.relay_chain_consensus.clone(),
		}
	}
}

#[async_trait::async_trait]
impl<Client> ParachainConsensus<Block> for WaitForAuraConsensus<Client>
where
	Client: sp_api::ProvideRuntimeApi<Block> + Send + Sync,
	Client::Api: AuraApi<Block, AuraId>,
{
	async fn produce_candidate(
		&mut self,
		parent: &Header,
		relay_parent: PHash,
		validation_data: &PersistedValidationData,
	) -> Option<ParachainCandidate<Block>> {
		let block_id = BlockId::hash(parent.hash());
		if self
			.client
			.runtime_api()
			.has_api::<dyn AuraApi<Block, AuraId>>(&block_id)
			.unwrap_or(false)
		{
			self.aura_consensus
				.lock()
				.await
				.get_mut()
				.produce_candidate(parent, relay_parent, validation_data)
				.await
		} else {
			self.relay_chain_consensus
				.lock()
				.await
				.produce_candidate(parent, relay_parent, validation_data)
				.await
		}
	}
}

struct Verifier<Client> {
	client: Arc<Client>,
	aura_verifier: BuildOnAccess<Box<dyn VerifierT<Block>>>,
	relay_chain_verifier: Box<dyn VerifierT<Block>>,
}

#[async_trait::async_trait]
impl<Client> VerifierT<Block> for Verifier<Client>
where
	Client: sp_api::ProvideRuntimeApi<Block> + Send + Sync,
	Client::Api: AuraApi<Block, AuraId>,
{
	async fn verify(
		&mut self,
		block_import: BlockImportParams<Block, ()>,
	) -> Result<
		(
			BlockImportParams<Block, ()>,
			Option<Vec<(CacheKeyId, Vec<u8>)>>,
		),
		String,
	> {
		let block_id = BlockId::hash(*block_import.header.parent_hash());

		if self
			.client
			.runtime_api()
			.has_api::<dyn AuraApi<Block, AuraId>>(&block_id)
			.unwrap_or(false)
		{
			self.aura_verifier.get_mut().verify(block_import).await
		} else {
			self.relay_chain_verifier.verify(block_import).await
		}
	}
}

/// Build the import queue for the calamari/manta-pc runtime.
pub fn parachain_build_import_queue<RuntimeApi, Executor>(
	client: Arc<TFullClient<Block, RuntimeApi, Executor>>,
	config: &Configuration,
	telemetry_handle: Option<TelemetryHandle>,
	task_manager: &TaskManager,
) -> Result<
	sc_consensus::DefaultImportQueue<Block, TFullClient<Block, RuntimeApi, Executor>>,
	sc_service::Error,
>
where
	RuntimeApi: ConstructRuntimeApi<Block, TFullClient<Block, RuntimeApi, Executor>>
		+ Send
		+ Sync
		+ 'static,
	RuntimeApi::RuntimeApi: sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block>
		+ sp_api::Metadata<Block>
		+ sp_session::SessionKeys<Block>
		+ sp_api::ApiExt<
			Block,
			StateBackend = sc_client_api::StateBackendFor<TFullBackend<Block>, Block>,
		> + sp_offchain::OffchainWorkerApi<Block>
		+ sp_block_builder::BlockBuilder<Block>
		+ sp_consensus_aura::AuraApi<Block, AuraId>,
	sc_client_api::StateBackendFor<TFullBackend<Block>, Block>: sp_api::StateBackend<BlakeTwo256>,
	Executor: sc_executor::NativeExecutionDispatch + 'static,
{
	let client2 = client.clone();

	let aura_verifier = move || {
		let slot_duration = cumulus_client_consensus_aura::slot_duration(&*client2).unwrap();

		Box::new(cumulus_client_consensus_aura::build_verifier::<
			sp_consensus_aura::sr25519::AuthorityPair,
			_,
			_,
			_,
		>(cumulus_client_consensus_aura::BuildVerifierParams {
			client: client2.clone(),
			create_inherent_data_providers: move |_, _| async move {
				let time = sp_timestamp::InherentDataProvider::from_system_time();

				let slot =
					sp_consensus_aura::inherents::InherentDataProvider::from_timestamp_and_duration(
						*time,
						slot_duration.slot_duration(),
					);

				Ok((time, slot))
			},
			can_author_with: sp_consensus::CanAuthorWithNativeVersion::new(
				client2.executor().clone(),
			),
			telemetry: telemetry_handle,
		})) as Box<_>
	};

	let relay_chain_verifier = Box::new(RelayChainVerifier::new(client.clone(), |_, _| async {
		Ok(())
	})) as Box<_>;

	let verifier = Verifier {
		client: client.clone(),
		relay_chain_verifier,
		aura_verifier: BuildOnAccess::Uninitialized(Some(Box::new(aura_verifier))),
	};

	let registry = config.prometheus_registry();
	let spawner = task_manager.spawn_essential_handle();

	Ok(BasicQueue::new(
		verifier,
		Box::new(ParachainBlockImport::new(client)),
		None,
		&spawner,
		registry,
	))
}

/// Start a calamari/manta-pc parachain node.
pub async fn start_parachain_node<RuntimeApi, Executor>(
	parachain_config: Configuration,
	polkadot_config: Configuration,
	id: ParaId,
) -> sc_service::error::Result<(TaskManager, Arc<TFullClient<Block, RuntimeApi, Executor>>)>
where
	RuntimeApi: ConstructRuntimeApi<Block, TFullClient<Block, RuntimeApi, Executor>>
		+ Send
		+ Sync
		+ 'static,
	RuntimeApi::RuntimeApi: sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block>
		+ sp_api::Metadata<Block>
		+ sp_session::SessionKeys<Block>
		+ sp_api::ApiExt<
			Block,
			StateBackend = sc_client_api::StateBackendFor<TFullBackend<Block>, Block>,
		> + sp_offchain::OffchainWorkerApi<Block>
		+ sp_block_builder::BlockBuilder<Block>
		+ cumulus_primitives_core::CollectCollationInfo<Block>
		+ sp_consensus_aura::AuraApi<Block, AuraId>
		+ pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<
			Block,
			manta_primitives::Balance,
		>,
	sc_client_api::StateBackendFor<TFullBackend<Block>, Block>: sp_api::StateBackend<BlakeTwo256>,
	Executor: sc_executor::NativeExecutionDispatch + 'static,
{
	start_node_impl::<RuntimeApi, Executor, _, _, _>(
		parachain_config,
		polkadot_config,
		id,
		|_| Ok(Default::default()),
		parachain_build_import_queue,
		|client,
		 prometheus_registry,
		 telemetry,
		 task_manager,
		 relay_chain_node,
		 transaction_pool,
		 sync_oracle,
		 keystore,
		 force_authoring| {
			let client2 = client.clone();
			let relay_chain_backend = relay_chain_node.backend.clone();
			let relay_chain_client = relay_chain_node.client.clone();
			let spawn_handle = task_manager.spawn_handle();
			let transaction_pool2 = transaction_pool.clone();
			let telemetry2 = telemetry.clone();
			let prometheus_registry2 = prometheus_registry.map(|r| (*r).clone());

			let aura_consensus = BuildOnAccess::Uninitialized(Some(Box::new(move || {
				let slot_duration =
					cumulus_client_consensus_aura::slot_duration(&*client2).unwrap();

				let proposer_factory = sc_basic_authorship::ProposerFactory::with_proof_recording(
					spawn_handle,
					client2.clone(),
					transaction_pool2,
					prometheus_registry2.as_ref(),
					telemetry2.clone(),
				);

				let relay_chain_backend2 = relay_chain_backend.clone();
				let relay_chain_client2 = relay_chain_client.clone();

				build_aura_consensus::<
					sp_consensus_aura::sr25519::AuthorityPair,
					_,
					_,
					_,
					_,
					_,
					_,
					_,
					_,
					_,
				>(BuildAuraConsensusParams {
					proposer_factory,
					create_inherent_data_providers: move |_, (relay_parent, validation_data)| {
						let parachain_inherent =
								cumulus_primitives_parachain_inherent::ParachainInherentData::create_at_with_client(
									relay_parent,
									&relay_chain_client,
									&*relay_chain_backend,
									&validation_data,
									id,
								);
						async move {
							let time = sp_timestamp::InherentDataProvider::from_system_time();

							let slot =
									sp_consensus_aura::inherents::InherentDataProvider::from_timestamp_and_duration(
										*time,
										slot_duration.slot_duration(),
									);

							let parachain_inherent = parachain_inherent.ok_or_else(|| {
								Box::<dyn std::error::Error + Send + Sync>::from(
									"Failed to create parachain inherent",
								)
							})?;
							Ok((time, slot, parachain_inherent))
						}
					},
					block_import: client2.clone(),
					relay_chain_client: relay_chain_client2,
					relay_chain_backend: relay_chain_backend2,
					para_client: client2.clone(),
					backoff_authoring_blocks: Option::<()>::None,
					sync_oracle,
					keystore,
					force_authoring,
					slot_duration,
					// We got around 500ms for proposing
					block_proposal_slot_portion: SlotProportion::new(1f32 / 24f32),
					// And a maximum of 750ms if slots are skipped
					max_block_proposal_slot_portion: Some(SlotProportion::new(1f32 / 16f32)),
					telemetry: telemetry2,
				})
			})));

			let proposer_factory = sc_basic_authorship::ProposerFactory::with_proof_recording(
				task_manager.spawn_handle(),
				client.clone(),
				transaction_pool,
				prometheus_registry,
				telemetry,
			);

			let relay_chain_backend = relay_chain_node.backend.clone();
			let relay_chain_client = relay_chain_node.client.clone();

			let relay_chain_consensus =
				cumulus_client_consensus_relay_chain::build_relay_chain_consensus(
					cumulus_client_consensus_relay_chain::BuildRelayChainConsensusParams {
						para_id: id,
						proposer_factory,
						block_import: client.clone(),
						relay_chain_client: relay_chain_node.client.clone(),
						relay_chain_backend: relay_chain_node.backend.clone(),
						create_inherent_data_providers:
							move |_, (relay_parent, validation_data)| {
								let parachain_inherent =
									cumulus_primitives_parachain_inherent::ParachainInherentData::create_at_with_client(
										relay_parent,
										&relay_chain_client,
										&*relay_chain_backend,
										&validation_data,
										id,
									);
								async move {
									let parachain_inherent =
										parachain_inherent.ok_or_else(|| {
											Box::<dyn std::error::Error + Send + Sync>::from(
												"Failed to create parachain inherent",
											)
										})?;
									Ok(parachain_inherent)
								}
							},
					},
				);

			let parachain_consensus = Box::new(WaitForAuraConsensus {
				client,
				aura_consensus: Arc::new(Mutex::new(aura_consensus)),
				relay_chain_consensus: Arc::new(Mutex::new(relay_chain_consensus)),
			});

			Ok(parachain_consensus)
		},
	)
	.await
}