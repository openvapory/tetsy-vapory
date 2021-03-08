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

//! Tetsy-specific rpc interface for operations altering the settings.

use vapory_types::{H160, H256, U256};
use tetsy_jsonrpc_core::{BoxFuture, Result};
use tetsy_jsonrpc_derive::rpc;

use v1::types::{Bytes, ReleaseInfo, Transaction};

/// Tetsy-specific rpc interface for operations altering the account-related settings.
#[rpc(server)]
pub trait TetsySetAccounts {
	/// Sets account for signing consensus messages.
	#[rpc(name = "tetsy_setEngineSigner")]
	fn set_engine_signer(&self, _: H160, _: String) -> Result<bool>;
}

/// Tetsy-specific rpc interface for operations altering the settings.
#[rpc(server)]
pub trait TetsySet {
	/// Sets new minimal gas price for mined blocks.
	#[rpc(name = "tetsy_setMinGasPrice")]
	fn set_min_gas_price(&self, _: U256) -> Result<bool>;

	/// Sets new gas floor target for mined blocks.
	#[rpc(name = "tetsy_setGasFloorTarget")]
	fn set_gas_floor_target(&self, _: U256) -> Result<bool>;

	/// Sets new gas ceiling target for mined blocks.
	#[rpc(name = "tetsy_setGasCeilTarget")]
	fn set_gas_ceil_target(&self, _: U256) -> Result<bool>;

	/// Sets new extra data for mined blocks.
	#[rpc(name = "tetsy_setExtraData")]
	fn set_extra_data(&self, _: Bytes) -> Result<bool>;

	/// Sets new author for mined block.
	#[rpc(name = "tetsy_setAuthor")]
	fn set_author(&self, _: H160) -> Result<bool>;

	/// Sets the secret of engine signer account.
	#[rpc(name = "tetsy_setEngineSignerSecret")]
	fn set_engine_signer_secret(&self, _: H256) -> Result<bool>;

	/// Unsets the engine signer account address.
	#[rpc(name = "tetsy_clearEngineSigner")]
	fn clear_engine_signer(&self) -> Result<bool>;

	/// Sets the limits for transaction queue.
	#[rpc(name = "tetsy_setTransactionsLimit")]
	fn set_transactions_limit(&self, _: usize) -> Result<bool>;

	/// Sets the maximum amount of gas a single transaction may consume.
	#[rpc(name = "tetsy_setMaxTransactionGas")]
	fn set_tx_gas_limit(&self, _: U256) -> Result<bool>;

	/// Add a reserved peer.
	#[rpc(name = "tetsy_addReservedPeer")]
	fn add_reserved_peer(&self, _: String) -> Result<bool>;

	/// Remove a reserved peer.
	#[rpc(name = "tetsy_removeReservedPeer")]
	fn remove_reserved_peer(&self, _: String) -> Result<bool>;

	/// Drop all non-reserved peers.
	#[rpc(name = "tetsy_dropNonReservedPeers")]
	fn drop_non_reserved_peers(&self) -> Result<bool>;

	/// Accept non-reserved peers (default behavior)
	#[rpc(name = "tetsy_acceptNonReservedPeers")]
	fn accept_non_reserved_peers(&self) -> Result<bool>;

	/// Start the network.
	///
	/// @deprecated - Use `set_mode("active")` instead.
	#[rpc(name = "tetsy_startNetwork")]
	fn start_network(&self) -> Result<bool>;

	/// Stop the network.
	///
	/// @deprecated - Use `set_mode("offline")` instead.
	#[rpc(name = "tetsy_stopNetwork")]
	fn stop_network(&self) -> Result<bool>;

	/// Set the mode. Argument must be one of: "active", "passive", "dark", "offline".
	#[rpc(name = "tetsy_setMode")]
	fn set_mode(&self, _: String) -> Result<bool>;

	/// Set the network spec. Argument must be one of pre-configured chains or a filename.
	#[rpc(name = "tetsy_setChain")]
	fn set_spec_name(&self, _: String) -> Result<bool>;

	/// Hash a file content under given URL.
	#[rpc(name = "tetsy_hashContent")]
	fn hash_content(&self, _: String) -> BoxFuture<H256>;

	/// Is there a release ready for install?
	#[rpc(name = "tetsy_upgradeReady")]
	fn upgrade_ready(&self) -> Result<Option<ReleaseInfo>>;

	/// Execute a release which is ready according to upgrade_ready().
	#[rpc(name = "tetsy_executeUpgrade")]
	fn execute_upgrade(&self) -> Result<bool>;

	/// Removes transaction from transaction queue.
	/// Makes sense only for transactions that were not propagated to other peers yet
	/// like scheduled transactions or transactions in future.
	/// It might also work for some local transactions with to low gas price
	/// or excessive gas limit that are not accepted by other peers whp.
	/// Returns `true` when transaction was removed, `false` if it was not found.
	#[rpc(name = "tetsy_removeTransaction")]
	fn remove_transaction(&self, _: H256) -> Result<Option<Transaction>>;
}
