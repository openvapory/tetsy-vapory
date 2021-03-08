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

//! Types used in the public API
//!
//! This crate stores Tetsy Vapory specific types that are
//! COMMONLY used across different separate modules of the codebase.
//! It should only focus on data structures, not any logic that relates to them.
//!
//! The interaction between modules should be possible by
//! implementing a required trait that potentially uses some of the data
//! structures from that crate.
//!
//! NOTE If you can specify your data type in the same crate as your trait, please do that.
//! Don't treat this crate as a bag for any types that we use in Tetsy Vapory.
//! This one is reserved for types that are shared heavily (like transactions),
//! historically this contains types extracted from `vapcore` crate, if possible
//! we should try to dissolve that crate in favour of more fine-grained crates,
//! by moving the types closer to where they are actually required.

#![warn(missing_docs, unused_extern_crates)]

extern crate vapbloom;
extern crate vapory_types;
extern crate vapjson;
extern crate tetsy_crypto;
#[macro_use]
extern crate derive_more;
extern crate tetsy_keccak_hash as hash;
extern crate tetsy_bytes as bytes;
extern crate patricia_trie_vapory as vaptrie;
extern crate tetsy_snappy;
extern crate tetsy_rlp;
extern crate tetsy_unexpected;

#[macro_use]
extern crate tetsy_rlp_derive;
extern crate tetsy_util_mem;
extern crate tetsy_util_mem as malloc_size_of;

#[cfg(test)]
extern crate rustc_hex;

#[macro_use]
pub mod views;

pub mod account_diff;
pub mod ancestry_action;
pub mod basic_account;
pub mod block;
pub mod block_status;
pub mod blockchain_info;
pub mod call_analytics;
pub mod chain_notify;
pub mod client_types;
pub mod encoded;
pub mod engines;
pub mod errors;
pub mod filter;
pub mod header;
pub mod ids;
pub mod io_message;
pub mod import_route;
pub mod log_entry;
pub mod pruning_info;
pub mod receipt;
pub mod security_level;
pub mod snapshot;
pub mod state_diff;
pub mod trace_filter;
pub mod transaction;
pub mod tree_route;
pub mod verification;
pub mod data_format;

/// Type for block number.
pub type BlockNumber = u64;
