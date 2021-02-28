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

//! Vap rpc interface.
use jsonrpc_core::{Result, BoxFuture};
use jsonrpc_derive::rpc;
use vapory_types::{H64, H160, H256, U64, U256};

use v1::types::{RichBlock, BlockNumber, Bytes, CallRequest, Filter, FilterChanges, Index, VapAccount};
use v1::types::{Log, Receipt, SyncStatus, Transaction, Work};

/// Vap rpc interface.
#[rpc(server)]
pub trait Vap {
	/// RPC Metadata
	type Metadata;

	/// Returns protocol version encoded as a string (quotes are necessary).
	#[rpc(name = "vap_protocolVersion")]
	fn protocol_version(&self) -> Result<String>;

	/// Returns an object with data about the sync status or false. (wtf?)
	#[rpc(name = "vap_syncing")]
	fn syncing(&self) -> Result<SyncStatus>;

	/// Returns the number of hashes per second that the node is mining with.
	#[rpc(name = "vap_hashrate")]
	fn hashrate(&self) -> Result<U256>;

	/// Returns block author.
	#[rpc(name = "vap_coinbase")]
	fn author(&self) -> Result<H160>;

	/// Returns true if client is actively mining new blocks.
	#[rpc(name = "vap_mining")]
	fn is_mining(&self) -> Result<bool>;

	/// Returns the chain ID used for transaction signing at the
	/// current best block. None is returned if not
	/// available.
	#[rpc(name = "vap_chainId")]
	fn chain_id(&self) -> Result<Option<U64>>;

	/// Returns current gas_price.
	#[rpc(name = "vap_gasPrice")]
	fn gas_price(&self) -> BoxFuture<U256>;

	/// Returns accounts list.
	#[rpc(name = "vap_accounts")]
	fn accounts(&self) -> Result<Vec<H160>>;

	/// Returns highest block number.
	#[rpc(name = "vap_blockNumber")]
	fn block_number(&self) -> Result<U256>;

	/// Returns balance of the given account.
	#[rpc(name = "vap_getBalance")]
	fn balance(&self, _: H160, _: Option<BlockNumber>) -> BoxFuture<U256>;

	/// Returns the account- and storage-values of the specified account including the Merkle-proof
	#[rpc(name = "vap_getProof")]
	fn proof(&self, _: H160, _: Vec<H256>, _: Option<BlockNumber>) -> BoxFuture<VapAccount>;

	/// Returns content of the storage at given address.
	#[rpc(name = "vap_getStorageAt")]
	fn storage_at(&self, _: H160, _: U256, _: Option<BlockNumber>) -> BoxFuture<H256>;

	/// Returns block with given hash.
	#[rpc(name = "vap_getBlockByHash")]
	fn block_by_hash(&self, _: H256, _: bool) -> BoxFuture<Option<RichBlock>>;

	/// Returns block with given number.
	#[rpc(name = "vap_getBlockByNumber")]
	fn block_by_number(&self, _: BlockNumber, _: bool) -> BoxFuture<Option<RichBlock>>;

	/// Returns the number of transactions sent from given address at given time (block number).
	#[rpc(name = "vap_getTransactionCount")]
	fn transaction_count(&self, _: H160, _: Option<BlockNumber>) -> BoxFuture<U256>;

	/// Returns the number of transactions in a block with given hash.
	#[rpc(name = "vap_getBlockTransactionCountByHash")]
	fn block_transaction_count_by_hash(&self, _: H256) -> BoxFuture<Option<U256>>;

	/// Returns the number of transactions in a block with given block number.
	#[rpc(name = "vap_getBlockTransactionCountByNumber")]
	fn block_transaction_count_by_number(&self, _: BlockNumber) -> BoxFuture<Option<U256>>;

	/// Returns the number of uncles in a block with given hash.
	#[rpc(name = "vap_getUncleCountByBlockHash")]
	fn block_uncles_count_by_hash(&self, _: H256) -> BoxFuture<Option<U256>>;

	/// Returns the number of uncles in a block with given block number.
	#[rpc(name = "vap_getUncleCountByBlockNumber")]
	fn block_uncles_count_by_number(&self, _: BlockNumber) -> BoxFuture<Option<U256>>;

	/// Returns the code at given address at given time (block number).
	#[rpc(name = "vap_getCode")]
	fn code_at(&self, _: H160, _: Option<BlockNumber>) -> BoxFuture<Bytes>;

	/// Sends signed transaction, returning its hash.
	#[rpc(name = "vap_sendRawTransaction")]
	fn send_raw_transaction(&self, _: Bytes) -> Result<H256>;

