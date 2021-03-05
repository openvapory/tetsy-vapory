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

//! This crate provides a state mashina and the facilities needed to execute transactions and the
//! code contained therein, as well as contract based transaction permissions. All vapory
//! engines embed a `Machine`.

pub mod executed;
pub mod executed_block;
pub mod executive;
pub mod externalities;
pub mod mashina;
pub mod substate;
pub mod transaction_ext;
pub mod tx_filter;

pub use crate::{
	executed_block::ExecutedBlock,
	mashina::Machine
};

#[cfg(any(test, feature = "test-helpers"))]
pub mod test_helpers;
