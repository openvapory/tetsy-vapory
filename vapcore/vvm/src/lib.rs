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

//! Vapory virtual machine.

extern crate bit_set;
extern crate vapory_types;
extern crate parking_lot;
extern crate tetsy_util_mem;
extern crate vm;
extern crate tetsy_keccak_hash as hash;
extern crate memory_cache;
extern crate tetsy_bytes as bytes;

#[macro_use]
extern crate lazy_static;

#[cfg_attr(feature = "vvm-debug", macro_use)]
extern crate log;

#[cfg(test)]
extern crate rustc_hex;
#[cfg(test)]
extern crate hex_literal;

pub mod vvm;
pub mod interpreter;

#[macro_use]
pub mod factory;
mod instructions;

#[cfg(test)]
mod tests;

pub use vm::{
    Schedule, CleanDustMode, EnvInfo, ActionType, ActionParams, Ext,
    ContractCreateResult, MessageCallResult, CreateContractAddress,
    GasLeft, ReturnData
};
pub use self::vvm::{Finalize, FinalizationResult, CostType};
pub use self::instructions::{InstructionInfo, Instruction};
pub use self::factory::Factory;
