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

//! Vapory rpc interface implementation.

mod debug;
mod vap;
mod vap_filter;
mod vap_pubsub;
mod net;
mod tetsy;
#[cfg(any(test, feature = "accounts"))]
mod tetsy_accounts;
mod tetsy_set;
#[cfg(any(test, feature = "accounts"))]
mod personal;
mod private;
mod pubsub;
mod rpc;
#[cfg(any(test, feature = "accounts"))]
mod secretstore;
mod signer;
mod signing;
mod signing_unsafe;
mod traces;
mod transactions_pool;
mod web3;

pub mod light;

pub use self::debug::DebugClient;
pub use self::vap::{VapClient, VapClientOptions};
pub use self::vap_filter::VapFilterClient;
pub use self::vap_pubsub::VapPubSubClient;
pub use self::transactions_pool::TransactionsPoolClient;
pub use self::net::NetClient;
pub use self::tetsy::TetsyClient;
#[cfg(any(test, feature = "accounts"))]
pub use self::tetsy_accounts::TetsyAccountsClient;
pub use self::tetsy_set::TetsySetClient;
#[cfg(any(test, feature = "accounts"))]
pub use self::tetsy_set::accounts::TetsySetAccountsClient;
#[cfg(any(test, feature = "accounts"))]
pub use self::personal::PersonalClient;
pub use self::private::PrivateClient;
pub use self::pubsub::PubSubClient;
pub use self::rpc::RpcClient;
#[cfg(any(test, feature = "accounts"))]
pub use self::secretstore::SecretStoreClient;
pub use self::signer::SignerClient;
pub use self::signing::SigningQueueClient;
pub use self::signing_unsafe::SigningUnsafeClient;
pub use self::traces::TracesClient;
pub use self::web3::Web3Client;
