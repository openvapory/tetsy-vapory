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

#![warn(missing_docs)]

//! A simple client to get the current VAP price using an external API.

use std::{cmp, fmt, io, str};
use tetsy_fetch::{Client as FetchClient, Fetch};
use futures::{Future, Stream};
use log::warn;
use tetsy_runtime::Executor;
use serde_json::Value;

pub use fetch;

/// Current VAP price information.
#[derive(Debug)]
pub struct PriceInfo {
	/// Current VAP price in USD.
	pub vapusd: f32,
}

/// A client to get the current VAP price using an external API.
pub struct Client<F = FetchClient> {
	pool: Executor,
	api_endpoint: String,
	fetch: F,
}

impl<F> fmt::Debug for Client<F> {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		fmt.debug_struct("price_info::Client")
		   .field("api_endpoint", &self.api_endpoint)
		   .finish()
	}
}

impl<F> cmp::PartialEq for Client<F> {
	fn eq(&self, other: &Client<F>) -> bool {
		self.api_endpoint == other.api_endpoint
	}
}

impl<F: Fetch> Client<F> {
	/// Creates a new instance of the `Client` given a `tetsy_fetch::Client`.
	pub fn new(fetch: F, pool: Executor, api_endpoint: String) -> Client<F> {
		Client { pool, api_endpoint, fetch }
	}

	/// Gets the current VAP price and calls `set_price` with the result.
	pub fn get<G: FnOnce(PriceInfo) + Sync + Send + 'static>(&self, set_price: G) {
		let future = self.fetch.get(&self.api_endpoint, tetsy_fetch::Abort::default())
			.and_then(|response| response.concat2())
			.and_then(move |body| {
				let body_str = str::from_utf8(&body).ok();
				let value: Option<Value> = body_str.and_then(|s| serde_json::from_str(s).ok());

				let vapusd = value
					.as_ref()
					.and_then(|value| value.pointer("/result/vapusd"))
					.and_then(|obj| obj.as_str())
					.and_then(|s| s.parse().ok());

				match vapusd {
					Some(vapusd) => {
						set_price(PriceInfo { vapusd });
						Ok(())
					},
					None => {
						let msg = format!("Unexpected response: {}", body_str.unwrap_or_default());
						let err = io::Error::new(io::ErrorKind::Other, msg);
						Err(tetsy_fetch::Error::Io(err))
					}
				}
			})
			.map_err(|err| {
				warn!("Failed to auto-update latest VAP price: {:?}", err);
			});
		self.pool.spawn(future)
	}
}

#[cfg(test)]
mod test {
	use std::sync::{
		Arc, atomic::{AtomicBool, Ordering}
	};
	use fake_fetch::FakeFetch;
	use tetsy_runtime::{Runtime, Executor};
	use super::Client;

	fn price_info_ok(response: &str, executor: Executor) -> Client<FakeFetch<String>> {
		Client::new(FakeFetch::new(Some(response.to_owned())), executor, "fake_endpoint".to_owned())
	}

	fn price_info_not_found(executor: Executor) -> Client<FakeFetch<String>> {
		Client::new(FakeFetch::new(None::<String>), executor, "fake_endpoint".to_owned())
	}

	#[test]
	fn should_get_price_info() {
		let runtime = Runtime::with_thread_count(1);

		// given
		let response = r#"{
			"status": "1",
			"message": "OK",
			"result": {
				"vapbtc": "0.0891",
				"vapbtc_timestamp": "1499894236",
				"vapusd": "209.55",
				"vapusd_timestamp": "1499894229"
			}
		}"#;

		let price_info = price_info_ok(response, runtime.executor());

		// when
		price_info.get(|price| {

			// then
			assert_eq!(price.vapusd, 209.55);
		});
	}

	#[test]
	fn should_not_call_set_price_if_response_is_malformed() {
		let runtime = Runtime::with_thread_count(1);

		// given
		let response = "{}";

		let price_info = price_info_ok(response, runtime.executor());
		let b = Arc::new(AtomicBool::new(false));

		// when
		let bb = b.clone();
		price_info.get(move |_| {
			bb.store(true, Ordering::Relaxed);
		});

		// then
		assert_eq!(b.load(Ordering::Relaxed), false);
	}

	#[test]
	fn should_not_call_set_price_if_response_is_invalid() {
		let runtime = Runtime::with_thread_count(1);

		// given
		let price_info = price_info_not_found(runtime.executor());
		let b = Arc::new(AtomicBool::new(false));

		// when
		let bb = b.clone();
		price_info.get(move |_| {
			bb.store(true, Ordering::Relaxed);
		});

		// then
		assert_eq!(b.load(Ordering::Relaxed), false);
	}
}
