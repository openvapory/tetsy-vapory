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

//! Spec seal.

use tetsy_rlp::RlpStream;
use vapory_types::{H64, H256, H520};
use vapjson;

/// Classic vapory seal.
#[derive(Debug)]
pub struct Vapory {
	/// Seal nonce.
	pub nonce: H64,
	/// Seal mix hash.
	pub mix_hash: H256,
}

impl Into<Generic> for Vapory {
	fn into(self) -> Generic {
		let mut s = RlpStream::new_list(2);
		s.append(&self.mix_hash).append(&self.nonce);
		Generic(s.out())
	}
}

/// AuthorityRound seal.
#[derive(Debug)]
pub struct AuthorityRound {
	/// Seal step.
	pub step: usize,
	/// Seal signature.
	pub signature: H520,
}

/// Tendermint seal.
#[derive(Debug)]
pub struct Tendermint {
	/// Seal round.
	pub round: usize,
	/// Proposal seal signature.
	pub proposal: H520,
	/// Precommit seal signatures.
	pub precommits: Vec<H520>,
}

impl Into<Generic> for AuthorityRound {
	fn into(self) -> Generic {
		let mut s = RlpStream::new_list(2);
		s.append(&self.step).append(&self.signature);
		Generic(s.out())
	}
}

impl Into<Generic> for Tendermint {
	fn into(self) -> Generic {
		let mut stream = RlpStream::new_list(3);
		stream
			.append(&self.round)
			.append(&self.proposal)
			.append_list(&self.precommits);
		Generic(stream.out())
	}
}

#[derive(Debug)]
pub struct Generic(pub Vec<u8>);

/// Genesis seal type.
#[derive(Debug)]
pub enum Seal {
	/// Classic vapory seal.
	Vapory(Vapory),
	/// AuthorityRound seal.
	AuthorityRound(AuthorityRound),
	/// Tendermint seal.
	Tendermint(Tendermint),
	/// Generic RLP seal.
	Generic(Generic),
}

impl From<vapjson::spec::Seal> for Seal {
	fn from(s: vapjson::spec::Seal) -> Self {
		match s {
			vapjson::spec::Seal::Vapory(vap) => Seal::Vapory(Vapory {
				nonce: vap.nonce.into(),
				mix_hash: vap.mix_hash.into()
			}),
			vapjson::spec::Seal::AuthorityRound(ar) => Seal::AuthorityRound(AuthorityRound {
				step: ar.step.into(),
				signature: ar.signature.into()
			}),
			vapjson::spec::Seal::Tendermint(tender) => Seal::Tendermint(Tendermint {
				round: tender.round.into(),
				proposal: tender.proposal.into(),
				precommits: tender.precommits.into_iter().map(Into::into).collect()
			}),
			vapjson::spec::Seal::Generic(g) => Seal::Generic(Generic(g.into())),
		}
	}
}

impl Into<Generic> for Seal {
	fn into(self) -> Generic {
		match self {
			Seal::Generic(generic) => generic,
			Seal::Vapory(vap) => vap.into(),
			Seal::AuthorityRound(ar) => ar.into(),
			Seal::Tendermint(tender) => tender.into(),
		}
	}
}
