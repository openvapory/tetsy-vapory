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

//! Key pair with signing ability

use std::sync::Arc;
use accounts::AccountProvider;
use vapkey::Password;
use tetsy_crypto::publickey::public_to_address;
use vapory_types::{H256, Address, Public};
use tetsy_crypto::publickey::{Signature, Error as VapKeyError};
use vapcore_secretstore::SigningKeyPair;

pub struct KeyStoreNodeKeyPair {
	account_provider: Arc<AccountProvider>,
	address: Address,
	public: Public,
	password: Password,
}

impl KeyStoreNodeKeyPair {
	pub fn new(account_provider: Arc<AccountProvider>, address: Address, password: Password) -> Result<Self, VapKeyError> {
		let public = account_provider.account_public(address.clone(), &password).map_err(|e| VapKeyError::Custom(format!("{}", e)))?;
		Ok(KeyStoreNodeKeyPair {
			account_provider,
			address,
			public,
			password,
		})
	}
}

impl SigningKeyPair for KeyStoreNodeKeyPair {
	fn public(&self) -> &Public {
		&self.public
	}

	fn address(&self) -> Address {
		public_to_address(&self.public)
	}

	fn sign(&self, data: &H256) -> Result<Signature, VapKeyError> {
		self.account_provider.sign(self.address.clone(), Some(self.password.clone()), data.clone())
			.map_err(|e| VapKeyError::Custom(format!("{}", e)))
	}
}
