//! Setup code for [`super::command`] which would otherwise bloat that module.
//!
//! Should only be used for benchmarking as it may break in other contexts.

use crate::service::FullClient;

use node_template_runtime as runtime;
use runtime::{AccountId, Balance, BalancesCall, SystemCall, NicksCall, TemplateCall, ContractsCall, Address};
use sc_cli::Result;
use sc_client_api::BlockBackend;
use sp_core::{Encode, Pair};
use sp_inherents::{InherentData, InherentDataProvider};
use sp_keyring::Sr25519Keyring;
use sp_runtime::{OpaqueExtrinsic, SaturatedConversion, AccountId32};

use std::{sync::Arc, time::Duration, str::FromStr};

use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

// generate a random integer
fn generate_random_integer() -> i64 {
	let mut trng = thread_rng();
	trng.gen()
}

// generate a random string of alphanumeric characters of length 100
fn generate_random_string() -> String {
	let trng = thread_rng();
	let length: usize = 100;
	// let length: usize = trng.gen_range(0..100);
	trng.sample_iter(Alphanumeric)
                .take(length)
                .map(char::from)
                .collect()
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
	client: Arc<FullClient>,
}

impl RemarkBuilder {
	/// Creates a new [`Self`] from the given client.
	pub fn new(client: Arc<FullClient>) -> Self {
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
	client: Arc<FullClient>,
	dest: AccountId,
	value: Balance,
}

impl TransferKeepAliveBuilder {
	/// Creates a new [`Self`] from the given client.
	pub fn new(client: Arc<FullClient>, dest: AccountId, value: Balance) -> Self {
		Self { client, dest, value }
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
				value: self.value.into(),
			}
			.into(),
			nonce,
		)
		.into();

		Ok(extrinsic)
	}
}

/// Generates `Nicks::SetName` extrinsics for the benchmarks.
///
/// Note: Should only be used for benchmarking.
pub struct SetNameBuilder {
	client: Arc<FullClient>,
	name: Vec<u8>,
}

impl SetNameBuilder {
	/// Creates a new [`Self`] from the given client.
	pub fn new(client: Arc<FullClient>, name: Vec<u8>) -> Self {
		Self { client, name }
	}
}

impl frame_benchmarking_cli::ExtrinsicBuilder for SetNameBuilder {

	fn pallet(&self) -> &str {
		"nicks"
	}

	fn extrinsic(&self) -> &str {
		"set_name"
	}

	fn build(&self, nonce: u32) -> std::result::Result<OpaqueExtrinsic, &'static str> {
		let acc = Sr25519Keyring::Bob.pair();
		let extrinsic: OpaqueExtrinsic = create_benchmark_extrinsic(
			self.client.as_ref(),
			acc,
			NicksCall::set_name {
				name: self.name.clone().into(),
			}
			.into(),
			nonce,
		)
		.into();

		Ok(extrinsic)
	}
}

/// Generates `Template::UpdateSomeNum` extrinsics for the benchmarks.
///
/// Note: Should only be used for benchmarking.
/// A random integer is generated and passed as an argument.
pub struct UpdateSomeNumBuilder {
	client: Arc<FullClient>,
}

impl UpdateSomeNumBuilder {
	pub fn new(client: Arc<FullClient>) -> Self {
		Self { client }
	}
}

impl frame_benchmarking_cli::ExtrinsicBuilder for UpdateSomeNumBuilder {
	fn pallet(&self) -> &str {
		"template"
	}

	fn extrinsic(&self) -> &str {
		"update_some_num"
	}

	fn build(&self, nonce: u32) -> std::result::Result<OpaqueExtrinsic, &'static str> {
		let acc = Sr25519Keyring::Bob.pair();
		let extrinsic: OpaqueExtrinsic = create_benchmark_extrinsic(
			self.client.as_ref(),
			acc,
			TemplateCall::update_some_num {
				value: generate_random_integer().clone().into(),
			}
			.into(),
			nonce,
		)
		.into();

		Ok(extrinsic)
	}
}

/// Generates `Template::GetSomeNum` extrinsics for the benchmarks.
///
/// Note: Should only be used for benchmarking.
pub struct GetSomeNumBuilder {
	client: Arc<FullClient>,
}

impl GetSomeNumBuilder {
	pub fn new(client: Arc<FullClient>) -> Self {
		Self { client }
	}
}

impl frame_benchmarking_cli::ExtrinsicBuilder for GetSomeNumBuilder {
	fn pallet(&self) -> &str {
		"template"
	}

	fn extrinsic(&self) -> &str {
		"get_some_num"
	}

	fn build(&self, nonce: u32) -> std::result::Result<OpaqueExtrinsic, &'static str> {
		let acc = Sr25519Keyring::Bob.pair();
		let extrinsic: OpaqueExtrinsic = create_benchmark_extrinsic(
			self.client.as_ref(),
			acc,
			TemplateCall::get_some_num {}
			.into(),
			nonce,
		)
		.into();

