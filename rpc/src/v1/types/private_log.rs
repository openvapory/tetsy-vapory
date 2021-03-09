// Copyright 2015-2020 Parity Technologies (UK) Ltd.
// This file is part of Tetsy.

// Tetsy is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Tetsy is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Tetsy.  If not, see <http://www.gnu.org/licenses/>.

use std::time::SystemTime;
use vapory_types::{H160, H256};
use private_tx::{TransactionLog as VapTransactionLog, ValidatorLog as VapValidatorLog, PrivateTxStatus as VapStatus};

/// Current status of the private transaction
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Status {
	/// Private tx was created but no validation received yet
	Created,
	/// Private state not found locally and being retrived from peers
	PrivateStateSync,
	/// Retrieval of the private state failed, transaction not created
	PrivateStateSyncFailed,
	/// Several validators (but not all) validated the transaction
	Validating,
	/// All validators validated the private tx
	/// Corresponding public tx was created and added into the pool
	Deployed,
}

impl From<VapStatus> for Status {
	fn from(c: VapStatus) -> Self {
		match c {
			VapStatus::Created => Status::Created,
			VapStatus::PrivateStateSync => Status::PrivateStateSync,
			VapStatus::PrivateStateSyncFailed => Status::PrivateStateSyncFailed,
			VapStatus::Validating => Status::Validating,
			VapStatus::Deployed => Status::Deployed,
		}
	}
}

/// Information about private tx validation
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidatorLog {
	/// Account of the validator
	pub account: H160,
	/// Validation timestamp, None, if the transaction is not validated yet
	pub validation_timestamp: Option<u64>,
}

impl From<VapValidatorLog> for ValidatorLog {
	fn from(r: VapValidatorLog) -> Self {
		ValidatorLog {
			account: r.account,
			validation_timestamp: r.validation_timestamp.map(|t| t.duration_since(SystemTime::UNIX_EPOCH).unwrap_or_default().as_secs()),
		}
	}
}

/// Information about the private transaction
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PrivateTransactionLog {
	/// Original signed transaction hash (used as a source for private tx)
	pub tx_hash: H256,
	/// Current status of the private transaction
	pub status: Status,
	/// Creation timestamp
	pub creation_timestamp: u64,
	/// List of validations
	pub validators: Vec<ValidatorLog>,
	/// Timestamp of the resulting public tx deployment
	pub deployment_timestamp: Option<u64>,
	/// Hash of the resulting public tx
	pub public_tx_hash: Option<H256>,
}

impl From<VapTransactionLog> for PrivateTransactionLog {
	fn from(r: VapTransactionLog) -> Self {
		PrivateTransactionLog {
			tx_hash: r.tx_hash,
			status: r.status.into(),
			creation_timestamp: r.creation_timestamp.duration_since(SystemTime::UNIX_EPOCH).unwrap_or_default().as_secs(),
			validators: r.validators.into_iter().map(Into::into).collect(),
			deployment_timestamp: r.deployment_timestamp.map(|t| t.duration_since(SystemTime::UNIX_EPOCH).unwrap_or_default().as_secs()),
			public_tx_hash: r.public_tx_hash,
		}
	}
}

