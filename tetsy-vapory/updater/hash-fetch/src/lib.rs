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

//! Hash-addressed content resolver & fetcher.

#![warn(missing_docs)]

#[macro_use]
extern crate log;

extern crate call_contract;
extern crate vapabi;
extern crate tetsy_bytes as bytes;
extern crate vapory_types;
extern crate futures;
extern crate tetsy_keccak_hash as hash;
extern crate mime;
extern crate mime_guess;
extern crate tetsy_runtime;
extern crate rand;
extern crate rustc_hex;
extern crate tetsy_registrar;
extern crate types;

pub extern crate fetch;

// #[macro_use]
extern crate vapabi_derive;
#[macro_use]
extern crate vapabi_contract;
#[cfg(test)]
extern crate parking_lot;
#[cfg(test)]
extern crate fake_fetch;

mod client;

pub mod urlhint;

pub use client::{HashFetch, Client, Error};
pub use fetch::Abort;
