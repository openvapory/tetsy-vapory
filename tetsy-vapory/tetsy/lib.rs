// Copyright 2015-2020 Parity Technologies (UK) Ltd.
// This file is part of Tetsy Vapory.

// Tetsy Vapory is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Tetsy Vapory is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Tetsy Vapory.  If not, see <http://www.gnu.org/licenses/>.

//! Vapcore client application.
#![warn(missing_docs)]

extern crate ansi_term;
extern crate docopt;
#[macro_use]
extern crate clap;
extern crate dir;
extern crate futures;
extern crate atty;
extern crate tetsy_jsonrpc_core;
extern crate num_cpus;
extern crate number_prefix;
extern crate parking_lot;
extern crate regex;
extern crate tetsy_rlp;
extern crate rpassword;
extern crate rustc_hex;
extern crate semver;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate toml;

extern crate blooms_db;
extern crate cli_signer;

extern crate client_traits;
extern crate common_types as types;
extern crate enjen;
extern crate vapcore;
extern crate vapcore_db;
extern crate vapcore_io as io;
extern crate vapcore_light as light;
extern crate vapcore_logger;
extern crate vapcore_miner as miner;
extern crate vapcore_network as network;
extern crate vapcore_private_tx;
extern crate vapcore_service;
extern crate vapcore_sync as sync;
extern crate vapory_types;
extern crate vapkey;
extern crate vapstore;
extern crate journaldb;
extern crate tetsy_keccak_hash as hash;
extern crate tetsy_kvdb;
extern crate node_filter;
extern crate tetsy_bytes as bytes;
extern crate tetsy_crypto;
extern crate tetsy_hash_fetch as hash_fetch;
extern crate tetsy_ipfs_api;
extern crate tetsy_local_store as local_store;
extern crate tetsy_path as path;
extern crate tetsy_rpc;
extern crate tetsy_runtime;
extern crate tetsy_updater as updater;
extern crate tetsy_version;
extern crate tetsy_registrar;
extern crate vapcore_snapshot;
extern crate spec;
extern crate verification;

#[macro_use]
extern crate log as rlog;

#[cfg(feature = "vapcore-accounts")]
extern crate vapcore_accounts as accounts;

#[cfg(feature = "secretstore")]
extern crate vapcore_secretstore;

#[cfg(feature = "secretstore")]
extern crate vapabi;

#[cfg(feature = "secretstore")]
extern crate vapcore_call_contract as call_contract;

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

#[cfg(test)]
extern crate tempdir;

mod account;
mod account_utils;
mod blockchain;
mod cache;
mod cli;
mod configuration;
mod export_hardcoded_sync;
mod ipfs;
mod deprecated;
mod helpers;
mod informant;
mod light_helpers;
mod modules;
mod params;
mod presale;
mod rpc;
mod rpc_apis;
mod run;
mod secretstore;
mod signer;
mod snapshot_cmd;
mod upgrade;
mod user_defaults;
mod db;

use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;

use cli::Args;
use configuration::{Cmd, Execute};
use deprecated::find_deprecated;
use hash::keccak_buffer;

#[cfg(feature = "memory_profiling")]
use std::alloc::System;

pub use self::configuration::Configuration;
pub use self::run::RunningClient;
pub use tetsy_rpc::PubSubSession;
pub use vapcore_logger::{Config as LoggerConfig, setup_log, RotatingLogger};

#[cfg(feature = "memory_profiling")]
#[global_allocator]
static A: System = System;

fn print_hash_of(maybe_file: Option<String>) -> Result<String, String> {
	if let Some(file) = maybe_file {
		let mut f = BufReader::new(File::open(&file).map_err(|_| "Unable to open file".to_owned())?);
		let hash = keccak_buffer(&mut f).map_err(|_| "Unable to read from file".to_owned())?;
		Ok(format!("{:x}", hash))
	} else {
		Err("Streaming from standard input not yet supported. Specify a file.".to_owned())
	}
}