		Ok(extrinsic)
	}
}

/// Generates `Template::UpdateSomeStr` extrinsics for the benchmarks.
///
/// Note: Should only be used for benchmarking.
/// A random alphanumeric string is generated and passed as an argument.
pub struct UpdateSomeStrBuilder {
	client: Arc<FullClient>,
}

impl UpdateSomeStrBuilder {
	pub fn new(client: Arc<FullClient>) -> Self {
		Self { client }
	}
}

impl frame_benchmarking_cli::ExtrinsicBuilder for UpdateSomeStrBuilder {

	fn pallet(&self) -> &str {
		"template"
	}

	fn extrinsic(&self) -> &str {
		"update_some_str"
	}

	fn build(&self, nonce: u32) -> std::result::Result<OpaqueExtrinsic, &'static str> {
		let acc = Sr25519Keyring::Bob.pair();
		let extrinsic: OpaqueExtrinsic = create_benchmark_extrinsic(
			self.client.as_ref(),
			acc,
			TemplateCall::update_some_str {
				new_str: generate_random_string().into_bytes()
			}
			.into(),
			nonce,
		)
		.into();

		Ok(extrinsic)
	}
}

/// Generates `Template::GetSomeStr` extrinsics for the benchmarks.
///
/// Note: Should only be used for benchmarking.
pub struct GetSomeStrBuilder {
	client: Arc<FullClient>,
}

impl GetSomeStrBuilder {
	pub fn new(client: Arc<FullClient>) -> Self {
		Self { client }
	}
}

impl frame_benchmarking_cli::ExtrinsicBuilder for GetSomeStrBuilder {
	fn pallet(&self) -> &str {
		"template"
	}

	fn extrinsic(&self) -> &str {
		"get_some_str"
	}

	fn build(&self, nonce: u32) -> std::result::Result<OpaqueExtrinsic, &'static str> {
		let acc = Sr25519Keyring::Bob.pair();
		let extrinsic: OpaqueExtrinsic = create_benchmark_extrinsic(
			self.client.as_ref(),
			acc,
			TemplateCall::get_some_str {}
			.into(),
			nonce,
		)
		.into();

		Ok(extrinsic)
	}
}

pub struct FibonacciBuilder {
	client: Arc<FullClient>,
	num: u32,
}

impl FibonacciBuilder {
	pub fn new(client: Arc<FullClient>, num: u32) -> Self {
		Self {client, num}
	}
}

impl frame_benchmarking_cli::ExtrinsicBuilder for FibonacciBuilder {
	fn pallet(&self) -> &str {
		"template"
	}

	fn extrinsic(&self) -> &str {
		"fibonacci"
	}

	fn build(&self, nonce: u32) -> std::result::Result<OpaqueExtrinsic, &'static str> {
		let acc = Sr25519Keyring::Bob.pair();
		let extrinsic: OpaqueExtrinsic = create_benchmark_extrinsic(
			self.client.as_ref(),
			acc,
			TemplateCall::fibonacci {
				n: self.num.clone().into(),
			}
			.into(),
			nonce,
		)
		.into();

		Ok(extrinsic)
	}
}

pub struct OddProductBuilder {
	client: Arc<FullClient>,
	num: u32,
}

impl OddProductBuilder {
	pub fn new(client: Arc<FullClient>, num: u32) -> Self {
		Self {client, num}
	}
}

impl frame_benchmarking_cli::ExtrinsicBuilder for OddProductBuilder {
	fn pallet(&self) -> &str {
		"template"
	}

	fn extrinsic(&self) -> &str {
		"odd_product"
	}

	fn build(&self, nonce: u32) -> std::result::Result<OpaqueExtrinsic, &'static str> {
		let acc = Sr25519Keyring::Bob.pair();
		let extrinsic: OpaqueExtrinsic = create_benchmark_extrinsic(
			self.client.as_ref(),
			acc,
			TemplateCall::odd_product {
				n: self.num.clone().into(),
			}
			.into(),
			nonce,
		)
		.into();

		Ok(extrinsic)
	}
}

pub struct TriangleNumBuilder {
	client: Arc<FullClient>,
	num: u32,
}

impl TriangleNumBuilder {
	pub fn new(client: Arc<FullClient>, num: u32) -> Self {
		Self {client, num}
	}
}

impl frame_benchmarking_cli::ExtrinsicBuilder for TriangleNumBuilder {
	fn pallet(&self) -> &str {
		"template"
	}

	fn extrinsic(&self) -> &str {
		"triangle_number"
	}

	fn build(&self, nonce: u32) -> std::result::Result<OpaqueExtrinsic, &'static str> {
		let acc = Sr25519Keyring::Bob.pair();
		let extrinsic: OpaqueExtrinsic = create_benchmark_extrinsic(
			self.client.as_ref(),
			acc,
			TemplateCall::triangle_number {
				n: self.num.clone().into(),
			}
			.into(),
			nonce,
		)
		.into();

