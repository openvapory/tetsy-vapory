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

//! Vvm factory.
//!
use std::sync::Arc;
use tetsy_vm::{Exec, Schedule};
use vapory_types::U256;
use super::tetsy_vm::ActionParams;
use super::interpreter::SharedCache;

/// Vvm factory. Creates appropriate Vvm.
#[derive(Clone)]
pub struct Factory {
	vvm_cache: Arc<SharedCache>,
}

impl Factory {
	/// Create fresh instance of VM
	/// Might choose implementation depending on supplied gas.
	pub fn create(&self, params: ActionParams, schedule: &Schedule, depth: usize) -> Box<dyn Exec> {
		if Self::can_fit_in_usize(&params.gas) {
			Box::new(super::interpreter::Interpreter::<usize>::new(params, self.vvm_cache.clone(), schedule, depth))
		} else {
			Box::new(super::interpreter::Interpreter::<U256>::new(params, self.vvm_cache.clone(), schedule, depth))
		}
	}

	/// Create new instance of a factory, with a size in bytes
	/// for caching jump destinations.
	pub fn new(cache_size: usize) -> Self {
		Factory {
			vvm_cache: Arc::new(SharedCache::new(cache_size)),
		}
	}

	fn can_fit_in_usize(gas: &U256) -> bool {
		gas == &U256::from(gas.low_u64() as usize)
	}
}

impl Default for Factory {
	/// Returns native rust vvm factory
	fn default() -> Factory {
		Factory {
			vvm_cache: Arc::new(SharedCache::default()),
		}
	}
}

#[test]
fn test_create_vm() {
	use tetsy_vm::Ext;
	use tetsy_vm::tests::FakeExt;
	use bytes::Bytes;

	let mut params = ActionParams::default();
	params.code = Some(Arc::new(Bytes::default()));
	let ext = FakeExt::new();
	let _vm = Factory::default().create(params, ext.schedule(), ext.depth());
}

/// Create tests by injecting different VM factories
#[macro_export]
macro_rules! vvm_test(
	($name_test: ident: $name_int: ident) => {
		#[test]
		fn $name_int() {
			$name_test(Factory::new(1024 * 32));
		}
	}
);

/// Create ignored tests by injecting different VM factories
#[macro_export]
macro_rules! vvm_test_ignore(
	($name_test: ident: $name_int: ident) => {
		#[test]
		#[ignore]
		#[cfg(feature = "ignored-tests")]
		fn $name_int() {
			$name_test(Factory::new(1024 * 32));
		}
	}
);
