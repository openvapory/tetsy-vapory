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

//! Vap PUB-SUB rpc interface.

use tetsy_jsonrpc_core::Result;
use tetsy_jsonrpc_derive::rpc;
use tetsy_jsonrpc_pubsub::{typed, SubscriptionId};

use v1::types::pubsub;

/// Vap PUB-SUB rpc interface.
#[rpc(server)]
pub trait VapPubSub {
	/// RPC Metadata
	type Metadata;

	/// Subscribe to Vap subscription.
	#[pubsub(subscription = "vap_subscription", subscribe, name = "vap_subscribe")]
	fn subscribe(
		&self,
		_: Self::Metadata,
		_: typed::Subscriber<pubsub::Result>,
		_: pubsub::Kind,
		_: Option<pubsub::Params>,
	);

	/// Unsubscribe from existing Vap subscription.
	#[pubsub(subscription = "vap_subscription", unsubscribe, name = "vap_unsubscribe")]
	fn unsubscribe(&self, _: Option<Self::Metadata>, _: SubscriptionId) -> Result<bool>;
}
