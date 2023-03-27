// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Contains code to setup the command invocations in [`super::command`] which would
//! otherwise bloat that module.

use std::{sync::Arc, time::Duration, str::FromStr};

use scale_codec::Encode;
// Substrate
use sc_cli::Result;
use sc_client_api::BlockBackend;
use sp_core::{sr25519, Pair, H160, U256};
use sp_inherents::{InherentData, InherentDataProvider};
use sp_keyring::Sr25519Keyring;
use sp_runtime::{generic::Era, AccountId32, OpaqueExtrinsic, SaturatedConversion};
// Frontier
use frontier_template_runtime::{self as runtime, AccountId, Balance, BalancesCall, SystemCall, EvmCall};

use crate::client::Client;

fn str_to_h160(s: &String) -> H160 {
	let s_str = s.as_str();
	return H160::from_str(s_str).unwrap();
}

fn hex_string_to_bytes(s: &String) -> Option<Vec<u8>> {
    if s.len() % 2 == 0 {
        (0..s.len())
            .step_by(2)
            .map(|i| s.get(i..i + 2)
                      .and_then(|sub| u8::from_str_radix(sub, 16).ok()))
            .collect()
    } else {
        None
    }
}

/// Generates extrinsics for the `benchmark overhead` command.
///
/// Note: Should only be used for benchmarking.
pub struct RemarkBuilder {
	client: Arc<Client>,
}

impl RemarkBuilder {
	/// Creates a new [`Self`] from the given client.
	pub fn new(client: Arc<Client>) -> Self {
		Self { client }
	}
}

impl frame_benchmarking_cli::ExtrinsicBuilder for RemarkBuilder {
	fn pallet(&self) -> &str {
		"system"
	}

	fn extrinsic(&self) -> &str {
		"remark"
	}

	fn build(&self, nonce: u32) -> std::result::Result<OpaqueExtrinsic, &'static str> {
		let acc = Sr25519Keyring::Bob.pair();
		let extrinsic: OpaqueExtrinsic = create_benchmark_extrinsic(
			self.client.as_ref(),
			acc,
			SystemCall::remark { remark: vec![] }.into(),
			nonce,
		)
		.into();

		Ok(extrinsic)
	}
}

/// Generates `Balances::TransferKeepAlive` extrinsics for the benchmarks.
///
/// Note: Should only be used for benchmarking.
pub struct TransferKeepAliveBuilder {
	client: Arc<Client>,
	dest: AccountId,
	value: Balance,
}

impl TransferKeepAliveBuilder {
	/// Creates a new [`Self`] from the given client.
	pub fn new(client: Arc<Client>, dest: AccountId, value: Balance) -> Self {
		Self {
			client,
			dest,
			value,
		}
	}
}

impl frame_benchmarking_cli::ExtrinsicBuilder for TransferKeepAliveBuilder {
	fn pallet(&self) -> &str {
		"balances"
	}

	fn extrinsic(&self) -> &str {
		"transfer_keep_alive"
	}

	fn build(&self, nonce: u32) -> std::result::Result<OpaqueExtrinsic, &'static str> {
		let acc = Sr25519Keyring::Bob.pair();
		let extrinsic: OpaqueExtrinsic = create_benchmark_extrinsic(
			self.client.as_ref(),
			acc,
			BalancesCall::transfer_keep_alive {
				dest: self.dest.clone().into(),
				value: self.value,
			}
			.into(),
			nonce,
		)
		.into();

		Ok(extrinsic)
	}
}

// Generates a EvmCall::call that represents a generic call.
// source is the Ethereum-type address that's making the call.
// target is the address of the contract that this message will be sent to.
// input is the call data sent to the native solidity contract that determines which solidity function gets invoked and the arguments that get passed in.
// input is read from `input.txt` and hence, this call is generic and can represent invoking any solidity function with any arguments depending upon the contents of `input.txt`.
pub struct NativeSolidityGenericCallBuilder {
	client: Arc<Client>,
	source: String,
	target: String,
	input: String,
	pallet_name: &'static str,
	extrinsic_name: &'static str,
}

