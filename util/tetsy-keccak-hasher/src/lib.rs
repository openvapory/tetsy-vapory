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

//! Hasher implementation for the Keccak-256 hash

#![warn(missing_docs)]

use tetsy_hash_db::Hasher;
use vapory_types::H256;
use tiny_keccak::Keccak;
use tetsy_plain_hasher::PlainHasher;

/// Concrete `Hasher` impl for the Keccak-256 hash
#[derive(Default, Debug, Clone, PartialEq)]
pub struct KeccakHasher;
impl Hasher for KeccakHasher {
	type Out = H256;
	type StdHasher = PlainHasher;
	const LENGTH: usize = 32;
	fn hash(x: &[u8]) -> Self::Out {
		let mut out = [0; 32];
		Keccak::keccak256(x, &mut out);
		out.into()
	}
}