		Ok(extrinsic)
	}
}

pub struct SampleEventEmitBuilder {
	client: Arc<FullClient>,
}

impl SampleEventEmitBuilder {
	pub fn new(client: Arc<FullClient>) -> Self {
		Self {client}
	}
}

impl frame_benchmarking_cli::ExtrinsicBuilder for SampleEventEmitBuilder {
	fn pallet(&self) -> &str {
		"template"
	}

	fn extrinsic(&self) -> &str {
		"emit_sample_event"
	}

	fn build(&self, nonce: u32) -> std::result::Result<OpaqueExtrinsic, &'static str> {
		let acc = Sr25519Keyring::Bob.pair();
		let extrinsic: OpaqueExtrinsic = create_benchmark_extrinsic(
			self.client.as_ref(),
			acc,
			TemplateCall::emit_sample_event{}
			.into(),
			nonce,
		)
		.into();

		Ok(extrinsic)
	}
}

pub struct CrossPalletCallBuilder {
	client: Arc<FullClient>,
}

impl CrossPalletCallBuilder {
	pub fn new(client: Arc<FullClient>) -> Self {
		Self {client}
	}
}

impl frame_benchmarking_cli::ExtrinsicBuilder for CrossPalletCallBuilder {
	fn pallet(&self) -> &str {
		"template"
	}

	fn extrinsic(&self) -> &str {
		"cross_pallet_call"
	}

	fn build(&self, nonce: u32) -> std::result::Result<OpaqueExtrinsic, &'static str> {
		let acc = Sr25519Keyring::Bob.pair();
		let extrinsic: OpaqueExtrinsic = create_benchmark_extrinsic(
			self.client.as_ref(),
			acc,
			TemplateCall::store_num_in_callee{
				value: generate_random_integer().clone().into(),
			}
			.into(),
			nonce,
		)
		.into();

		Ok(extrinsic)
	}
}
/// Generates `Contracts::Call` extrinsics that represents calling the `test` contracts `update_num` message/function for the benchmarks.
///
/// Note: Should only be used for benchmarking.
/// generates a random integer and passes it as an argument.
pub struct InkUpdateNumBuilder {
	client: Arc<FullClient>,
	contract_addr: String,
}

impl InkUpdateNumBuilder {
	pub fn new(client: Arc<FullClient>, contract_addr: String) -> Self {
		Self { client, contract_addr }
	}
}

impl frame_benchmarking_cli::ExtrinsicBuilder for InkUpdateNumBuilder {
	fn pallet(&self) -> &str {
		"contract-test"
	}

	fn extrinsic(&self) -> &str {
		"update_num"
	}

	fn build(&self, nonce: u32) -> std::result::Result<OpaqueExtrinsic, &'static str> {
		let mut call_data: Vec<u8> = Vec::new();
		// `msg_selector` identifies the message to be invoked
		let mut msg_selector: Vec<u8> = [0xFB, 0xAF, 0x91, 0xE1].into();
		// `msg_args` are the SCALE-encoded args to be passed to the above message
		let mut msg_args = generate_random_integer().encode();
		// construct a call to a specific message with the arguments
		call_data.append(&mut msg_selector);
		call_data.append(&mut msg_args);
		let acc = Sr25519Keyring::Bob.pair();
		let extrinsic: OpaqueExtrinsic = create_benchmark_extrinsic(
			self.client.as_ref(),
			acc,
			ContractsCall::call {
				 dest: Address::Address32(*AccountId32::from_str(&*self.contract_addr).unwrap().as_ref()).into(),
				 value: Default::default(),
				 gas_limit: Default::default(),
				 storage_deposit_limit: Default::default(),
				 data: call_data.clone()
			}.into(),
			nonce,
		)
		.into();

		Ok(extrinsic)
	}
}

/// Generates `Contracts::Call` extrinsics that represents calling the `test` contracts `get_num` message/function for the benchmarks.
///
/// Note: Should only be used for benchmarking.
pub struct InkGetNumBuilder {
	client: Arc<FullClient>,
	contract_addr: String,
}

impl InkGetNumBuilder {
	pub fn new(client: Arc<FullClient>, contract_addr: String) -> Self {
		Self { client, contract_addr }
	}
}

impl frame_benchmarking_cli::ExtrinsicBuilder for InkGetNumBuilder {
	fn pallet(&self) -> &str {
		"contract-test"
	}

	fn extrinsic(&self) -> &str {
		"get_num"
	}

