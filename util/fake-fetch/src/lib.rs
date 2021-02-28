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

extern crate tetsy_fetch;
extern crate hyper;
extern crate futures;

use hyper::{StatusCode, Body};
use futures::{future, future::FutureResult};
use tetsy_fetch::{Fetch, Url, Request};

#[derive(Clone, Default)]
pub struct FakeFetch<T> where T: Clone + Send + Sync {
	val: Option<T>,
}

impl<T> FakeFetch<T> where T: Clone + Send + Sync {
	pub fn new(t: Option<T>) -> Self {
		FakeFetch { val : t }
	}
}

impl<T: 'static> Fetch for FakeFetch<T> where T: Clone + Send+ Sync {
	type Result = FutureResult<tetsy_fetch::Response, tetsy_fetch::Error>;

	fn fetch(&self, request: Request, abort: tetsy_fetch::Abort) -> Self::Result {
		let u = request.url().clone();
		future::ok(if self.val.is_some() {
			let r = hyper::Response::new("Some content".into());
			tetsy_fetch::client::Response::new(u, r, abort)
		} else {
			let r = hyper::Response::builder()
				.status(StatusCode::NOT_FOUND)
				.body(Body::empty()).expect("Nothing to parse, can not fail; qed");
			tetsy_fetch::client::Response::new(u, r, abort)
		})
	}

	fn get(&self, url: &str, abort: tetsy_fetch::Abort) -> Self::Result {
		let url: Url = match url.parse() {
			Ok(u) => u,
			Err(e) => return future::err(e.into())
		};
		self.fetch(Request::get(url), abort)
	}

	fn post(&self, url: &str, abort: tetsy_fetch::Abort) -> Self::Result {
		let url: Url = match url.parse() {
			Ok(u) => u,
			Err(e) => return future::err(e.into())
		};
		self.fetch(Request::post(url), abort)
	}
}
