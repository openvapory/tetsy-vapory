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

//! RPC implementations for the light client.
//!
//! This doesn't re-implement all of the RPC APIs, just those which aren't
//! significantly generic to be reused.

pub mod vap;
pub mod tetsy;
pub mod tetsy_set;
pub mod trace;
pub mod net;

pub use self::vap::VapClient;
pub use self::tetsy::TetsyClient;
pub use self::tetsy_set::TetsySetClient;
pub use self::net::NetClient;
pub use self::trace::TracesClient;