	fn build(&self, nonce: u32) -> std::result::Result<OpaqueExtrinsic, &'static str> {
		let mut call_data: Vec<u8> = Vec::new();
		// `msg_selector` identifies the message to be invoked
		let mut msg_selector: Vec<u8> = [0xCF, 0xE3, 0x9F, 0xC5].into();
		// construct a call to a specific message with the arguments
		call_data.append(&mut msg_selector);
		let acc = Sr25519Keyring::Bob.pair();
		let extrinsic: OpaqueExtrinsic = create_benchmark_extrinsic(
			self.client.as_ref(),
			acc,
			ContractsCall::call {
				 dest: Address::Address32(*AccountId32::from_str(&*self.contract_addr).unwrap().as_ref()).into(),
				 value: Default::default(),
				 gas_limit: Default::default(),
				 storage_deposit_limit: Default::default(),
				 data: call_data.clone()
			}.into(),
			nonce,
		)
		.into();

		Ok(extrinsic)
	}
}

/// Generates `Contracts::Call` extrinsics that represents calling the `test` contracts `update_s` message/function for the benchmarks.
///
/// Note: Should only be used for benchmarking.
/// generates a random alphanumeric string and passes it as an argument.
pub struct InkUpdateSBuilder {
	client: Arc<FullClient>,
	contract_addr: String,
}

impl InkUpdateSBuilder {
	pub fn new(client: Arc<FullClient>, contract_addr: String) -> Self {
		Self { client, contract_addr }
	}
}

impl frame_benchmarking_cli::ExtrinsicBuilder for InkUpdateSBuilder {
	fn pallet(&self) -> &str {
		"contract-test"
	}

	fn extrinsic(&self) -> &str {
		"update_s"
	}

	fn build(&self, nonce: u32) -> std::result::Result<OpaqueExtrinsic, &'static str> {
		let mut call_data: Vec<u8> = Vec::new();
		// `msg_selector` identifies the message to be invoked
		let mut msg_selector: Vec<u8> = [0x90, 0xC9, 0xB3, 0xF8].into();
		// `msg_args` are the SCALE-encoded args to be passed to the above message
		let mut msg_args = generate_random_string().encode();
		// construct a call to a specific message with the arguments
		call_data.append(&mut msg_selector);
		call_data.append(&mut msg_args);
		let acc = Sr25519Keyring::Bob.pair();
		let extrinsic: OpaqueExtrinsic = create_benchmark_extrinsic(
			self.client.as_ref(),
			acc,
			ContractsCall::call {
				 dest: Address::Address32(*AccountId32::from_str(&*self.contract_addr).unwrap().as_ref()).into(),
				 value: Default::default(),
				 gas_limit: Default::default(),
				 storage_deposit_limit: Default::default(),
				 data: call_data.clone()
			}.into(),
			nonce,
		)
		.into();

		Ok(extrinsic)
	}
}

/// Generates `Contracts::Call` extrinsics that represents calling the `test` contracts `get_s` message/function for the benchmarks.
///
/// Note: Should only be used for benchmarking.
pub struct InkGetSBuilder {
	client: Arc<FullClient>,
	contract_addr: String,
}

impl InkGetSBuilder {
	pub fn new(client: Arc<FullClient>, contract_addr: String) -> Self {
		Self { client, contract_addr }
	}
}

impl frame_benchmarking_cli::ExtrinsicBuilder for InkGetSBuilder {
	fn pallet(&self) -> &str {
		"contract-test"
	}

	fn extrinsic(&self) -> &str {
		"get_s"
	}

	fn build(&self, nonce: u32) -> std::result::Result<OpaqueExtrinsic, &'static str> {
		let mut call_data: Vec<u8> = Vec::new();
		// `msg_selector` identifies the message to be invoked
		let mut msg_selector: Vec<u8> = [0xA9, 0xE8, 0x9D, 0x26].into();
		// construct a call to a specific message with the arguments
		call_data.append(&mut msg_selector);
		let acc = Sr25519Keyring::Bob.pair();
		let extrinsic: OpaqueExtrinsic = create_benchmark_extrinsic(
			self.client.as_ref(),
			acc,
			ContractsCall::call {
				 dest: Address::Address32(*AccountId32::from_str(&*self.contract_addr).unwrap().as_ref()).into(),
				 value: Default::default(),
				 gas_limit: Default::default(),
				 storage_deposit_limit: Default::default(),
				 data: call_data.clone()
			}.into(),
			nonce,
		)
		.into();

		Ok(extrinsic)
	}
}

pub struct InkFibonacciBuilder {
	client: Arc<FullClient>,
	contract_addr: String,
}

impl InkFibonacciBuilder {
	pub fn new(client: Arc<FullClient>, contract_addr: String) -> Self {
		Self { client, contract_addr }
	}
}

impl frame_benchmarking_cli::ExtrinsicBuilder for InkFibonacciBuilder {
	fn pallet(&self) -> &str {
		"contract-test"
	}

	fn extrinsic(&self) -> &str {
		"fibonacci"
	}