#[cfg(feature = "deadlock_detection")]
fn run_deadlock_detection_thread() {
	use std::thread;
	use std::time::Duration;
	use parking_lot::deadlock;
	use ansi_term::Style;

	info!("Starting deadlock detection thread.");
	// Create a background thread which checks for deadlocks every 10s
	thread::spawn(move || {
		loop {
			thread::sleep(Duration::from_secs(10));
			let deadlocks = deadlock::check_deadlock();
			if deadlocks.is_empty() {
				continue;
			}

			warn!("{} {} detected", deadlocks.len(), Style::new().bold().paint("deadlock(s)"));
			for (i, threads) in deadlocks.iter().enumerate() {
				warn!("{} #{}", Style::new().bold().paint("Deadlock"), i);
				for t in threads {
					warn!("Thread Id {:#?}", t.thread_id());
					warn!("{:#?}", t.backtrace());
				}
			}
		}
	});
}

/// Action that Parity performed when running `start`.
pub enum ExecutionAction {
	/// The execution didn't require starting a node, and thus has finished.
	/// Contains the string to print on stdout, if any.
	Instant(Option<String>),

	/// The client has started running and must be shut down manually by calling `shutdown`.
	///
	/// If you don't call `shutdown()`, execution will continue in the background.
	Running(RunningClient),
}

fn execute<Cr, Rr>(
	command: Execute,
	logger: Arc<RotatingLogger>,
	on_client_rq: Cr,
	on_updater_rq: Rr
) -> Result<ExecutionAction, String>
	where
		Cr: Fn(String) + 'static + Send,
		Rr: Fn() + 'static + Send
{
	#[cfg(feature = "deadlock_detection")]
	run_deadlock_detection_thread();

	match command.cmd {
		Cmd::Run(run_cmd) => {
			let outcome = run::execute(run_cmd, logger, on_client_rq, on_updater_rq)?;
			Ok(ExecutionAction::Running(outcome))
		},
		Cmd::Version => Ok(ExecutionAction::Instant(Some(Args::print_version()))),
		Cmd::Hash(maybe_file) => print_hash_of(maybe_file).map(|s| ExecutionAction::Instant(Some(s))),
		Cmd::Account(account_cmd) => account::execute(account_cmd).map(|s| ExecutionAction::Instant(Some(s))),
		Cmd::ImportPresaleWallet(presale_cmd) => presale::execute(presale_cmd).map(|s| ExecutionAction::Instant(Some(s))),
		Cmd::Blockchain(blockchain_cmd) => blockchain::execute(blockchain_cmd).map(|_| ExecutionAction::Instant(None)),
		Cmd::SignerToken(ws_conf, logger_config) => signer::execute(ws_conf, logger_config).map(|s| ExecutionAction::Instant(Some(s))),
		Cmd::SignerSign { id, pwfile, port, authfile } => cli_signer::signer_sign(id, pwfile, port, authfile).map(|s| ExecutionAction::Instant(Some(s))),
		Cmd::SignerList { port, authfile } => cli_signer::signer_list(port, authfile).map(|s| ExecutionAction::Instant(Some(s))),
		Cmd::SignerReject { id, port, authfile } => cli_signer::signer_reject(id, port, authfile).map(|s| ExecutionAction::Instant(Some(s))),
		Cmd::Snapshot(snapshot_cmd) => snapshot_cmd::execute(snapshot_cmd).map(|s| ExecutionAction::Instant(Some(s))),
		Cmd::ExportHardcodedSync(export_hs_cmd) => export_hardcoded_sync::execute(export_hs_cmd).map(|s| ExecutionAction::Instant(Some(s))),
	}
}

/// Starts the parity client.
///
/// `on_client_rq` is the action to perform when the client receives an RPC request to be restarted
/// with a different chain.
///
/// `on_updater_rq` is the action to perform when the updater has a new binary to execute.
///
/// The first parameter is the command line arguments that you would pass when running the parity
/// binary.
///
/// On error, returns what to print on stderr.
// FIXME: totally independent logging capability, see https://github.com/openvapory/tetsy-vapory/issues/10252
pub fn start<Cr, Rr>(
	conf: Configuration,
	logger: Arc<RotatingLogger>,
	on_client_rq: Cr,
	on_updater_rq: Rr
) -> Result<ExecutionAction, String>
	where
		Cr: Fn(String) + 'static + Send,
		Rr: Fn() + 'static + Send
{
	let deprecated = find_deprecated(&conf.args);
	for d in deprecated {
		println!("{}", d);
	}

	execute(conf.into_command()?, logger, on_client_rq, on_updater_rq)
}
