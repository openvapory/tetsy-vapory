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

use private_tx::{Receipt as VapPrivateReceipt};
use vapory_types::{H160, H256};
use v1::types::TransactionRequest;

/// Receipt
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PrivateTransactionReceipt {
	/// Transaction Hash
	pub transaction_hash: H256,
	/// Private contract address
	pub contract_address: H160,
	/// Status code
	#[serde(rename = "status")]
	pub status_code: u8,
}

impl From<VapPrivateReceipt> for PrivateTransactionReceipt {
	fn from(r: VapPrivateReceipt) -> Self {
		PrivateTransactionReceipt {
			transaction_hash: r.hash,
			contract_address: r.contract_address,
			status_code: r.status_code,
		}
	}
}

/// Receipt and Transaction
#[derive(Debug, Serialize)]
pub struct PrivateTransactionReceiptAndTransaction {
	/// Receipt
	pub receipt: PrivateTransactionReceipt,
	/// Transaction
	pub transaction: TransactionRequest,
}