	fn build(&self, nonce: u32) -> std::result::Result<OpaqueExtrinsic, &'static str> {
		let mut call_data: Vec<u8> = Vec::new();
		// `msg_selector` identifies the message to be invoked
		let mut msg_selector: Vec<u8> = [0x3C, 0x87, 0x40, 0x79].into();
		// `msg_args` are the SCALE-encoded args to be passed to the above message
		let mut msg_args = 15u32.encode();
		// construct a call to a specific message with the arguments
		call_data.append(&mut msg_selector);
		call_data.append(&mut msg_args);
		let acc = Sr25519Keyring::Bob.pair();
		let extrinsic: OpaqueExtrinsic = create_benchmark_extrinsic(
			self.client.as_ref(),
			acc,
			ContractsCall::call {
				 dest: Address::Address32(*AccountId32::from_str(&*self.contract_addr).unwrap().as_ref()).into(),
				 value: Default::default(),
				 gas_limit: Default::default(),
				 storage_deposit_limit: Default::default(),
				 data: call_data.clone()
			}.into(),
			nonce,
		)
		.into();

		Ok(extrinsic)
	}
}

pub struct InkOddProductBuilder {
	client: Arc<FullClient>,
	contract_addr: String,
}

impl InkOddProductBuilder {
	pub fn new(client: Arc<FullClient>, contract_addr: String) -> Self {
		Self { client, contract_addr }
	}
}

impl frame_benchmarking_cli::ExtrinsicBuilder for InkOddProductBuilder {
	fn pallet(&self) -> &str {
		"contract-test"
	}

	fn extrinsic(&self) -> &str {
		"odd_product"
	}

	fn build(&self, nonce: u32) -> std::result::Result<OpaqueExtrinsic, &'static str> {
		let mut call_data: Vec<u8> = Vec::new();
		// `msg_selector` identifies the message to be invoked
		let mut msg_selector: Vec<u8> = [0x2A, 0xC6, 0x52, 0x85].into();
		// `msg_args` are the SCALE-encoded args to be passed to the above message
		let mut msg_args = 15u32.encode();
		// construct a call to a specific message with the arguments
		call_data.append(&mut msg_selector);
		call_data.append(&mut msg_args);
		let acc = Sr25519Keyring::Bob.pair();
		let extrinsic: OpaqueExtrinsic = create_benchmark_extrinsic(
			self.client.as_ref(),
			acc,
			ContractsCall::call {
				 dest: Address::Address32(*AccountId32::from_str(&*self.contract_addr).unwrap().as_ref()).into(),
				 value: Default::default(),
				 gas_limit: Default::default(),
				 storage_deposit_limit: Default::default(),
				 data: call_data.clone()
			}.into(),
			nonce,
		)
		.into();

		Ok(extrinsic)
	}
}

pub struct InkTriangleNumberBuilder {
	client: Arc<FullClient>,
	contract_addr: String,
}

impl InkTriangleNumberBuilder {
	pub fn new(client: Arc<FullClient>, contract_addr: String) -> Self {
		Self { client, contract_addr }
	}
}

impl frame_benchmarking_cli::ExtrinsicBuilder for InkTriangleNumberBuilder {
	fn pallet(&self) -> &str {
		"contract-test"
	}

	fn extrinsic(&self) -> &str {
		"triangle_number"
	}

	fn build(&self, nonce: u32) -> std::result::Result<OpaqueExtrinsic, &'static str> {
		let mut call_data: Vec<u8> = Vec::new();
		// `msg_selector` identifies the message to be invoked
		let mut msg_selector: Vec<u8> = [0x8E, 0x70, 0x88, 0x51].into();
		// `msg_args` are the SCALE-encoded args to be passed to the above message
		let mut msg_args = 15u32.encode();
		// construct a call to a specific message with the arguments
		call_data.append(&mut msg_selector);
		call_data.append(&mut msg_args);
		let acc = Sr25519Keyring::Bob.pair();
		let extrinsic: OpaqueExtrinsic = create_benchmark_extrinsic(
			self.client.as_ref(),
			acc,
			ContractsCall::call {
				 dest: Address::Address32(*AccountId32::from_str(&*self.contract_addr).unwrap().as_ref()).into(),
				 value: Default::default(),
				 gas_limit: Default::default(),
				 storage_deposit_limit: Default::default(),
				 data: call_data.clone()
			}.into(),
			nonce,
		)
		.into();

		Ok(extrinsic)
	}
}

pub struct InkSampleEmitBuilder {
	client: Arc<FullClient>,
	contract_addr: String,
}

impl InkSampleEmitBuilder {
	pub fn new(client: Arc<FullClient>, contract_addr: String) -> Self {
		Self { client, contract_addr }
	}
}

impl frame_benchmarking_cli::ExtrinsicBuilder for InkSampleEmitBuilder {
	fn pallet(&self) -> &str {
		"contract-test"
	}

	fn extrinsic(&self) -> &str {
		"emit_sample_event"
	}

