---
id: Diem-framework
title: Diem Framework
custom_edit_url: https://github.com/aptos-labs/diem-core/edit/main/Diem-move/Diem-framework/README.md
---

## The Diem Framework

The Diem Framework defines the standard actions that can be performed on-chain
both by the Diem VM---through the various prologue/epilogue functions---and by
users of the blockchain---through the allowed set of transactions. This
directory contains different directories that hold the source Move
modules and transaction scripts, along with a framework for generation of
documentation, ABIs, and error information from the Move source
files. See the [Layout](#layout) section for a more detailed overview of the structure.

## Documentation

Each of the main components of the Diem Framework and contributing guidelines are documented separately. See them by version below:

* *Diem tokens* - [main](https://github.com/aptos-labs/diem-core/blob/main/diem-move/framework/diem-token/doc/overview.md), [testnet](https://github.com/aptos-labs/diem-core/blob/testnet/diem-move/framework/diem-token/doc/overview.md), [devnet](https://github.com/aptos-labs/diem-core/blob/devnet/diem-move/framework/diem-token/doc/overview.md)
* *Diem framework* - [main](https://github.com/aptos-labs/diem-core/blob/main/diem-move/framework/diem-framework/doc/overview.md), [testnet](https://github.com/aptos-labs/diem-core/blob/testnet/diem-move/framework/diem-framework/doc/overview.md), [devnet](https://github.com/aptos-labs/diem-core/blob/devnet/diem-move/framework/diem-framework/doc/overview.md)
* *Diem stdlib* - [main](https://github.com/aptos-labs/diem-core/blob/main/diem-move/framework/diem-stdlib/doc/overview.md), [testnet](https://github.com/aptos-labs/diem-core/blob/testnet/diem-move/framework/diem-stdlib/doc/overview.md), [devnet](https://github.com/aptos-labs/diem-core/blob/devnet/diem-move/framework/diem-stdlib/doc/overview.md)
* *Move stdlib* - [main](https://github.com/aptos-labs/diem-core/blob/main/diem-move/framework/move-stdlib/doc/overview.md), [testnet](https://github.com/aptos-labs/diem-core/blob/testnet/diem-move/framework/move-stdlib/doc/overview.md), [devnet](https://github.com/aptos-labs/diem-core/blob/devnet/diem-move/framework/move-stdlib/doc/overview.md)

Follow our [contributing guidelines](CONTRIBUTING.md) and basic coding standards for the Diem Framework.

## Compilation and Generation

The documents above were created by the Move documentation generator for Diem. It is available as part of the Diem CLI. To see its options, run:
```shell
diem move document --help
```

The documentation process is also integrated into the framework building process and will be automatically triggered like other derived artifacts, via `cached-packages` or explicit release building.

## Running Move tests

To test our Move code while developing the Diem Framework, run `cargo test` inside this directory:

```
cargo test
```

(Alternatively, run `cargo test -p diem-framework` from anywhere.)

To skip the Move prover tests, run:

```
cargo test -- --skip prover
```

To filter and run only the tests in specific packages (e.g., `diem_stdlib`), run:

```
cargo test -- diem_stdlib --skip prover
```

(See tests in `tests/move_unit_test.rs` to determine which filter to use; e.g., to run the tests in `diem_framework` you must filter by `move_framework`.)

Sometimes, Rust runs out of stack memory in dev build mode.  You can address this by either:
1. Adjusting the stack size

```
export RUST_MIN_STACK=4297152
```

2. Compiling in release mode

```
cargo test --release -- --skip prover
```

## Layout
The overall structure of the Diem Framework is as follows:

```
├── diem-framework                                 # Sources, testing and generated documentation for Diem framework component
├── diem-token                                 # Sources, testing and generated documentation for Diem token component
├── diem-stdlib                                 # Sources, testing and generated documentation for Diem stdlib component
├── move-stdlib                                 # Sources, testing and generated documentation for Move stdlib component
├── cached-packages                                 # Tooling to generate SDK from mvoe sources.
├── src                                     # Compilation and generation of information from Move source files in the Diem Framework. Not designed to be used as a Rust library
├── releases                                    # Move release bundles
└── tests
```
