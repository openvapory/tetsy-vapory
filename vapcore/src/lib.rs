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

#![warn(missing_docs, unused_extern_crates)]

//! Vapcore library

extern crate account_state;
extern crate ansi_term;
extern crate client_traits;
extern crate common_types as types;
extern crate engine;
extern crate vapcore_blockchain as blockchain;
extern crate vapcore_call_contract as call_contract;
extern crate vapcore_db as db;
extern crate vapcore_io as io;
extern crate vapcore_miner;
extern crate vapory_types;
extern crate executive_state;
extern crate futures;
extern crate tetsy_hash_db;
extern crate itertools;
extern crate journaldb;
extern crate tetsy_keccak_hash as hash;
extern crate tetsy_kvdb;
extern crate machine;
extern crate memory_cache;
extern crate tetsy_bytes as bytes;
extern crate parking_lot;
extern crate tetsy_trie_db as trie;
extern crate patricia_trie_vapory as vaptrie;
extern crate rand;
extern crate rayon;
extern crate tetsy_registrar;
extern crate tetsy_rlp;
extern crate rustc_hex;
extern crate serde;
extern crate snapshot;
extern crate spec;
extern crate state_db;
extern crate trace;
extern crate trie_vm_factories;
extern crate triehash_vapory as triehash;
extern crate tetsy_unexpected;
extern crate using_queue;
extern crate verification;
extern crate tetsy_vm;

#[cfg(test)]
extern crate account_db;
#[cfg(test)]
extern crate vapcore_accounts as accounts;
#[cfg(test)]
extern crate tetsy_stats;

#[cfg(feature = "stratum")]
extern crate vapcore_stratum;

#[cfg(feature = "stratum")]
extern crate vapash;

#[cfg(any(test, feature = "test-helpers"))]
extern crate tetsy_crypto;
#[cfg(any(test, feature = "test-helpers"))]
extern crate vapjson;
#[cfg(any(test, feature = "test-helpers"))]
extern crate tetsy_kvdb_memorydb;
#[cfg(any(test, feature = "tetsy-kvdb-rocksdb"))]
extern crate tetsy_kvdb_rocksdb;
#[cfg(feature = "json-tests")]
#[macro_use]
extern crate lazy_static;
#[cfg(any(test, feature = "json-tests"))]
#[macro_use]
extern crate tetsy_macros;
#[cfg(any(test, feature = "test-helpers"))]
extern crate pod;
#[cfg(any(test, feature = "blooms-db"))]
extern crate blooms_db;
#[cfg(feature = "env_logger")]
extern crate env_logger;
#[cfg(test)]
extern crate serde_json;
#[cfg(any(test, feature = "tempdir"))]
extern crate tempdir;

#[macro_use]
extern crate log;
#[macro_use]
extern crate trace_time;

#[cfg_attr(test, macro_use)]
extern crate vvm;

#[cfg(all(test, feature = "price-info"))]
extern crate fetch;

#[cfg(all(test, feature = "price-info"))]
extern crate tetsy_runtime;

pub mod block;
pub mod client;
pub mod miner;

#[cfg(test)]
mod tests;
#[cfg(feature = "json-tests")]
pub mod json_tests;
#[cfg(any(test, feature = "test-helpers"))]
pub mod test_helpers;