	fn build(&self, nonce: u32) -> std::result::Result<OpaqueExtrinsic, &'static str> {
		let mut call_data: Vec<u8> = Vec::new();
		// `msg_selector` identifies the message to be invoked
		let mut msg_selector: Vec<u8> = [0x43, 0x5A, 0x64, 0x4F].into();
		// `msg_args` are the SCALE-encoded args to be passed to the above message
		// let mut msg_args = 15u32.encode();
		// construct a call to a specific message with the arguments
		call_data.append(&mut msg_selector);
		// call_data.append(&mut msg_args);
		let acc = Sr25519Keyring::Bob.pair();
		let extrinsic: OpaqueExtrinsic = create_benchmark_extrinsic(
			self.client.as_ref(),
			acc,
			ContractsCall::call {
				 dest: Address::Address32(*AccountId32::from_str(&*self.contract_addr).unwrap().as_ref()).into(),
				 value: Default::default(),
				 gas_limit: Default::default(),
				 storage_deposit_limit: Default::default(),
				 data: call_data.clone()
			}.into(),
			nonce,
		)
		.into();

		Ok(extrinsic)
	}
}

pub struct InkCrossContractCallBuilder {
	client: Arc<FullClient>,
	contract_addr: String,
	callee_contract_addr: String,
}

impl InkCrossContractCallBuilder {
	pub fn new(client: Arc<FullClient>, contract_addr: String, callee_contract_addr: String) -> Self {
		Self { client, contract_addr, callee_contract_addr }
	}
}

impl frame_benchmarking_cli::ExtrinsicBuilder for InkCrossContractCallBuilder {
	fn pallet(&self) -> &str {
		"contract-test"
	}

	fn extrinsic(&self) -> &str {
		"cross_contract_call"
	}

	fn build(&self, nonce: u32) -> std::result::Result<OpaqueExtrinsic, &'static str> {
		let mut call_data: Vec<u8> = Vec::new();
		// `msg_selector` identifies the message to be invoked
		let mut msg_selector: Vec<u8> = [0x5A, 0xCC, 0xC8, 0x95].into();
		// `msg_args` are the SCALE-encoded args to be passed to the above message
		let cntrct_addr: [u8; 32] = (AccountId32::from_str(&*self.callee_contract_addr).unwrap()).into();
		let mut cntrct_addr_encoded = cntrct_addr.encode();
		let mut new_value_encoded = 100i64.encode();
		// construct a call to a specific message with the arguments
		call_data.append(&mut msg_selector);
		call_data.append(&mut cntrct_addr_encoded);
		call_data.append(&mut new_value_encoded);
		let acc = Sr25519Keyring::Bob.pair();
		let extrinsic: OpaqueExtrinsic = create_benchmark_extrinsic(
			self.client.as_ref(),
			acc,
			ContractsCall::call {
				 dest: Address::Address32(*AccountId32::from_str(&*self.contract_addr).unwrap().as_ref()).into(),
				 value: Default::default(),
				 gas_limit: Default::default(),
				 storage_deposit_limit: Default::default(),
				 data: call_data.clone()
			}.into(),
			nonce,
		)
		.into();

		Ok(extrinsic)
	}
}

pub struct SoliditySetSomeNumBuilder {
	client: Arc<FullClient>,
	contract_addr: String,
}

impl SoliditySetSomeNumBuilder {
	pub fn new(client: Arc<FullClient>, contract_addr: String) -> Self {
		Self {client, contract_addr}
	}
}

impl frame_benchmarking_cli::ExtrinsicBuilder for SoliditySetSomeNumBuilder {
	fn pallet(&self) -> &str {
		"solidity_wasm"
	}

	fn extrinsic(&self) -> &str {
		"set_some_num"
	}

	fn build(&self, nonce: u32) -> std::result::Result<OpaqueExtrinsic, &'static str> {
		let call_data_str = String::from("9ac1762f00000000000000000000000000000000000000000000000000000000000002d9");
		let call_data: Vec<u8> = hex_string_to_bytes(&call_data_str).unwrap();
		let acc = Sr25519Keyring::Bob.pair();
		let extrinsic: OpaqueExtrinsic = create_benchmark_extrinsic(
			self.client.as_ref(),
			acc,
			ContractsCall::call {
				 dest: Address::Address32(*AccountId32::from_str(&*self.contract_addr).unwrap().as_ref()).into(),
				 value: Default::default(),
				 gas_limit: Default::default(),
				 storage_deposit_limit: Default::default(),
				 data: call_data.clone()
			}.into(),
			nonce,
		)
		.into();

		Ok(extrinsic)
	}
}

pub struct SolidityGetSomeNumBuilder {
	client: Arc<FullClient>,
	contract_addr: String,
}

impl SolidityGetSomeNumBuilder {
	pub fn new(client: Arc<FullClient>, contract_addr: String) -> Self {
		Self {client, contract_addr}
	}
}

impl frame_benchmarking_cli::ExtrinsicBuilder for SolidityGetSomeNumBuilder {
	fn pallet(&self) -> &str {
		"solidity_wasm"
	}

