// This file is part of Substrate.

// Copyright (C) 2017-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use clap::Parser;
// Substrate
use sc_cli::{ChainSpec, RuntimeVersion, SubstrateCli};
use sc_service::DatabaseSource;
// Frontier
use fc_db::frontier_database_dir;

use crate::{
	chain_spec,
	cli::{Cli, Subcommand},
	service::{self, db_config_dir},
};

use std::fs;
use std::path::Path;
// use std::io::{prelude::*, BufReader};

fn get_file_contents<P: AsRef<Path>>(file_path: P) -> String {
	let contents = fs::read_to_string(file_path).expect("failed to read file");
	println!("{}", contents);
	contents.trim().into()
}

fn read_contract_address() -> String {
	get_file_contents("./template/contract_address.txt")
}

fn read_source_address() -> String {
	get_file_contents("./template/source_address.txt")
}

fn read_call_data() -> String {
	get_file_contents("./template/call_data.txt")
}

// fn read_args_from_file() -> [String; 3] {
//     let file = File::open("./template/contents.txt").unwrap();
//     let mut reader = BufReader::new(file);

//     let mut contents: [String; 3] = [
//         String::new(),
//         String::new(),
//         String::new(),
//     ];
//     for i in 0..3 {
//         let _ = reader.read_line(&mut contents[i]);
//         if i < 2 {
//             contents[i].pop();
//         }
//     }
// 	for i in 0..3 {
// 		println!("{}", contents[i]);
// 	}
//     contents
// }

impl SubstrateCli for Cli {
	fn impl_name() -> String {
		"Frontier Node".into()
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
		2021
	}

	fn load_spec(&self, id: &str) -> Result<Box<dyn ChainSpec>, String> {
		Ok(match id {
			"dev" => {
				let enable_manual_seal = self.sealing.map(|_| true);
				Box::new(chain_spec::development_config(enable_manual_seal))
			}
			"" | "local" => Box::new(chain_spec::local_testnet_config()),
			path => Box::new(chain_spec::ChainSpec::from_json_file(
				std::path::PathBuf::from(path),
			)?),
		})
	}

	fn native_runtime_version(_: &Box<dyn ChainSpec>) -> &'static RuntimeVersion {
		&frontier_template_runtime::VERSION
	}
}

