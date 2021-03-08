//! Transactions pool PUB-SUB rpc interface.

use tetsy_jsonrpc_core::Result;
use tetsy_jsonrpc_pubsub::{typed, SubscriptionId};
use tetsy_jsonrpc_derive::rpc;
use miner::pool::TxStatus;

use vapory_types::H256;

/// Transactions Pool PUB-SUB rpc interface.
#[rpc(server)]
pub trait TransactionsPool {
	/// Pub/Sub Metadata
	type Metadata;

	/// Subscribe to Transactions Pool subscription.
	#[pubsub(subscription = "tetsy_watchTransactionsPool", subscribe, name = "tetsy_watchTransactionsPool")]
	fn subscribe(&self, _: Self::Metadata, _: typed::Subscriber<(H256, TxStatus)>);

	/// Unsubscribe from existing Transactions Pool subscription.
	#[pubsub(subscription = "tetsy_watchTransactionsPool", unsubscribe, name = "tetsy_unwatchTransactionsPool")]
	fn unsubscribe(&self, _: Option<Self::Metadata>, _: SubscriptionId) -> Result<bool>;
}
