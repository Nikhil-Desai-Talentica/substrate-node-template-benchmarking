# Substrate Node Template For Benchmarking

A FRAME-based [Substrate](https://www.substrate.io/) node, with emphasis on and support for benchmarking custom pallets and smart contracts.

Includes a custom pallet and contracts pallet along with their benchmarking code and a guide to help you benchmark your own pallets and smart contracts.

## Getting Started

Follow the steps below to get started with the Node Template

### Rust Setup

First, complete the [basic Rust setup instructions](./docs/rust-setup.md).
Note: Rust nightly 1.68 is required to build the template. Some of the dependencies report issues when you attempt to compile them using Rust nightly 1.70. Other users have also experienced the same [issue](https://substrate.stackexchange.com/questions/7714/cannot-run-substrate-on-a-fresh-macbook-m2).

### Run

Use Rust's native `cargo` command to build and launch the template node:

```sh
cargo run --release -- --dev
```

### Build

The `cargo run` command will perform an initial build. Use the following command to build the node
without launching it:

```sh
cargo build --release
```

### Benchmark

Use the following command to list out all the extrinsics you can benchmark:
```sh
./target/release/node-template benchmark extrinsic --list
```

and use the following command to benchmark a pallet extrinsic, or a smartcontract's public function:
```sh
./target/release/node-template benchmark extrinsic --pallet <name of pallet/smartcontract as per the trait> --extrinsic <name of the extrinsic/function as per the trait>
```
### Embedded Docs

Once the project has been built, the following command can be used to explore all parameters and
subcommands:

```sh
./target/release/node-template -h
```

## Run

The provided `cargo run` command will launch a temporary node and its state will be discarded after
you terminate the process. After the project has been built, there are other ways to launch the
node.

### Single-Node Development Chain

This command will start the single-node development chain with non-persistent state:

```bash
./target/release/node-template --dev
```

Purge the development chain's state:

```bash
./target/release/node-template purge-chain --dev
```

Start the development chain with detailed logging:

```bash
RUST_BACKTRACE=1 ./target/release/node-template -ldebug --dev
```

> Development chain means that the state of our chain will be in a tmp folder while the nodes are
> running. Also, **alice** account will be authority and sudo account as declared in the
> [genesis state](https://github.com/substrate-developer-hub/substrate-node-template/blob/main/node/src/chain_spec.rs#L49).
> At the same time the following accounts will be pre-funded:
> - Alice
> - Bob
> - Alice//stash
> - Bob//stash

In case of being interested in maintaining the chain' state between runs a base path must be added
so the db can be stored in the provided folder instead of a temporal one. We could use this folder
to store different chain databases, as a different folder will be created per different chain that
is ran. The following commands shows how to use a newly created folder as our db base path.

```bash
// Create a folder to use as the db base path
$ mkdir my-chain-state

// Use of that folder to store the chain state
$ ./target/release/node-template --dev --base-path ./my-chain-state/

// Check the folder structure created inside the base path after running the chain
$ ls ./my-chain-state
chains
$ ls ./my-chain-state/chains/
dev
$ ls ./my-chain-state/chains/dev
db keystore network
```


### Connect with Polkadot-JS Apps Front-end

Once the node template is running locally, you can connect it with **Polkadot-JS Apps** front-end
to interact with your chain. [Click
here](https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944) connecting the Apps to your
local node template.

### Multi-Node Local Testnet

If you want to see the multi-node consensus algorithm in action, refer to our
[Simulate a network tutorial](https://docs.substrate.io/tutorials/get-started/simulate-network/).

## Template Structure

A Substrate project such as this consists of a number of components that are spread across a few
directories.

### Node

A blockchain node is an application that allows users to participate in a blockchain network.
Substrate-based blockchain nodes expose a number of capabilities:

- Networking: Substrate nodes use the [`libp2p`](https://libp2p.io/) networking stack to allow the
  nodes in the network to communicate with one another.
- Consensus: Blockchains must have a way to come to
  [consensus](https://docs.substrate.io/main-docs/fundamentals/consensus/) on the state of the
  network. Substrate makes it possible to supply custom consensus engines and also ships with
  several consensus mechanisms that have been built on top of
  [Web3 Foundation research](https://research.web3.foundation/en/latest/polkadot/NPoS/index.html).
- RPC Server: A remote procedure call (RPC) server is used to interact with Substrate nodes.

There are several files in the `node` directory - take special note of the following:

- [`chain_spec.rs`](./node/src/chain_spec.rs): A
  [chain specification](https://docs.substrate.io/main-docs/build/chain-spec/) is a
  source code file that defines a Substrate chain's initial (genesis) state. Chain specifications
  are useful for development and testing, and critical when architecting the launch of a
  production chain. Take note of the `development_config` and `testnet_genesis` functions, which
  are used to define the genesis state for the local development chain configuration. These
  functions identify some
  [well-known accounts](https://docs.substrate.io/reference/command-line-tools/subkey/)
  and use them to configure the blockchain's initial state.
- [`service.rs`](./node/src/service.rs): This file defines the node implementation. Take note of
  the libraries that this file imports and the names of the functions it invokes. In particular,
  there are references to consensus-related topics, such as the
  [block finalization and forks](https://docs.substrate.io/main-docs/fundamentals/consensus/#finalization-and-forks)
  and other [consensus mechanisms](https://docs.substrate.io/main-docs/fundamentals/consensus/#default-consensus-models)
  such as Aura for block authoring and GRANDPA for finality.

After the node has been [built](#build), refer to the embedded documentation to learn more about the
capabilities and configuration parameters that it exposes:

```shell
./target/release/node-template --help
```

### Runtime

In Substrate, the terms
"runtime" and "state transition function"
are analogous - they refer to the core logic of the blockchain that is responsible for validating
blocks and executing the state changes they define. The Substrate project in this repository uses
[FRAME](https://docs.substrate.io/main-docs/fundamentals/runtime-intro/#frame) to construct a
blockchain runtime. FRAME allows runtime developers to declare domain-specific logic in modules
called "pallets". At the heart of FRAME is a helpful
[macro language](https://docs.substrate.io/reference/frame-macros/) that makes it easy to
create pallets and flexibly compose them to create blockchains that can address
[a variety of needs](https://substrate.io/ecosystem/projects/).

Review the [FRAME runtime implementation](./runtime/src/lib.rs) included in this template and note
the following:

- This file configures several pallets to include in the runtime. Each pallet configuration is
  defined by a code block that begins with `impl $PALLET_NAME::Config for Runtime`.
- The pallets are composed into a single runtime by way of the
  [`construct_runtime!`](https://crates.parity.io/frame_support/macro.construct_runtime.html)
  macro, which is part of the core
  FRAME Support [system](https://docs.substrate.io/reference/frame-pallets/#system-pallets) library.

### Pallets

The runtime in this project is constructed using many FRAME pallets that ship with the
[core Substrate repository](https://github.com/paritytech/substrate/tree/master/frame) and a
template pallet that is [defined in the `pallets`](./pallets/template/src/lib.rs) directory.

A FRAME pallet is compromised of a number of blockchain primitives:

- Storage: FRAME defines a rich set of powerful
  [storage abstractions](https://docs.substrate.io/main-docs/build/runtime-storage/) that makes
  it easy to use Substrate's efficient key-value database to manage the evolving state of a
  blockchain.
- Dispatchables: FRAME pallets define special types of functions that can be invoked (dispatched)
  from outside of the runtime in order to update its state.
- Events: Substrate uses [events and errors](https://docs.substrate.io/main-docs/build/events-errors/)
  to notify users of important changes in the runtime.
- Errors: When a dispatchable fails, it returns an error.
- Config: The `Config` configuration interface is used to define the types and parameters upon
  which a FRAME pallet depends.

### Pallet Extrinsic benchmarking

This node supports pallet extrinsic benchmarking by leveraging [frame-benchmarking-cli](https://github.com/paritytech/substrate/tree/master/utils/frame/benchmarking-cli).
The above tool creates an empty block using sc-block-builder and then populates it with as many instances of an extrinsic as possible and runs it several times and returns the block execution time stats.
By default, frame-benchmarking-cli only supports benchmarking two hardcoded pallet extrinsics, but has been extended to support the custom pallet and smart contract that come with this repo.
The following steps show you how to extend it for your own pallets' extrinsics:

- Once you've developed and published your pallet, you need to include it in the runtime. This can be done by following the steps below:

  - Within the package [`node-template-runtime`](./runtime/Cargo.toml), add your pallet as a dependency. You can see that all the other pallets that make up the runtime are dependencies too.
  - Within the same file, under the `features` section and within the `std` array, add the line `"<pallet-name>/std",`.
  - Within [`lib.rs`](./runtime/src/lib.rs), configure your pallet by implementing `<pallet-name>::Config` trait with a code block that begins with `impl <pallet-name>::Config for Runtime`.
  - Finally, compose the runtime with your pallet by way of the `construct_runtime!`.
  - For more info, please visit [add a pallet to the runtime](https://docs.substrate.io/tutorials/work-with-pallets/add-a-pallet/).

- frame-benchmarking-cli expects us to implement the `frame_benchmarking_cli::ExtrinsicBuilder` trait for each extrinsic that we want to benchmark. Now that you've added your pallet to the runtime, let's extend the frame-benchmarking-cli to support your pallets' extrinsics by following the steps below:

  - Within [`benchmarking.rs`](./node/src/benchmarking.rs), create a struct whose fields are a client and any arguments the extrinsic expects. Implement the `frame_benchmarking_cli::ExtrinsicBuilder` trait on this struct. The `fn build(&self, nonce: u32) -> std::result::Result<OpaqueExtrinsic, &'static str>` method of this trait is used by the frame-benchmarking-cli tool internally to create instances of an extrinsic and populate a block.

  - Remember to implement rest of the functions that comprise the aforementioned trait.

  - Within [`command.rs`](./node/src/command.rs), inside the `BenchmarkCmd::Extrinsic(cmd)` code block and within the `ext_factory` vec, construct a new box with an instance of the above defined struct. The `ext_factory` vector holds a Boxed instance of every struct that implements the aforementioned trait.

- benchmark your pallets' extrinsic using the following command, `./target/release/node-template benchmark extrinsic --pallet <pallet-name> --extrinsic <extrinsic-name>`. This command outputs the block execution time(in nanoseconds) stats and also the number of extrinsic instances included in a block. Dividing the average block execution time by number of extrinsics per block gives you the average time taken(in nanoseconds) to execute an extrinisic.

Note:
A [`sample pallet`](./pallets/) and the required trait implementation to make it work with the frame-benchmarking-cli are included for your reference.
### Ink! smartcontract benchmarking

This node's runtime is composed of many pallets and one of them, [`pallet-contracts`](https://docs.rs/pallet-contracts/latest/pallet_contracts/) is capable of executing WASM code.[Ink!](https://use.ink/) smartcontracts are compiled to WASM and are uploaded, instantiated and invoked using `pallet-contracts`.

`pallet-contracts` exposes a few extrinsics and `Call` is the one to use to invoke a WASM smartcontract.
It takes the smartcontract address, the amount of funds to be transferred to the smartcontract, the max gas limit for the txn execution, the storage deposit limit and a byte vector as it's arguments.
The last argument, a byte vector, is where we specify the public function to invoke and the arguments to invoke it with.

As Ink! smartcontracts are executed by a pallet, we can use the same tool as above, frame-benchmarking-cli to benchmark the contracts' public functions. This can be done by following the steps below:

- Develop an Ink! smartcontract and compile it. For more info on how to do this, please refer to [develop Ink! smartcontracts](https://use.ink/getting-started/creating-an-ink-project/) and [compile Ink! smartcontracts](https://use.ink/getting-started/building-your-contract).

- You need two pieces of data at the very least, the address of the smartcontract and the byte vector that encodes both, the public function you wish to call and the arguments you wish to pass to it.

- Upon successfully compiling your smartcontract, navigate to `<contract-name-folder>/target/ink/` and open the `<contract-name.json>` file. This JSON represents the structure of your contract, it's constructors, public functions, types, events and other pieces.
public functions are referred to as messages and every message has a label(function name), for e.g., `"label": "get_num",` and a selector(function hash/ID), for e.g., `"selector": "0xcfe39fc5"`. This selector is what needs to be included in the byte vector. So, for e.g., if we wish to invoke `get_num` function whose selector is `0xcfe39fc5`, we need to pass the selector as a byte vector i.e., `vec![0xCF, 0xE3, 0x9f, 0xC5]`.

- We also need to pass in any arguments that a function might expect as [`SCALE`](https://lib.rs/crates/parity-scale-codec)-encoded byte vector, so in case a function, with selector `0xfbaf91e1` takes in an i64 value(say 100 for example) as an argument, our byte vector argument can be constructed using the following snippet of code,
`let mut call_data: Vec<u8> = Vec::new();`
`let mut msg_selector: Vec<u8> = [0xFB, 0xAF, 0x91, 0xE1].into();`
`let mut msg_args = 100u64.encode();`
`call_data.append(&mut msg_selector);`
`call_data.append(&mut msg_args);`
Note: encode() is a function that comes from the SCALE codec library and has encodings defined for most of the primitives. See [this](https://lib.rs/crates/scale-info) for more info.

- Start this node and deploy your contract. For more info on how to deploy your contract, please refer to [deploy Ink! smartcontracts](https://use.ink/getting-started/deploy-your-contract).

- Optionally, you can test it using the same UI mentioned in the link above by following the link [interact with Ink! smartcontracts](https://use.ink/getting-started/calling-your-contract).

- Once you've created an instance of your smartcontract, you can see it's address in the [contracts-UI](https://contracts-ui.substrate.io/). Copy that into the file [`contract_address.txt`](./contract_address.txt).

- Now we have the two essential arguments to pass to the `pallet-contracts::Call` and as for the rest of the arguments, `Default::default()` makes sense. From here on, the process is the same as what we followed for `Pallet Extrinsic benchmarking` section.

- An extrinsic that calls a public function of a smartcontract is actually represented by a `pallet-contracts::Call` extrinsic with the `dest` argument being your smartcontract address and the `data` argument being a byte vector constructed by appending the byte vector representation of the public function's selector and the SCALE-encoded representations of each of the arguments to be passed to that public function in the exact order.

- frame-benchmarking-cli expects us to implement the `frame_benchmarking_cli::ExtrinsicBuilder` trait for each extrinsic that we want to benchmark. Let's extend the frame-benchmarking-cli to support your smartcontracts' public function by following the steps below:

  - Within [`benchmarking.rs`](./node/src/benchmarking.rs), create a struct whose fields are a client, address of the smartcontract instance and any arguments the function expects. Implement the `frame_benchmarking_cli::ExtrinsicBuilder` trait on this struct.

  - Remember to implement rest of the functions that comprise the aforementioned trait.

  - Within [`command.rs`](./node/src/command.rs), inside the `BenchmarkCmd::Extrinsic(cmd)` code block and within the `ext_factory` vec, construct a new box with an instance of the above defined struct.

- benchmark your smartcontracts' function using the following command, `./target/release/node-template benchmark extrinsic --pallet <pallet-name> --extrinsic <extrinsic-name>`. This command outputs the block execution time(in nanoseconds) stats and also the number of extrinsic instances included in a block. Dividing the average block execution time by number of extrinsics per block gives you the average time taken(in nanoseconds) to execute an extrinisic.

Note:
A [`sample Ink! smartcontract`](./test/) and the required trait implementation to make it work with the frame-benchmarking-cli are included for your reference.

### Solidity WASM smartcontract benchmarking

As Solidity WASM smartcontracts are executed by the same `pallet-contracts` pallet, we can use the same tool and approach as above, frame-benchmarking-cli to benchmark the Solidity contracts' public functions. This can be done by following the steps below:

- Develop a [Solidity](https://soliditylang.org/) smartcontract and compile it to WASM using [Solang](https://solang.readthedocs.io/en/latest/).

- You need two pieces of data at the very least, the address of the smartcontract and the byte vector that encodes both, the public function you wish to call and the arguments you wish to pass to it.

- Start this node and deploy your contract. For more info on how to deploy your contract, please refer to [deploy Ink! smartcontracts](https://use.ink/getting-started/deploy-your-contract). Though the tutorial talks about Ink! smartcontracts in particular, the process also works for Solang-compiled Solidity smartcontracts.

- Optionally, you can test it using the same UI mentioned in the link above by following the link [interact with Ink! smartcontracts](https://use.ink/getting-started/calling-your-contract).

- Once you've created an instance of your smartcontract, you can see it's address in the [contracts-UI](https://contracts-ui.substrate.io/). Copy that into the file [`contract_address.txt`](./contract_address.txt).

- An extrinsic that calls a public function of a smartcontract is actually represented by a `pallet-contracts::Call` extrinsic with the `dest` argument being your smartcontract address and the `data` argument being a byte vector that's encoded just like in [Remix Web IDE](https://remix.ethereum.org/). Simulate the call in `Remix` and expand the transaction, copy the value of the input key and that's your input/call-data.

- frame-benchmarking-cli expects us to implement the `frame_benchmarking_cli::ExtrinsicBuilder` trait for each extrinsic that we want to benchmark. Let's extend the frame-benchmarking-cli to support your smartcontracts' public function by following the steps below:

  - Within [`benchmarking.rs`](./node/src/benchmarking.rs), create a struct whose fields are a client, address of the smartcontract instance. The input is a `byte-string without the hex prefix` and can be obtained from the input key within the transaction object after simulating a call to the same function with same args in `Remix Web IDE`. The function `hex_string_to_bytes` takes this hex encoded string and returns a byte vector representation, which can be passed in for data argument. Implement the `frame_benchmarking_cli::ExtrinsicBuilder` trait on this struct.

  - Remember to implement rest of the functions that comprise the aforementioned trait.

  - Within [`command.rs`](./node/src/command.rs), inside the `BenchmarkCmd::Extrinsic(cmd)` code block and within the `ext_factory` vec, construct a new box with an instance of the above defined struct.

- benchmark your smartcontracts' function using the following command, `./target/release/node-template benchmark extrinsic --pallet <pallet-name> --extrinsic <extrinsic-name>`. This command outputs the block execution time(in nanoseconds) stats and also the number of extrinsic instances included in a block. Dividing the average block execution time by number of extrinsics per block gives you the average time taken(in nanoseconds) to execute an extrinisic.

Note:
A [`sample solidity WASM smartcontract`](./solidity-sample-contract/) and the required trait implementation to make it work with the frame-benchmarking-cli are included for your reference.

### Run in Docker

First, install [Docker](https://docs.docker.com/get-docker/) and
[Docker Compose](https://docs.docker.com/compose/install/).

Then run the following command to start a single node development chain.

```bash
./scripts/docker_run.sh
```

This command will firstly compile your code, and then start a local development network. You can
also replace the default command
(`cargo build --release && ./target/release/node-template --dev --ws-external`)
by appending your own. A few useful ones are as follow.

```bash
# Run Substrate node without re-compiling
./scripts/docker_run.sh ./target/release/node-template --dev --ws-external

# Purge the local dev chain
./scripts/docker_run.sh ./target/release/node-template purge-chain --dev

# Check whether the code is compilable
./scripts/docker_run.sh cargo check
```