/// Parse and run command line arguments
pub fn run() -> sc_cli::Result<()> {
	let cli = Cli::parse();

	match &cli.subcommand {
		Some(Subcommand::Key(cmd)) => cmd.run(&cli),
		Some(Subcommand::BuildSpec(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
		}
		Some(Subcommand::CheckBlock(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|mut config| {
				let (client, _, import_queue, task_manager, _) =
					service::new_chain_ops(&mut config, &cli.eth)?;
				Ok((cmd.run(client, import_queue), task_manager))
			})
		}
		Some(Subcommand::ExportBlocks(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|mut config| {
				let (client, _, _, task_manager, _) =
					service::new_chain_ops(&mut config, &cli.eth)?;
				Ok((cmd.run(client, config.database), task_manager))
			})
		}
		Some(Subcommand::ExportState(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|mut config| {
				let (client, _, _, task_manager, _) =
					service::new_chain_ops(&mut config, &cli.eth)?;
				Ok((cmd.run(client, config.chain_spec), task_manager))
			})
		}
		Some(Subcommand::ImportBlocks(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|mut config| {
				let (client, _, import_queue, task_manager, _) =
					service::new_chain_ops(&mut config, &cli.eth)?;
				Ok((cmd.run(client, import_queue), task_manager))
			})
		}
		Some(Subcommand::PurgeChain(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| {
				// Remove Frontier offchain db
				let db_config_dir = db_config_dir(&config);
				let frontier_database_config = match config.database {
					DatabaseSource::RocksDb { .. } => DatabaseSource::RocksDb {
						path: frontier_database_dir(&db_config_dir, "db"),
						cache_size: 0,
					},
					DatabaseSource::ParityDb { .. } => DatabaseSource::ParityDb {
						path: frontier_database_dir(&db_config_dir, "paritydb"),
					},
					_ => {
						return Err(format!("Cannot purge `{:?}` database", config.database).into())
					}
				};
				cmd.run(frontier_database_config)?;
				cmd.run(config.database)
			})
		}
		Some(Subcommand::Revert(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|mut config| {
				let (client, backend, _, task_manager, _) =
					service::new_chain_ops(&mut config, &cli.eth)?;
				let aux_revert = Box::new(move |client, _, blocks| {
					sc_finality_grandpa::revert(client, blocks)?;
					Ok(())
				});
				Ok((cmd.run(client, backend, Some(aux_revert)), task_manager))
			})
		}
		#[cfg(feature = "runtime-benchmarks")]
		Some(Subcommand::Benchmark(cmd)) => {
			use crate::benchmarking::{
				inherent_benchmark_data, RemarkBuilder, TransferKeepAliveBuilder, NativeSolidityGenericCallBuilder,
			};
			use frame_benchmarking_cli::{
				BenchmarkCmd, ExtrinsicFactory, SUBSTRATE_REFERENCE_HARDWARE,
			};
			use frontier_template_runtime::{Block, ExistentialDeposit};

			let runner = cli.create_runner(cmd)?;
			match cmd {
				BenchmarkCmd::Pallet(cmd) => runner
					.sync_run(|config| cmd.run::<Block, service::TemplateRuntimeExecutor>(config)),
				BenchmarkCmd::Block(cmd) => runner.sync_run(|mut config| {
					let (client, _, _, _, _) = service::new_chain_ops(&mut config, &cli.eth)?;
					cmd.run(client)
				}),
				BenchmarkCmd::Storage(cmd) => runner.sync_run(|mut config| {
					let (client, backend, _, _, _) = service::new_chain_ops(&mut config, &cli.eth)?;
					let db = backend.expose_db();
					let storage = backend.expose_storage();
					cmd.run(config, client, db, storage)
				}),
				BenchmarkCmd::Overhead(cmd) => runner.sync_run(|mut config| {
					let (client, _, _, _, _) = service::new_chain_ops(&mut config, &cli.eth)?;
					let ext_builder = RemarkBuilder::new(client.clone());
					cmd.run(
						config,
						client,
						inherent_benchmark_data()?,
						Vec::new(),
						&ext_builder,
					)
				}),
				BenchmarkCmd::Extrinsic(cmd) => runner.sync_run(|mut config| {
					let (client, _, _, _, _) = service::new_chain_ops(&mut config, &cli.eth)?;
					// let file_contents = read_args_from_file();
					// Register the *Remark* and *TKA* builders.
					let ext_factory = ExtrinsicFactory(vec![
						Box::new(RemarkBuilder::new(client.clone())),
						Box::new(TransferKeepAliveBuilder::new(
							client.clone(),
							sp_keyring::Sr25519Keyring::Alice.to_account_id(),
							ExistentialDeposit::get(),
						)),
						Box::new(NativeSolidityGenericCallBuilder::new(
							client.clone(),
							read_source_address(),
							read_contract_address(),
							String::from("9ac1762f00000000000000000000000000000000000000000000000000000000000003e8"),
							("native_solidity_benchmark_sample").clone(),
							("set_some_num").clone(),
						)),
						Box::new(NativeSolidityGenericCallBuilder::new(
							client.clone(),
							read_source_address(),
							read_contract_address(),
							String::from("b40a136b"),
							("native_solidity_benchmark_sample").clone(),
							("get_some_num").clone(),
						)),
						Box::new(NativeSolidityGenericCallBuilder::new(
							client.clone(),
							read_source_address(),
							read_contract_address(),
							String::from("0583a02c0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000003342656e63686d61726b696e6720536f6c6964697479204e617469766520636f6e747261637473206f6e2053756273747261746500000000000000000000000000"),
							("native_solidity_benchmark_sample").clone(),
							("set_some_str").clone(),
						)),
						Box::new(NativeSolidityGenericCallBuilder::new(
							client.clone(),
							read_source_address(),
							read_contract_address(),
							String::from("5e9c155e"),
							("native_solidity_benchmark_sample").clone(),
							("get_some_str").clone(),
						)),
						Box::new(NativeSolidityGenericCallBuilder::new(
							client.clone(),
							read_source_address(),
							read_contract_address(),
							read_call_data(),
							("native_solidity").clone(),
							("generic_call").clone(),
						)),
					]);

					cmd.run(client, inherent_benchmark_data()?, Vec::new(), &ext_factory)
				}),
				BenchmarkCmd::Machine(cmd) => {
					runner.sync_run(|config| cmd.run(&config, SUBSTRATE_REFERENCE_HARDWARE.clone()))
				}
			}
		}
		#[cfg(not(feature = "runtime-benchmarks"))]
		Some(Subcommand::Benchmark) => Err("Benchmarking wasn't enabled when building the node. \
			You can enable it with `--features runtime-benchmarks`."
			.into()),
		Some(Subcommand::FrontierDb(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|mut config| {
				let (client, _, _, _, frontier_backend) =
					service::new_chain_ops(&mut config, &cli.eth)?;
				cmd.run(client, frontier_backend)
			})
		}
		None => {
			let runner = cli.create_runner(&cli.run)?;
			runner.run_node_until_exit(|config| async move {
				service::build_full(config, cli.eth, cli.sealing).map_err(Into::into)
			})
		}
	}
}