impl NativeSolidityGenericCallBuilder {
	pub fn new(client: Arc<Client>, source: String, target: String, input: String, pallet_name: &'static str, extrinsic_name: &'static str) -> Self {
		Self {
			client,
			source,
			target,
			input,
			pallet_name,
			extrinsic_name,
		}
	}
}

impl frame_benchmarking_cli::ExtrinsicBuilder for NativeSolidityGenericCallBuilder {
	fn pallet(&self) -> &str {
		self.pallet_name.into()
	}

	fn extrinsic(&self) -> &str {
		self.extrinsic_name.into()
	}

	fn build(&self, nonce: u32) -> std::result::Result<OpaqueExtrinsic, &'static str> {
		let acc = Sr25519Keyring::Bob.pair();
		let extrinsic: OpaqueExtrinsic = create_benchmark_extrinsic(
			self.client.as_ref(),
			acc,
			EvmCall::call {
				source: str_to_h160(&self.source),
				target: str_to_h160(&self.target),
				input: hex_string_to_bytes(&self.input).unwrap(),
				value: 0.into(),
				gas_limit: 42949672u64,
				max_fee_per_gas: 1000000000u128.into(),
				max_priority_fee_per_gas: Default::default(),
				nonce: Some(U256::from(nonce+1)),
				access_list: Default::default(),
			}
			.into(),
			nonce,
		)
		.into();

		Ok(extrinsic)
	}
}
/// Create a transaction using the given `call`.
///
/// Note: Should only be used for benchmarking.
pub fn create_benchmark_extrinsic(
	client: &Client,
	sender: sr25519::Pair,
	call: runtime::RuntimeCall,
	nonce: u32,
) -> runtime::UncheckedExtrinsic {
	let genesis_hash = client
		.block_hash(0)
		.ok()
		.flatten()
		.expect("Genesis block exists; qed");
	let best_hash = client.chain_info().best_hash;
	let best_block = client.chain_info().best_number;

	let period = runtime::BlockHashCount::get()
		.checked_next_power_of_two()
		.map(|c| c / 2)
		.unwrap_or(2) as u64;
	let extra: runtime::SignedExtra = (
		frame_system::CheckNonZeroSender::<runtime::Runtime>::new(),
		frame_system::CheckSpecVersion::<runtime::Runtime>::new(),
		frame_system::CheckTxVersion::<runtime::Runtime>::new(),
		frame_system::CheckGenesis::<runtime::Runtime>::new(),
		frame_system::CheckMortality::<runtime::Runtime>::from(Era::mortal(
			period,
			best_block.saturated_into(),
		)),
		frame_system::CheckNonce::<runtime::Runtime>::from(nonce),
		frame_system::CheckWeight::<runtime::Runtime>::new(),
		pallet_transaction_payment::ChargeTransactionPayment::<runtime::Runtime>::from(0),
	);

	let raw_payload = runtime::SignedPayload::from_raw(
		call.clone(),
		extra.clone(),
		(
			(),
			runtime::VERSION.spec_version,
			runtime::VERSION.transaction_version,
			genesis_hash,
			best_hash,
			(),
			(),
			(),
		),
	);
	let signature = raw_payload.using_encoded(|e| sender.sign(e));

	runtime::UncheckedExtrinsic::new_signed(
		call,
		AccountId32::from(sender.public()).into(),
		runtime::Signature::Sr25519(signature),
		extra,
	)
}

/// Generates inherent data for the `benchmark overhead` command.
///
/// Note: Should only be used for benchmarking.
pub fn inherent_benchmark_data() -> Result<InherentData> {
	let mut inherent_data = InherentData::new();
	let d = Duration::from_millis(0);
	let timestamp = sp_timestamp::InherentDataProvider::new(d.into());

	futures::executor::block_on(timestamp.provide_inherent_data(&mut inherent_data))
		.map_err(|e| format!("creating inherent data: {:?}", e))?;
	Ok(inherent_data)
}
