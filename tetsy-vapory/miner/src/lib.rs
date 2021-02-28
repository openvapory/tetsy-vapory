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

#![warn(missing_docs)]

//! Miner module
//! Keeps track of transactions and mined block.

extern crate ansi_term;
extern crate common_types as types;
extern crate vapabi;
extern crate vapcore_call_contract as call_contract;
extern crate vapory_types;
extern crate futures;

extern crate tetsy_util_mem;
extern crate tetsy_keccak_hash as hash;
extern crate linked_hash_map;
extern crate tetsy_runtime;
extern crate parking_lot;
#[cfg(feature = "price-info")]
extern crate price_info;
extern crate tetsy_registrar;
extern crate tetsy_rlp;
extern crate transaction_pool as txpool;
extern crate serde;

#[macro_use]
extern crate vapabi_contract;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate trace_time;

#[cfg(test)]
extern crate rustc_hex;
#[cfg(test)]
extern crate tetsy_crypto;
#[cfg(test)]
extern crate env_logger;

pub mod external;
#[cfg(feature = "price-info")]
pub mod gas_price_calibrator;
pub mod gas_pricer;
pub mod local_accounts;
pub mod pool;
pub mod service_transaction_checker;
#[cfg(feature = "work-notify")]
pub mod work_notify;