	fn extrinsic(&self) -> &str {
		"get_some_num"
	}

	fn build(&self, nonce: u32) -> std::result::Result<OpaqueExtrinsic, &'static str> {
		let call_data_str = String::from("b40a136b");
		let call_data: Vec<u8> = hex_string_to_bytes(&call_data_str).unwrap();
		let acc = Sr25519Keyring::Bob.pair();
		let extrinsic: OpaqueExtrinsic = create_benchmark_extrinsic(
			self.client.as_ref(),
			acc,
			ContractsCall::call {
				 dest: Address::Address32(*AccountId32::from_str(&*self.contract_addr).unwrap().as_ref()).into(),
				 value: Default::default(),
				 gas_limit: Default::default(),
				 storage_deposit_limit: Default::default(),
				 data: call_data.clone()
			}.into(),
			nonce,
		)
		.into();

		Ok(extrinsic)
	}
}

pub struct SoliditySetSomeStrBuilder {
	client: Arc<FullClient>,
	contract_addr: String,
}

impl SoliditySetSomeStrBuilder {
	pub fn new(client: Arc<FullClient>, contract_addr: String) -> Self {
		Self {client, contract_addr}
	}
}

impl frame_benchmarking_cli::ExtrinsicBuilder for SoliditySetSomeStrBuilder {
	fn pallet(&self) -> &str {
		"solidity_wasm"
	}

	fn extrinsic(&self) -> &str {
		"set_some_str"
	}

	fn build(&self, nonce: u32) -> std::result::Result<OpaqueExtrinsic, &'static str> {
		let call_data_str = String::from("0583a02c0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000001a536f6c6964697479207761736d206f6e20737562737472617465000000000000");
		let call_data: Vec<u8> = hex_string_to_bytes(&call_data_str).unwrap();
		let acc = Sr25519Keyring::Bob.pair();
		let extrinsic: OpaqueExtrinsic = create_benchmark_extrinsic(
			self.client.as_ref(),
			acc,
			ContractsCall::call {
				 dest: Address::Address32(*AccountId32::from_str(&*self.contract_addr).unwrap().as_ref()).into(),
				 value: Default::default(),
				 gas_limit: Default::default(),
				 storage_deposit_limit: Default::default(),
				 data: call_data.clone()
			}.into(),
			nonce,
		)
		.into();

		Ok(extrinsic)
	}
}

pub struct SolidityGetSomeStrBuilder {
	client: Arc<FullClient>,
	contract_addr: String,
}

impl SolidityGetSomeStrBuilder {
	pub fn new(client: Arc<FullClient>, contract_addr: String) -> Self {
		Self {client, contract_addr}
	}
}

impl frame_benchmarking_cli::ExtrinsicBuilder for SolidityGetSomeStrBuilder {
	fn pallet(&self) -> &str {
		"solidity_wasm"
	}

	fn extrinsic(&self) -> &str {
		"get_some_str"
	}

	fn build(&self, nonce: u32) -> std::result::Result<OpaqueExtrinsic, &'static str> {
		let call_data_str = String::from("5e9c155e");
		let call_data: Vec<u8> = hex_string_to_bytes(&call_data_str).unwrap();
		let acc = Sr25519Keyring::Bob.pair();
		let extrinsic: OpaqueExtrinsic = create_benchmark_extrinsic(
			self.client.as_ref(),
			acc,
			ContractsCall::call {
				 dest: Address::Address32(*AccountId32::from_str(&*self.contract_addr).unwrap().as_ref()).into(),
				 value: Default::default(),
				 gas_limit: Default::default(),
				 storage_deposit_limit: Default::default(),
				 data: call_data.clone()
			}.into(),
			nonce,
		)
		.into();

		Ok(extrinsic)
	}
}

pub struct SolidityFibonacciBuilder {
	client: Arc<FullClient>,
	contract_addr: String,
}

impl SolidityFibonacciBuilder {
	pub fn new(client: Arc<FullClient>, contract_addr: String) -> Self {
		Self {client, contract_addr}
	}
}

impl frame_benchmarking_cli::ExtrinsicBuilder for SolidityFibonacciBuilder {
	fn pallet(&self) -> &str {
		"solidity_wasm"
	}

	fn extrinsic(&self) -> &str {
		"fibonacci"
	}

	fn build(&self, nonce: u32) -> std::result::Result<OpaqueExtrinsic, &'static str> {
		let call_data_str = String::from("61047ff4000000000000000000000000000000000000000000000000000000000000000f");
		let call_data: Vec<u8> = hex_string_to_bytes(&call_data_str).unwrap();
		let acc = Sr25519Keyring::Bob.pair();
		let extrinsic: OpaqueExtrinsic = create_benchmark_extrinsic(
			self.client.as_ref(),
			acc,
			ContractsCall::call {
				 dest: Address::Address32(*AccountId32::from_str(&*self.contract_addr).unwrap().as_ref()).into(),
				 value: Default::default(),
				 gas_limit: Default::default(),
				 storage_deposit_limit: Default::default(),
				 data: call_data.clone()
			}.into(),
			nonce,
		)
		.into();

