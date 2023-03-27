# Frontier Node Template for Benchmarking
A FRAME-based [Frontier](https://github.com/paritytech/frontier/tree/master) node, with emphasis on and support for benchmarking native solidity smartcontracts.

[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/paritytech/frontier/rust.yml)](https://github.com/paritytech/frontier/actions)
[![Matrix](https://img.shields.io/matrix/frontier:matrix.org)](https://matrix.to/#/#frontier:matrix.org)

Frontier is Substrate's Ethereum compatibility layer. It allows you to run
unmodified Ethereum dapps.

The goal of Ethereum compatibility layer is to be able to:

* Run a normal web3 application via the compatibility layer, using local nodes,
  where an extra bridge binary is acceptable.
* Be able to import state from Ethereum mainnet.

## Releases

### Primitives

Those are suitable to be included in a runtime. Primitives are structures shared
by higher-level code.

* `fp-consensus`: Consensus layer primitives.
  ![Crates.io](https://img.shields.io/crates/v/fp-consensus)
* `fp-evm`: EVM primitives. ![Crates.io](https://img.shields.io/crates/v/fp-evm)
* `fp-rpc`: RPC primitives. ![Crates.io](https://img.shields.io/crates/v/fp-rpc)
* `fp-storage`: Well-known storage information.
  ![Crates.io](https://img.shields.io/crates/v/fp-storage)

### Pallets

Those pallets serve as runtime components for projects using Frontier.

* `pallet-evm`: EVM execution handling.
  ![Crates.io](https://img.shields.io/crates/v/pallet-evm)
* `pallet-ethereum`: Ethereum block handling.
  ![Crates.io](https://img.shields.io/crates/v/pallet-ethereum)
* `pallet-dynamic-fee`: Extends the fee handling logic so that it can be changed
  within the runtime.
  ![Crates.io](https://img.shields.io/crates/v/pallet-dynamic-fee)

### EVM Pallet precompiles

Those precompiles can be used together with `pallet-evm` for additional
functionalities of the EVM executor.

* `pallet-evm-precompile-simple`: Four basic precompiles in Ethereum EVMs.
  ![Crates.io](https://img.shields.io/crates/v/pallet-evm-precompile-simple)
* `pallet-evm-precompile-blake2`: BLAKE2 precompile.
  ![Crates.io](https://img.shields.io/crates/v/pallet-evm-precompile-blake2)
* `pallet-evm-precompile-bn128`: BN128 precompile.
  ![Crates.io](https://img.shields.io/crates/v/pallet-evm-precompile-bn128)
* `pallet-evm-precompile-ed25519`: ED25519 precompile.
  ![Crates.io](https://img.shields.io/crates/v/pallet-evm-precompile-ed25519)
* `pallet-evm-precompile-modexp`: MODEXP precompile.
  ![Crates.io](https://img.shields.io/crates/v/pallet-evm-precompile-modexp)
* `pallet-evm-precompile-sha3fips`: Standard SHA3 precompile.
  ![Crates.io](https://img.shields.io/crates/v/pallet-evm-precompile-sha3fips)
* `pallet-evm-precompile-dispatch`: Enable interoperability between EVM
  contracts and other Substrate runtime components.
  ![Crates.io](https://img.shields.io/crates/v/pallet-evm-precompile-dispatch)

### Client-side libraries

Those are libraries that should be used on client-side to enable RPC, block hash
mapping, and other features.

* `fc-consensus`: Consensus block import.
  ![Crates.io](https://img.shields.io/crates/v/fc-consensus)
* `fc-db`: Frontier-specific database backend.
  ![Crates.io](https://img.shields.io/crates/v/fc-db)
* `fc-mapping-sync`: Block hash mapping syncing logic.
  ![Crates.io](https://img.shields.io/crates/v/fc-mapping-sync)
* `fc-rpc-core`: Core RPC logic.
  ![Crates.io](https://img.shields.io/crates/v/fc-rpc-core)
* `fc-rpc`: RPC implementation.
  ![Crates.io](https://img.shields.io/crates/v/fc-rpc)

## Development workflow

### Pull request

All changes (except new releases) are handled through pull requests.

### Versioning

Frontier follows [Semantic Versioning](https://semver.org/). An unreleased crate
in the repository will have the `-dev` suffix in the end, and we do rolling
releases.

When you make a pull request against this repository, please also update the
affected crates' versions, using the following rules. Note that the rules should
be applied recursively -- if a change modifies any upper crate's dependency
(even just the `Cargo.toml` file), then the upper crate will also need to apply
those rules.

Additionally, if your change is notable, then you should also modify the
corresponding `CHANGELOG.md` file, in the "Unreleased" section.

If the affected crate already has `-dev` suffix:

* If your change is a patch, then you do not have to update any versions.
* If your change introduces a new feature, please check if the local version
  already had its minor version bumped, if not, bump it.
* If your change modifies the current interface, please check if the local
  version already had its major version bumped, if not, bump it.

If the affected crate does not yet have `-dev` suffix:

* If your change is a patch, then bump the patch version, and add `-dev` suffix.
* If your change introduces a new feature, then bump the minor version, and add
  `-dev` suffix.
* If your change modifies the current interface, then bump the major version,
  and add `-dev` suffix.

If your pull request introduces a new crate, please set its version to
`1.0.0-dev`.

### Native Solidity smartcontracts benchmarking

This node supports native solidity smartcontracts benchmarking by leveraging [frame-benchmarking-cli](https://github.com/paritytech/substrate/tree/master/utils/frame/benchmarking-cli).
The above tool creates an empty block using sc-block-builder and then populates it with as many instances of an extrinsic as possible and runs it several times and returns the block execution time stats.
By default, `frame-benchmarking-cli` only supports benchmarking two hardcoded pallet extrinsics, but has been extended to support the exposed functions in the custom smart contract that comes with this repo.

This node's runtime is composed of many pallets and one of them, [`pallet-evm`](https://paritytech.github.io/frontier/rustdocs/pallet_evm/index.html) is capable of executing unmodified EVM code in a Substrate-based blockchain.
`pallet-evm` exposes a few extrinsics:

- `create`, using which you can deploy EVM bytecode, thereby, creating a contract,
- `call`, using which you can invoke a public function within a deployed contract,
- and others too.
since, `call` is the one that invokes a public function, we will be extending `frame-benchmarking-cli` to support benchmarking this extrinsic and in doing so, we will have the ability to benchmark any call to a EVM smartcontract.
`call` takes 9 arguments,
- `source`, an Ethereum-like addressing scheme that represents account that's sending the transaction,
- `target`, an Ethereum-like addressing scheme that represents the EVM smartcontract to send the transaction to,
- `input`, Ethereum-like encoding scheme that encodes the smartcontract function to call and the arguments to be passed in,
- `value`, the amount of funds to transfer along with this transaction,
- `gas_limit`, the max amount of gas this transaction can consume,
- `max_fee_per_gas`, the max fee per unit of gas,
- `nonce`, the number of transactions made from `source` so far, starting at 0.
- and others.

So, an extrinsic that calls a public function of a EVM smartcontract is actually represented by a `pallet-evm::call` extrinsic with the `source` argument being the Ethereum-like address invoking the smartcontract, `target` argument being the Ethereum-like address of the EVM smartcontract to be invoked, `input` argument being a byte vector that represents Ethereum-like encoding that includes the public function's selector and the arguments to be passed to that public function in the exact order.
`nonce` should start from 1 this time, because we would have already sent a transaction wherein we would have deployed the EVM contract by calling `pallet-evm::create`.
Note: you can configure `gas_limit` and `max_fee_per_gas` as per your needs. The provided values for these make for a good default.

Note: make sure `gas_limit` is below the `BLOCK_GAS_LIMIT` as set in the [`runtime`](./template/runtime/src/lib.rs).
It is currently set to a value of `7500000000` using the following line,
```
const BLOCK_GAS_LIMIT: u64 = 7500000000;
```

As for the rest of the arguments, `Default::default()` makes sense.

The following steps show you how to extend it for your own EVM based smartcontracts' functions:

- Develop a smartcontract in Solidity, Vyper or any other language that can be compiled to EVM bytecode.

- Compile it and copy the outputted EVM bytecode.

- Start the local node, refer to `Connect to the node` section of this [`article`](https://docs.substrate.io/tutorials/integrate-with-tools/access-evm-accounts/) within the official Substrate docs.

- Deploy this EVM based smartcontract by following the steps mentioned in the `Deploy a smart contract` section within the same `article` as above.

- Verify the deployment by following the steps mentioned in the `View the smart contract` section within the same `article` as above.

- Understand how a function is called by going through the `Transfer tokens` section within the same `article` as above.

- We need to supply the following arguments at the very least to call an EVM function, `source`, `target`, `value`, `input`, `gas_limit` and `max_fee_per_gas`. We have already described what each of these stand for.

- frame-benchmarking-cli expects us to implement the `frame_benchmarking_cli::ExtrinsicBuilder` trait for each extrinsic that we want to benchmark. Let's extend the frame-benchmarking-cli to support your EVM smartcontracts' public function by following the steps below:

 - Within [`benchmarking.rs`](.template/node/src/benchmarking.rs), create a struct whose fields are a client, EVM-like address of the source and the deployed EVM smartcontract instance, input to be provided that encodes the function to call and the arguments to be supplied and other args. Implement the `frame_benchmarking_cli::ExtrinsicBuilder` trait on this struct.

 - Remember to implement rest of the functions that comprise the aforementioned trait.

 - Within [`command.rs`](./node/src/command.rs), inside the `BenchmarkCmd::Extrinsic(cmd)` code block and within the `ext_factory` vec, construct a new box with an instance of the above defined struct.

 - Alternatively, you can leverage the provided struct `NativeSolidityGenericCallBuilder` that can represent any call to an EVM smartcontract. All you need to do is store the source address in source_address.txt, contract address in contract_address.txt and the call-data/input in call_data.txt. This call-data/input can be easily obtained by simulating the same call with the same args in [Remix Web IDE](https://remix.ethereum.org/) as already mentioned in the above `article`.

- benchmark your smartcontracts' function using the following command, `./target/release/node-template benchmark extrinsic --pallet <pallet-name> --extrinsic <extrinsic-name>`.
`native_solidity` can be used for `<pallet-name>` and `generic_call` can be used for `<extrinic-name>` in case you choose to use `NativeSolidityGenericCallBuilder` rather than develop your own struct that implements the aforementioned trait.
This command outputs the block execution time(in nanoseconds) stats and also the number of extrinsic instances included in a block. Dividing the average block execution time by number of extrinsics per block gives you the average time taken(in nanoseconds) to execute an extrinisic.

Note:
A [`sample solidity smartcontract`](./template/benchmark-sample/contracts/benchmark-sample.sol) and it's [`bytecode`](./template/benchmark-sample/build/contracts/BenchmarkSample.json) and the required trait implementation to make it work with the frame-benchmarking-cli are included for your reference.

### Benchmark

Use the following command to list out all the extrinsics you can benchmark:
```sh
./target/release/frontier-template-node benchmark extrinsic --list
```

and use the following command to benchmark a pallet extrinsic, or a smartcontract's public function:
```sh
./target/release/frontier-template-node benchmark extrinsic --pallet <name of smartcontract as per the trait> --extrinsic <name of the smartcontract-function as per the trait>
```