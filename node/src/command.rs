use crate::{
	benchmarking::{inherent_benchmark_data, RemarkBuilder, TransferKeepAliveBuilder, SetNameBuilder, UpdateSomeNumBuilder, GetSomeNumBuilder, UpdateSomeStrBuilder, GetSomeStrBuilder, FibonacciBuilder, OddProductBuilder, TriangleNumBuilder, SampleEventEmitBuilder, CrossPalletCallBuilder, InkUpdateNumBuilder, InkGetNumBuilder,InkUpdateSBuilder, InkGetSBuilder, InkFibonacciBuilder, InkOddProductBuilder, InkTriangleNumberBuilder, InkSampleEmitBuilder, InkCrossContractCallBuilder, SoliditySetSomeNumBuilder, SolidityGetSomeNumBuilder, SoliditySetSomeStrBuilder, SolidityGetSomeStrBuilder, SolidityFibonacciBuilder, SolidityOddProductBuilder, SolidityTriangleNumBuilder, SoliditySampleEventBuilder},
	chain_spec,
	cli::{Cli, Subcommand},
	service,
};
use frame_benchmarking_cli::{BenchmarkCmd, ExtrinsicFactory, SUBSTRATE_REFERENCE_HARDWARE};
use node_template_runtime::{Block, EXISTENTIAL_DEPOSIT};
use sc_cli::{ChainSpec, RuntimeVersion, SubstrateCli};
use sc_service::PartialComponents;
use sp_keyring::Sr25519Keyring;
use std::fs;

fn read_contract_address() -> String {
	let contents = fs::read_to_string("./contract_address.txt").expect("failed to read 'contract_address.txt'");
	// println!("{}", contents);
	contents.trim().into()
}

fn read_callee_contract_address() -> String {
	let contents = fs::read_to_string("./callee_contract_address.txt").expect("failed to read 'callee_contract_address.txt'");
	// println!("{}", contents);
	contents.trim().into()
}

impl SubstrateCli for Cli {
	fn impl_name() -> String {
		"Substrate Node".into()
	}

	fn impl_version() -> String {
		env!("SUBSTRATE_CLI_IMPL_VERSION").into()
	}

	fn description() -> String {
		env!("CARGO_PKG_DESCRIPTION").into()
	}

	fn author() -> String {
		env!("CARGO_PKG_AUTHORS").into()
	}

	fn support_url() -> String {
		"support.anonymous.an".into()
	}

	fn copyright_start_year() -> i32 {
		2017
	}

	fn load_spec(&self, id: &str) -> Result<Box<dyn sc_service::ChainSpec>, String> {
		Ok(match id {
			"dev" => Box::new(chain_spec::development_config()?),
			"" | "local" => Box::new(chain_spec::local_testnet_config()?),
			path =>
				Box::new(chain_spec::ChainSpec::from_json_file(std::path::PathBuf::from(path))?),
		})
	}

	fn native_runtime_version(_: &Box<dyn ChainSpec>) -> &'static RuntimeVersion {
		&node_template_runtime::VERSION
	}
}

