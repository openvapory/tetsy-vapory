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

//! Light client logic and implementation.
//!
//! A "light" client stores very little chain-related data locally
//! unlike a full node, which stores all blocks, headers, receipts, and more.
//!
//! This enables the client to have a much lower resource footprint in
//! exchange for the cost of having to ask the network for state data
//! while responding to queries. This makes a light client unsuitable for
//! low-latency applications, but perfectly suitable for simple everyday
//! use-cases like sending transactions from a personal account.
//!
//! The light client performs a header-only sync, doing verification and pruning
//! historical blocks. Upon pruning, batches of 2048 blocks have a number => (hash, TD)
//! mapping sealed into "canonical hash tries" which can later be used to verify
//! historical block queries from peers.

#![deny(missing_docs)]

pub mod client;
pub mod cht;
pub mod net;
pub mod on_demand;
pub mod transaction_queue;
pub mod cache;
pub mod provider;

mod types;

pub use self::cache::Cache;
pub use self::provider::{Provider, MAX_HEADERS_PER_REQUEST};
pub use self::transaction_queue::TransactionQueue;
pub use types::request as request;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;

extern crate bincode;
extern crate client_traits;
extern crate common_types;
extern crate enjen;
extern crate vapcore_blockchain;
extern crate vapcore_db;
extern crate vapcore_io as io;
extern crate vapcore_network as network;
extern crate executive_state;
extern crate tetsy_bytes as bytes;
extern crate vapory_types;
extern crate vapcore_miner as miner;
extern crate tetsy_hash_db;
extern crate tetsy_util_mem;
extern crate tetsy_util_mem as malloc_size_of;
extern crate failsafe;
extern crate futures;
extern crate tetsy_keccak_hasher;
extern crate mashina;
extern crate tetsy_memory_db;
extern crate tetsy_trie_db as trie;
extern crate patricia_trie_vapory as vaptrie;
extern crate fastmap;
extern crate rand;
extern crate tetsy_rlp;
extern crate parking_lot;
#[macro_use]
extern crate tetsy_rlp_derive;
extern crate serde;
extern crate vapcore_spec;
extern crate smallvec;
extern crate tetsy_stats;
extern crate tetsy_vm;
extern crate tetsy_keccak_hash as hash;
extern crate triehash_vapory as triehash;
extern crate tetsy_kvdb;
extern crate memory_cache;
extern crate derive_more;
extern crate verification;

#[cfg(test)]
extern crate vapcore;
#[cfg(test)]
extern crate tetsy_kvdb_memorydb;
#[cfg(test)]
extern crate tempdir;
extern crate journaldb;
