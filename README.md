![Tetsy Vapory](docs/logo-tetsy-vapory.svg)

<h2 align="center">The Fastest and most Advanced Vapory Client.</h2>

<p align="center"><strong><a href="https://github.com/openvapory/tetsy-vapory/releases/latest">» Download the latest release «</a></strong></p>

<a href="https://www.gnu.org/licenses/gpl-3.0.en.html" target="_blank"><img src="https://img.shields.io/badge/license-GPL%20v3-green.svg" /></a></p>

## Table of Contents

1. [Description](#chapter-001)
2. [Technical Overview](#chapter-002)
3. [Building](#chapter-003)<br>
  3.1 [Building Dependencies](#chapter-0031)<br>
  3.2 [Building from Source Code](#chapter-0032)<br>
  3.3 [Simple One-Line Installer for Mac and Linux](#chapter-0033)<br>
  3.4 [Starting Tetsy Vapory](#chapter-0034)
4. [Testing](#chapter-004)
5. [Documentation](#chapter-005)
6. [Toolchain](#chapter-006)
7. [Community](#chapter-007)
8. [Contributing](#chapter-008)
9. [License](#chapter-009)


## 1. Description <a id="chapter-001"></a>

**Built for mission-critical use**: Miners, service providers, and exchanges need fast synchronisation and maximum uptime. Tetsy Vapory provides the core infrastructure essential for speedy and reliable services.

- Clean, modular codebase for easy customisation
- Advanced CLI-based client
- Minimal memory and storage footprint
- Synchronise in hours, not days with Warp Sync
- Modular for light integration into your service or product

## 2. Technical Overview <a id="chapter-002"></a>

Tetsy Vapory's goal is to be the fastest, lightest, and most secure Vapory client. We are developing Tetsy Vapory using the sophisticated and cutting-edge **Rust programming language**. Tetsy Vapory is licensed under the GPLv3 and can be used for all your Vapory needs.

By default, Tetsy Vapory runs a JSON-RPC HTTP server on port `:8545` and a Web-Sockets server on port `:8546`. This is fully configurable and supports a number of APIs.

If you run into problems while using Tetsy Vapory, check out the [wiki for documentation](https://wiki.tetcoin.org/), feel free to [file an issue in this repository](https://github.com/openvapory/tetsy-vapory/issues/new), or hop on our [Gitter](https://gitter.im/paritytech/parity) or [Riot](https://riot.im/app/#/group/+tetsy:matrix.tetcoin.org) chat room to ask a question. We are glad to help! **For security-critical issues**, please refer to the security policy outlined in [SECURITY.md](SECURITY.md).

Tetsy Vapory's current beta-release is 2.6. You can download it at [the releases page](https://github.com/openvapory/tetsy-vapory/releases) or follow the instructions below to build from source. Please, mind the [CHANGELOG.md](CHANGELOG.md) for a list of all changes between different versions.

## 3. Building <a id="chapter-003"></a>

### 3.1 Build Dependencies <a id="chapter-0031"></a>

Tetsy Vapory requires **latest stable Rust version** to build.

We recommend installing Rust through [rustup](https://www.rustup.rs/). If you don't already have `rustup`, you can install it like this:

- Linux:
  ```bash
  $ curl https://sh.rustup.rs -sSf | sh
  ```

  Tetsy Vapory also requires `gcc`, `g++`, `pkg-config`, `file`, `make`, and `cmake` packages to be installed.

- OSX:
  ```bash
  $ curl https://sh.rustup.rs -sSf | sh
  ```

  `clang` is required. It comes with Xcode command line tools or can be installed with homebrew.

- Windows:
  Make sure you have Visual Studio 2015 with C++ support installed. Next, download and run the `rustup` installer from
  https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe, start "VS2015 x64 Native Tools Command Prompt", and use the following command to install and set up the `msvc` toolchain:
  ```bash
  $ rustup default stable-x86_64-pc-windows-msvc
  ```

Once you have `rustup` installed, then you need to install:
* [Perl](https://www.perl.org)
* [Yasm](https://yasm.tortall.net)

Make sure that these binaries are in your `PATH`. After that, you should be able to build Tetsy Vapory from source.

### 3.2 Build from Source Code <a id="chapter-0032"></a>

```bash
# download Tetsy Vapory code
$ git clone https://github.com/openvapory/tetsy-vapory
$ cd tetsy-vapory

# build in release mode
$ cargo build --release --features final
```

This produces an executable in the `./target/release` subdirectory.

Note: if cargo fails to parse manifest try:

```bash
$ ~/.cargo/bin/cargo build --release
```

Note, when compiling a crate and you receive errors, it's in most cases your outdated version of Rust, or some of your crates have to be recompiled. Cleaning the repository will most likely solve the issue if you are on the latest stable version of Rust, try:

```bash
$ cargo clean
```

This always compiles the latest nightly builds. If you want to build stable or beta, do a

```bash
$ git checkout stable
```

or

```bash
$ git checkout beta
```

### 3.3 Simple One-Line Installer for Mac and Linux <a id="chapter-0033"></a>

```bash
bash <(curl https://get.tetcoin.org -L)
```

The one-line installer always defaults to the latest beta release. To install a stable release, run:

```bash
bash <(curl https://get.tetcoin.org -L) -r stable
```

### 3.4 Starting Tetsy Vapory <a id="chapter-0034"></a>

#### Manually

To start Tetsy Vapory manually, just run

```bash
$ ./target/release/tetsy
```

so Tetsy Vapory begins syncing the Vapory blockchain.

#### Using `systemd` service file

To start Tetsy Vapory as a regular user using `systemd` init:

1. Copy `./scripts/tetsy.service` to your
`systemd` user directory (usually `~/.config/systemd/user`).
2. Copy release to bin folder, write `sudo install ./target/release/tetsy /usr/bin/tetsy`
3. To configure Tetsy Vapory, write a `/etc/tetsy/config.toml` config file, see [Configuring Tetsy Vapory](https://paritytech.github.io/wiki/Configuring-Parity) for details.

## 4. Testing <a id="chapter-004"></a>

Download the required test files: `git submodule update --init --recursive`. You can run tests with the following commands:

* **All** packages
  ```
  cargo test --all
  ```

* Specific package
  ```
  cargo test --package <spec>
  ```

Replace `<spec>` with one of the packages from the [package list](#package-list) (e.g. `cargo test --package vvmbin`).

You can show your logs in the test output by passing `--nocapture` (i.e. `cargo test --package vvmbin -- --nocapture`)

## 5. Documentation <a id="chapter-005"></a>

Official website: https://tetcoin.org

Be sure to [check out our wiki](https://wiki.tetcoin.org) for more information.

### Viewing documentation for Tetsy Vapory packages

You can generate documentation for Tetsy Vapory Rust packages that automatically opens in your web browser using [rustdoc with Cargo](https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html#using-rustdoc-with-cargo) (of the The Rustdoc Book), by running the the following commands:

* **All** packages
  ```
  cargo doc --document-private-items --open
  ```

* Specific package
  ```
  cargo doc --package <spec> -- --document-private-items --open
  ```

Use`--document-private-items` to also view private documentation and `--no-deps` to exclude building documentation for dependencies.

Replacing `<spec>` with one of the following from the details section below (i.e. `cargo doc --package tetsy-vapory --open`):

<a id="package-list"></a>
**Package List**
<details><p>

* Tetsy Vapory (VapCore) Client Application
  ```bash
  tetsy-vapory
  ```
* Tetsy Vapory Account Management, Key Management Tool, and Keys Generator
  ```bash
  vapcore-accounts, vapkey-cli, vapstore, vapstore-cli
  ```
* Parity Chain Specification
  ```bash
  chainspec
  ```
* Parity CLI Signer Tool & RPC Client
  ```bash
  cli-signer tetsy-rpc-client
  ```
* Tetsy Vapory Vapash & ProgPoW Implementations
  ```bash
  vapash
  ```
* Parity (VapCore) Library
  ```bash
  vapcore
  ```
  * Tetsy Vapory Blockchain Database, Test Generator, Configuration,
Caching, Importing Blocks, and Block Information
    ```bash
    vapcore-blockchain
    ```
  * Tetsy Vapory (VapCore) Contract Calls and Blockchain Service & Registry Information
    ```bash
    vapcore-call-contract
    ```
  * Tetsy Vapory (VapCore) Database Access & Utilities, Database Cache Manager
    ```bash
    vapcore-db
    ```
  * Tetsy Vapory Virtual Machine (VVM) Rust Implementation
    ```bash
    vvm
    ```
  * Tetsy Vapory (VapCore) Light Client Implementation
    ```bash
    vapcore-light
    ```
  * Parity Smart Contract based Node Filter, Manage Permissions of Network Connections
    ```bash
    node-filter
    ```
  * Parity Private Transactions
    ```bash
    private
    ```
  * Tetsy Vapory (VapCore) Client & Network Service Creation & Registration with the I/O Subsystem
    ```bash
    vapcore-service
    ```
  * Tetsy Vapory (VapCore) Blockchain Synchronization
    ```bash
    vapcore-sync
    ```
  * Tetsy Vapory Common Types
    ```bash
    common-types
    ```
  * Tetsy Vapory Virtual Machines (VM) Support Library
    ```bash
    vm
    ```
  * Tetsy Vapory WASM Interpreter
    ```bash
    wasm
    ```
  * Vapcore WASM Test Runner
    ```bash
    vapcore-wasm-run-test
    ```
  * Parity VVM Implementation
    ```bash
    vvmbin
    ```
  * Tetsy Vapory IPFS-compatible API
    ```bash
    tetsy-ipfs-api
    ```
  * Tetsy Vapory JSON Deserialization
    ```bash
    vapjson
    ```
  * Tetsy Vapory State Machine Generalization for Consensus Engines
    ```bash
    tetsy-machine
    ```
* Tetsy Vapory (VapCore) Miner Interface
  ```bash
  vapcore-miner tetsy-local-store price-info vapcore-stratum using_queue
  ```
* Tetsy Vapory (VapCore) Logger Implementation
  ```bash
  vapcore-logger
  ```
* C bindings library for the Tetsy Vapory client
  ```bash
  tetsy-clib
  ```
* Tetsy Vapory JSON-RPC Servers
  ```bash
  tetsy-rpc
  ```
* Tetsy Vapory (VapCore) Secret Store
  ```bash
  vapcore-secretstore
  ```
* Parity Updater Service
  ```bash
  tetsy-updater tetsy-hash-fetch
  ```
* Parity Core Libraries (Parity Util)
  ```bash
  vapcore-bloom-journal blooms-db dir vip-712 fake-fetch fastmap fetch vapcore-io
  journaldb tetsy-keccak-hasher len-caching-lock macros memory-cache memzero
  migration-rocksdb vapcore-network vapcore-network-devp2p panic_hook
  patricia-trie-vapory tetsy-registrar rlp_compress tetsy-rlp-derive tetsy-runtime stats
  time-utils triehash-vapory unexpected tetsy-version
  ```

</p></details>

### Contributing to documentation for Tetsy Vapory packages

[Document source code](https://doc.rust-lang.org/1.9.0/book/documentation.html) for Tetsy Vapory packages by annotating the source code with documentation comments.

Example (generic documentation comment):
```markdown
/// Summary
///
/// Description
///
/// # Panics
///
/// # Errors
///
/// # Safety
///
/// # Examples
///
/// Summary of Example 1
///
/// ```rust
/// // insert example 1 code here for use with documentation as tests
/// ```
///
```

## 6. Toolchain <a id="chapter-006"></a>

In addition to the Tetsy Vapory client, there are additional tools in this repository available:

- [vvmbin](./vvmbin) - Tetsy Vapory VVM Implementation.
- [vapstore](./accounts/vapstore) - Tetsy Vapory Key Management.
- [vapkey](./accounts/vapkey) - Tetsy Vapory Keys Generator.

The following tool is available in a separate repository:
- [vapabi](https://github.com/paritytech/vapabi) - Tetsy Vapory Encoding of Function Calls. [Docs here](https://crates.io/crates/vapabi)
- [whisper](https://github.com/paritytech/whisper) - Tetsy Vapory Whisper-v2 PoC Implementation.

## 7. Community <a id="chapter-007"></a>

### Join the chat!

Questions? Get in touch with us on Twitter:
[Twitter](https://twitter.com/tetcoin)

## 8. Contributing <a id="chapter-008"></a>

An introduction has been provided in the ["So You Want to be a Core Developer" presentation slides by Hernando Castano](http://tiny.cc/contrib-to-tetsy-vap). Additional guidelines are provided in [CONTRIBUTING](./.github/CONTRIBUTING.md).

### Contributor Code of Conduct

[CODE_OF_CONDUCT](./.github/CODE_OF_CONDUCT.md)

## 9. License <a id="chapter-009"></a>

[LICENSE](./LICENSE)
