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

//! Updater for Tetsy executables

#![warn(missing_docs)]

extern crate client_traits;
extern crate common_types;
extern crate vapabi;
extern crate vapabi_derive;
extern crate vapcore;
extern crate vapcore_sync as sync;
extern crate vapory_types;
extern crate tetsy_keccak_hash as hash;
extern crate tetsy_bytes as bytes;
extern crate tetsy_hash_fetch as hash_fetch;
extern crate tetsy_path;
extern crate tetsy_version as version;
extern crate parking_lot;
extern crate rand;
extern crate semver;
extern crate target_info;

#[macro_use]
extern crate vapabi_contract;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

#[cfg(test)]
extern crate tempdir;

#[cfg(test)]
#[macro_use]
extern crate matches;

mod updater;
mod types;
mod service;

pub use service::Service;
pub use types::{ReleaseInfo, OperationsInfo, CapState, VersionInfo, ReleaseTrack};
pub use updater::{Updater, UpdateFilter, UpdatePolicy};
