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

//! Tetsy RPC requests Metadata.
use std::sync::Arc;

use tetsy_jsonrpc_core;
use tetsy_jsonrpc_pubsub::{Session, PubSubMetadata};

use v1::types::Origin;

/// RPC methods metadata.
#[derive(Clone, Default, Debug)]
pub struct Metadata {
	/// Request origin
	pub origin: Origin,
	/// Request PubSub Session
	pub session: Option<Arc<Session>>,
}

impl tetsy_jsonrpc_core::Metadata for Metadata {}
impl PubSubMetadata for Metadata {
	fn session(&self) -> Option<Arc<Session>> {
		self.session.clone()
	}
}