		Ok(extrinsic)
	}
}

pub struct SolidityOddProductBuilder {
	client: Arc<FullClient>,
	contract_addr: String,
}

impl SolidityOddProductBuilder {
	pub fn new(client: Arc<FullClient>, contract_addr: String) -> Self {
		Self {client, contract_addr}
	}
}

impl frame_benchmarking_cli::ExtrinsicBuilder for SolidityOddProductBuilder {
	fn pallet(&self) -> &str {
		"solidity_wasm"
	}

	fn extrinsic(&self) -> &str {
		"odd_product"
	}

	fn build(&self, nonce: u32) -> std::result::Result<OpaqueExtrinsic, &'static str> {
		let call_data_str = String::from("113d5646000000000000000000000000000000000000000000000000000000000000000f");
		let call_data: Vec<u8> = hex_string_to_bytes(&call_data_str).unwrap();
		let acc = Sr25519Keyring::Bob.pair();
		let extrinsic: OpaqueExtrinsic = create_benchmark_extrinsic(
			self.client.as_ref(),
			acc,
			ContractsCall::call {
				 dest: Address::Address32(*AccountId32::from_str(&*self.contract_addr).unwrap().as_ref()).into(),
				 value: Default::default(),
				 gas_limit: Default::default(),
				 storage_deposit_limit: Default::default(),
				 data: call_data.clone()
			}.into(),
			nonce,
		)
		.into();

		Ok(extrinsic)
	}
}

pub struct SolidityTriangleNumBuilder {
	client: Arc<FullClient>,
	contract_addr: String,
}

impl SolidityTriangleNumBuilder {
	pub fn new(client: Arc<FullClient>, contract_addr: String) -> Self {
		Self {client, contract_addr}
	}
}

impl frame_benchmarking_cli::ExtrinsicBuilder for SolidityTriangleNumBuilder {
	fn pallet(&self) -> &str {
		"solidity_wasm"
	}

	fn extrinsic(&self) -> &str {
		"triangle_number"
	}

	fn build(&self, nonce: u32) -> std::result::Result<OpaqueExtrinsic, &'static str> {
		let call_data_str = String::from("8187c52b000000000000000000000000000000000000000000000000000000000000000f");
		let call_data: Vec<u8> = hex_string_to_bytes(&call_data_str).unwrap();
		let acc = Sr25519Keyring::Bob.pair();
		let extrinsic: OpaqueExtrinsic = create_benchmark_extrinsic(
			self.client.as_ref(),
			acc,
			ContractsCall::call {
				 dest: Address::Address32(*AccountId32::from_str(&*self.contract_addr).unwrap().as_ref()).into(),
				 value: Default::default(),
				 gas_limit: Default::default(),
				 storage_deposit_limit: Default::default(),
				 data: call_data.clone()
			}.into(),
			nonce,
		)
		.into();

		Ok(extrinsic)
	}
}

pub struct SoliditySampleEventBuilder {
	client: Arc<FullClient>,
	contract_addr: String,
}

impl SoliditySampleEventBuilder {
	pub fn new(client: Arc<FullClient>, contract_addr: String) -> Self {
		Self {client, contract_addr}
	}
}

impl frame_benchmarking_cli::ExtrinsicBuilder for SoliditySampleEventBuilder {
	fn pallet(&self) -> &str {
		"solidity_wasm"
	}

	fn extrinsic(&self) -> &str {
		"emit_sample_event"
	}

	fn build(&self, nonce: u32) -> std::result::Result<OpaqueExtrinsic, &'static str> {
		let call_data_str = String::from("93063bb1");
		let call_data: Vec<u8> = hex_string_to_bytes(&call_data_str).unwrap();
		let acc = Sr25519Keyring::Bob.pair();
		let extrinsic: OpaqueExtrinsic = create_benchmark_extrinsic(
			self.client.as_ref(),
			acc,
			ContractsCall::call {
				 dest: Address::Address32(*AccountId32::from_str(&*self.contract_addr).unwrap().as_ref()).into(),
				 value: Default::default(),
				 gas_limit: Default::default(),
				 storage_deposit_limit: Default::default(),
				 data: call_data.clone()
			}.into(),
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
	client: &FullClient,
	sender: sp_core::sr25519::Pair,
	call: runtime::RuntimeCall,
	nonce: u32,
) -> runtime::UncheckedExtrinsic {
	let genesis_hash = client.block_hash(0).ok().flatten().expect("Genesis block exists; qed");
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
		frame_system::CheckEra::<runtime::Runtime>::from(sp_runtime::generic::Era::mortal(
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
		call.clone(),
		sp_runtime::AccountId32::from(sender.public()).into(),
		runtime::Signature::Sr25519(signature.clone()),
		extra.clone(),
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
