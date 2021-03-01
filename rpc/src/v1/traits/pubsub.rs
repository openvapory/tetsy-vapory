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

//! Tetsy-specific PUB-SUB rpc interface.

use tetsy_jsonrpc_core::{Result, Value, Params};
use tetsy_jsonrpc_pubsub::{typed::Subscriber, SubscriptionId};
use tetsy_jsonrpc_derive::rpc;

/// Tetsy-specific PUB-SUB rpc interface.
#[rpc(server)]
pub trait PubSub {
	/// Pub/Sub Metadata
	type Metadata;

	/// Subscribe to changes of any RPC method in Tetsy.
	#[pubsub(subscription = "tetsy_subscription", subscribe, name = "tetsy_subscribe")]
	fn tetsy_subscribe(&self, _: Self::Metadata, _: Subscriber<Value>, _: String, _: Option<Params>);

	/// Unsubscribe from existing Tetsy subscription.
	#[pubsub(subscription = "tetsy_subscription", unsubscribe, name = "tetsy_unsubscribe")]
	fn tetsy_unsubscribe(&self, _: Option<Self::Metadata>, _: SubscriptionId) -> Result<bool>;
}
