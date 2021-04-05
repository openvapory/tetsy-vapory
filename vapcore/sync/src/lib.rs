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

//! Blockchain sync module
//! Implements vapory protocol version 63 as specified here:
//! https://github.com/vaporyco/wiki/wiki/Vapory-Wire-Protocol
//!

// needed to make the procedural macro `MallocSizeOf` to work
#[macro_use] extern crate tetsy_util_mem as malloc_size_of;

mod api;
mod chain;
mod blocks;
mod block_sync;
mod sync_io;
mod private_tx;
mod snapshot_sync;
mod transactions_stats;

pub mod light_sync;

#[cfg(test)]
mod tests;

pub use api::*;
pub use chain::{SyncStatus, SyncState};
pub use devp2p::validate_node_url;
pub use network::{NonReservedPeerMode, Error, ConnectionFilter, ConnectionDirection};
pub use crate::private_tx::{PrivateTxHandler, NoopPrivateTxHandler, SimplePrivateTxHandler};
