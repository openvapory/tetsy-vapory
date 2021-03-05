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

//! Transaction execution format module.

use vapcore_trace::{VMTrace, FlatTrace};
use common_types::{
	engines::mashina,
	errors::ExecutionError,
};

/// /// Transaction execution receipt, parametrised with convenient defaults.
pub type Executed = mashina::Executed<FlatTrace, VMTrace>;

/// Transaction execution result.
pub type ExecutionResult = Result<Box<Executed>, ExecutionError>;