	/// @alias of `vap_sendRawTransaction`.
	#[rpc(name = "vap_submitTransaction")]
	fn submit_transaction(&self, _: Bytes) -> Result<H256>;

	/// Call contract, returning the output data.
	#[rpc(name = "vap_call")]
	fn call(&self, _: CallRequest, _: Option<BlockNumber>) -> BoxFuture<Bytes>;

	/// Estimate gas needed for execution of given contract.
	#[rpc(name = "vap_estimateGas")]
	fn estimate_gas(&self, _: CallRequest, _: Option<BlockNumber>) -> BoxFuture<U256>;

	/// Get transaction by its hash.
	#[rpc(name = "vap_getTransactionByHash")]
	fn transaction_by_hash(&self, _: H256) -> BoxFuture<Option<Transaction>>;

	/// Returns transaction at given block hash and index.
	#[rpc(name = "vap_getTransactionByBlockHashAndIndex")]
	fn transaction_by_block_hash_and_index(&self, _: H256, _: Index) -> BoxFuture<Option<Transaction>>;

	/// Returns transaction by given block number and index.
	#[rpc(name = "vap_getTransactionByBlockNumberAndIndex")]
	fn transaction_by_block_number_and_index(&self, _: BlockNumber, _: Index) -> BoxFuture<Option<Transaction>>;

	/// Returns transaction receipt by transaction hash.
	#[rpc(name = "vap_getTransactionReceipt")]
	fn transaction_receipt(&self, _: H256) -> BoxFuture<Option<Receipt>>;

	/// Returns an uncles at given block and index.
	#[rpc(name = "vap_getUncleByBlockHashAndIndex")]
	fn uncle_by_block_hash_and_index(&self, _: H256, _: Index) -> BoxFuture<Option<RichBlock>>;

	/// Returns an uncles at given block and index.
	#[rpc(name = "vap_getUncleByBlockNumberAndIndex")]
	fn uncle_by_block_number_and_index(&self, _: BlockNumber, _: Index) -> BoxFuture<Option<RichBlock>>;

	/// Returns available compilers.
	/// @deprecated
	#[rpc(name = "vap_getCompilers")]
	fn compilers(&self) -> Result<Vec<String>>;

	/// Compiles lll code.
	/// @deprecated
	#[rpc(name = "vap_compileLLL")]
	fn compile_lll(&self, _: String) -> Result<Bytes>;

	/// Compiles solidity.
	/// @deprecated
	#[rpc(name = "vap_compileSolidity")]
	fn compile_solidity(&self, _: String) -> Result<Bytes>;

	/// Compiles serpent.
	/// @deprecated
	#[rpc(name = "vap_compileSerpent")]
	fn compile_serpent(&self, _: String) -> Result<Bytes>;

	/// Returns logs matching given filter object.
	#[rpc(name = "vap_getLogs")]
	fn logs(&self, _: Filter) -> BoxFuture<Vec<Log>>;

	/// Returns the hash of the current block, the seedHash, and the boundary condition to be met.
	#[rpc(name = "vap_getWork")]
	fn work(&self, _: Option<u64>) -> Result<Work>;

	/// Used for submitting a proof-of-work solution.
	#[rpc(name = "vap_submitWork")]
	fn submit_work(&self, _: H64, _: H256, _: H256) -> Result<bool>;

	/// Used for submitting mining hashrate.
	#[rpc(name = "vap_submitHashrate")]
	fn submit_hashrate(&self, _: U256, _: H256) -> Result<bool>;
}

/// Vap filters rpc api (polling).
// TODO: do filters api properly
#[rpc(server)]
pub trait VapFilter {
	/// Returns id of new filter.
	#[rpc(name = "vap_newFilter")]
	fn new_filter(&self, _: Filter) -> Result<U256>;

	/// Returns id of new block filter.
	#[rpc(name = "vap_newBlockFilter")]
	fn new_block_filter(&self) -> Result<U256>;

	/// Returns id of new block filter.
	#[rpc(name = "vap_newPendingTransactionFilter")]
	fn new_pending_transaction_filter(&self) -> Result<U256>;

	/// Returns filter changes since last poll.
	#[rpc(name = "vap_getFilterChanges")]
	fn filter_changes(&self, _: Index) -> BoxFuture<FilterChanges>;

	/// Returns all logs matching given filter (in a range 'from' - 'to').
	#[rpc(name = "vap_getFilterLogs")]
	fn filter_logs(&self, _: Index) -> BoxFuture<Vec<Log>>;

	/// Uninstalls filter.
	#[rpc(name = "vap_uninstallFilter")]
	fn uninstall_filter(&self, _: Index) -> Result<bool>;
}