/// Parse and run command line arguments
pub fn run() -> sc_cli::Result<()> {
	let mut cli = Cli::from_args();

	// this is a development node: make dev chain spec the default
	if cli.run.shared_params.chain.is_none() {
		cli.run.shared_params.dev = true;
	}

	// remove block production noise and output contracts debug buffer by default
	if cli.run.shared_params.log.is_empty() {
		cli.run.shared_params.log = vec![
			"runtime::contracts=debug".into(),
			"sc_cli=info".into(),
			"sc_rpc_server=info".into(),
			"warn".into(),
		];
	}

	match &cli.subcommand {
		Some(Subcommand::Key(cmd)) => cmd.run(&cli),
		Some(Subcommand::BuildSpec(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
		},
		Some(Subcommand::CheckBlock(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let PartialComponents { client, task_manager, import_queue, .. } =
					service::new_partial(&config)?;
				Ok((cmd.run(client, import_queue), task_manager))
			})
		},
		Some(Subcommand::ExportBlocks(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let PartialComponents { client, task_manager, .. } = service::new_partial(&config)?;
				Ok((cmd.run(client, config.database), task_manager))
			})
		},
		Some(Subcommand::ExportState(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let PartialComponents { client, task_manager, .. } = service::new_partial(&config)?;
				Ok((cmd.run(client, config.chain_spec), task_manager))
			})
		},
		Some(Subcommand::ImportBlocks(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let PartialComponents { client, task_manager, import_queue, .. } =
					service::new_partial(&config)?;
				Ok((cmd.run(client, import_queue), task_manager))
			})
		},
		Some(Subcommand::PurgeChain(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| cmd.run(config.database))
		},
		Some(Subcommand::Revert(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let PartialComponents { client, task_manager, backend, .. } =
					service::new_partial(&config)?;
				let aux_revert = Box::new(|client, _, blocks| {
					sc_finality_grandpa::revert(client, blocks)?;
					Ok(())
				});
				Ok((cmd.run(client, backend, Some(aux_revert)), task_manager))
			})
		},
		Some(Subcommand::Benchmark(cmd)) => {
			let runner = cli.create_runner(cmd)?;

			runner.sync_run(|config| {
				// This switch needs to be in the client, since the client decides
				// which sub-commands it wants to support.
				match cmd {
					BenchmarkCmd::Pallet(cmd) => {
						if !cfg!(feature = "runtime-benchmarks") {
							return Err(
								"Runtime benchmarking wasn't enabled when building the node. \
							You can enable it with `--features runtime-benchmarks`."
									.into(),
							)
						}

						cmd.run::<Block, service::ExecutorDispatch>(config)
					},
					BenchmarkCmd::Block(cmd) => {
						let PartialComponents { client, .. } = service::new_partial(&config)?;
						cmd.run(client)
					},
					#[cfg(not(feature = "runtime-benchmarks"))]
					BenchmarkCmd::Storage(_) => Err(
						"Storage benchmarking can be enabled with `--features runtime-benchmarks`."
							.into(),
					),
					#[cfg(feature = "runtime-benchmarks")]
					BenchmarkCmd::Storage(cmd) => {
						let PartialComponents { client, backend, .. } =
							service::new_partial(&config)?;
						let db = backend.expose_db();
						let storage = backend.expose_storage();

						cmd.run(config, client, db, storage)
					},
					BenchmarkCmd::Overhead(cmd) => {
						let PartialComponents { client, .. } = service::new_partial(&config)?;
						let ext_builder = RemarkBuilder::new(client.clone());

						cmd.run(
							config,
							client,
							inherent_benchmark_data()?,
							Vec::new(),
							&ext_builder,
						)
					},
					BenchmarkCmd::Extrinsic(cmd) => {
						let PartialComponents { client, .. } = service::new_partial(&config)?;
						// Register a few extrinsic builders so the CLI can iterate over and choose the right extrinsic builder that matches the one chosen by the user.
						let ext_factory = ExtrinsicFactory(vec![
							Box::new(RemarkBuilder::new(client.clone())),
							Box::new(TransferKeepAliveBuilder::new(
								client.clone(),
								Sr25519Keyring::Alice.to_account_id(),
								EXISTENTIAL_DEPOSIT,
							)),
							Box::new(SetNameBuilder::new(
								client.clone(),
								String::from("abcdefghijklmnopqrstuvwxyz123456").into_bytes()
							)),
							Box::new(UpdateSomeNumBuilder::new(
								client.clone(),
							)),
							Box::new(GetSomeNumBuilder::new(
								client.clone(),
							)),
							Box::new(UpdateSomeStrBuilder::new(
								client.clone(),
							)),
							Box::new(GetSomeStrBuilder::new(
								client.clone(),
							)),
							Box::new(FibonacciBuilder::new(
								client.clone(),
								10u32,
							)),
							Box::new(OddProductBuilder::new(
								client.clone(),
								10u32,
							)),
							Box::new(TriangleNumBuilder::new(
								client.clone(),
								10u32,
							)),
							Box::new(SampleEventEmitBuilder::new(
								client.clone(),
							)),
							Box::new(CrossPalletCallBuilder::new(
								client.clone(),
							)),
							Box::new(InkUpdateNumBuilder::new(
								client.clone(),
								read_contract_address(),
							)),
							Box::new(InkGetNumBuilder::new(
								client.clone(),
								read_contract_address(),
							)),
							Box::new(InkUpdateSBuilder::new(
								client.clone(),
								read_contract_address(),
							)),
							Box::new(InkGetSBuilder::new(
								client.clone(),
								read_contract_address(),
							)),
							Box::new(InkFibonacciBuilder::new(
								client.clone(),
								read_contract_address(),
							)),
							Box::new(InkOddProductBuilder::new(
								client.clone(),
								read_contract_address(),
							)),
							Box::new(InkTriangleNumberBuilder::new(
								client.clone(),
								read_contract_address(),
							)),
							Box::new(InkSampleEmitBuilder::new(
								client.clone(),
								read_contract_address(),
							)),
							Box::new(InkCrossContractCallBuilder::new(
								client.clone(),
								read_contract_address(),
								read_callee_contract_address(),
							)),
							Box::new(SoliditySetSomeNumBuilder::new(
								client.clone(),
								read_contract_address(),
							)),
							Box::new(SolidityGetSomeNumBuilder::new(
								client.clone(),
								read_contract_address(),
							)),
							Box::new(SoliditySetSomeStrBuilder::new(
								client.clone(),
								read_contract_address(),
							)),
							Box::new(SolidityGetSomeStrBuilder::new(
								client.clone(),
								read_contract_address(),
							)),
							Box::new(SolidityFibonacciBuilder::new(
								client.clone(),
								read_contract_address(),
							)),
							Box::new(SolidityOddProductBuilder::new(
								client.clone(),
								read_contract_address(),
							)),
							Box::new(SolidityTriangleNumBuilder::new(
								client.clone(),
								read_contract_address(),
							)),
							Box::new(SoliditySampleEventBuilder::new(
								client.clone(),
								read_contract_address(),
							)),
						]);
						cmd.run(client, inherent_benchmark_data()?, Vec::new(), &ext_factory)
					},
					BenchmarkCmd::Machine(cmd) =>
						cmd.run(&config, SUBSTRATE_REFERENCE_HARDWARE.clone()),
				}
			})
		},
		#[cfg(feature = "try-runtime")]
		Some(Subcommand::TryRuntime(cmd)) => {
			use crate::service::ExecutorDispatch;
			use sc_executor::{sp_wasm_interface::ExtendedHostFunctions, NativeExecutionDispatch};
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				// we don't need any of the components of new_partial, just a runtime, or a task
				// manager to do `async_run`.
				let registry = config.prometheus_config.as_ref().map(|cfg| &cfg.registry);
				let task_manager =
					sc_service::TaskManager::new(config.tokio_handle.clone(), registry)
						.map_err(|e| sc_cli::Error::Service(sc_service::Error::Prometheus(e)))?;
				Ok((
					cmd.run::<Block, ExtendedHostFunctions<
						sp_io::SubstrateHostFunctions,
						<ExecutorDispatch as NativeExecutionDispatch>::ExtendHostFunctions,
					>>(),
					task_manager,
				))
			})
		},
		#[cfg(not(feature = "try-runtime"))]
		Some(Subcommand::TryRuntime) => Err("TryRuntime wasn't enabled when building the node. \
				You can enable it with `--features try-runtime`."
			.into()),
		Some(Subcommand::ChainInfo(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| cmd.run::<Block>(&config))
		},
		None => {
			let runner = cli.create_runner(&cli.run)?;
			runner.run_node_until_exit(|config| async move {
				service::new_full(config).map_err(sc_cli::Error::Service)
			})
		},
	}
}
