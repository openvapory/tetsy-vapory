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

//! Vapory key-management.

#![warn(missing_docs)]

extern crate dir;
extern crate itertools;
extern crate libc;
extern crate parking_lot;
extern crate rand;
extern crate rustc_hex;
extern crate serde;
extern crate serde_json;
extern crate smallvec;
extern crate time;
extern crate tiny_keccak;
extern crate tempdir;

extern crate tetsy_crypto as crypto;
extern crate vapory_types;
extern crate vapkey as vapkey;
extern crate tetsy_wordlist;

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

#[cfg(test)]
#[macro_use]
extern crate matches;

pub mod accounts_dir;

mod account;
mod json;

mod error;
mod vapstore;
mod import;
mod presale;
mod random;
mod secret_store;

pub use self::account::{SafeAccount, Crypto};
pub use self::error::Error;
pub use self::vapstore::{VapStore, VapMultiStore};
pub use self::import::{import_account, import_accounts, read_gvap_accounts};
pub use self::json::OpaqueKeyFile as KeyFile;
pub use self::presale::PresaleWallet;
pub use self::secret_store::{
	SecretVaultRef, StoreAccountRef, SimpleSecretStore, SecretStore,
	Derivation, IndexDerivation,
};
pub use self::random::random_string;
pub use self::tetsy_wordlist::random_phrase;

/// An opaque wrapper for secret.
pub struct OpaqueSecret(crypto::publickey::Secret);

// Additional converters for Address
use crypto::publickey::Address;

impl Into<json::H160> for Address {
	fn into(self) -> json::H160 {
		let a: [u8; 20] = self.into();
		From::from(a)
	}
}

impl From<json::H160> for Address {
	fn from(json: json::H160) -> Self {
		let a: [u8; 20] = json.into();
		From::from(a)
	}
}

impl<'a> From<&'a json::H160> for Address {
	fn from(json: &'a json::H160) -> Self {
		let mut a = [0u8; 20];
		a.copy_from_slice(json);
		From::from(a)
	}
}

