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

//! Tetsy Vapory JSON-RPC Servers (WS, HTTP, IPC).

#![warn(missing_docs, unused_extern_crates)]
#![cfg_attr(feature = "cargo-clippy", warn(clippy::all, clippy::pedantic))]
#![cfg_attr(
	feature = "cargo-clippy",
	allow(
		// things are often more readable this way
		clippy::cast_lossless,
		clippy::module_name_repetitions,
		clippy::single_match_else,
		clippy::type_complexity,
		clippy::use_self,
		// not practical
		clippy::match_bool,
		clippy::needless_pass_by_value,
		clippy::similar_names,
		// don't require markdown syntax for docs
		clippy::doc_markdown,
	),
	warn(clippy::indexing_slicing)
)]

#[macro_use]
extern crate futures;

extern crate ansi_term;
extern crate cid;
extern crate itertools;
extern crate machine;
extern crate multihash;
extern crate order_stat;
extern crate parking_lot;
extern crate rand;
extern crate rustc_hex;
extern crate semver;
extern crate serde;
extern crate serde_json;
extern crate tokio_timer;
extern crate transient_hashmap;

extern crate tetsy_jsonrpc_core;
extern crate tetsy_jsonrpc_derive;
extern crate tetsy_jsonrpc_http_server as http;
extern crate tetsy_jsonrpc_ipc_server as ipc;
extern crate tetsy_jsonrpc_pubsub;

extern crate client_traits;
extern crate common_types as types;
extern crate vapash;
extern crate vapcore;
extern crate fastmap;
extern crate tetsy_bytes as bytes;
extern crate tetsy_crypto as crypto;
extern crate vapcore_light as light;
extern crate vapcore_logger;
extern crate vapcore_miner as miner;
extern crate vapcore_network as network;
extern crate vapcore_private_tx;
extern crate vapcore_sync as sync;
extern crate vapory_types;
extern crate vapkey;
extern crate vapstore;
extern crate tetsy_fetch;
extern crate tetsy_keccak_hash as hash;
extern crate tetsy_runtime;
extern crate tetsy_updater as updater;
extern crate tetsy_version as version;
extern crate vip_712;
extern crate tetsy_rlp;
extern crate account_state;

extern crate tetsy_stats;
extern crate snapshot;
extern crate tempdir;
extern crate vapcore_trace;
extern crate tetsy_vm;

#[cfg(any(test, feature = "vapcore-accounts"))]
extern crate vapcore_accounts as accounts;

#[cfg(any(test, feature = "vapcore-accounts"))]
extern crate tiny_keccak;

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

#[cfg(test)]
extern crate rand_xorshift;

#[cfg(test)]
extern crate engine;

#[cfg(test)]
extern crate vapjson;
#[cfg(test)]
extern crate transaction_pool as txpool;

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

#[cfg(test)]
#[macro_use]
extern crate tetsy_macros;

#[cfg(test)]
extern crate fake_fetch;

#[cfg(test)]
extern crate vapcore_io as io;

#[cfg(test)]
extern crate spec;
#[cfg(test)]
extern crate verification;

pub extern crate tetsy_jsonrpc_ws_server as ws;

mod authcodes;
mod http_common;
pub mod v1;

pub mod tests;

pub use tetsy_jsonrpc_core::{FutureOutput, FutureResult, FutureResponse, FutureRpcResult};
pub use tetsy_jsonrpc_pubsub::Session as PubSubSession;
pub use ipc::{
	MetaExtractor as IpcMetaExtractor,
	RequestContext as IpcRequestContext,
	SecurityAttributes,
	Server as IpcServer,
};
pub use http::{
	hyper,
	RequestMiddleware, RequestMiddlewareAction,
	AccessControlAllowOrigin, Host, DomainsValidation, cors::AccessControlAllowHeaders
};

pub use v1::{NetworkSettings, Metadata, Origin, informant, dispatch, signer};
pub use v1::block_import::{is_major_importing_or_waiting};
pub use v1::PubSubSyncStatus;
pub use v1::extractors::{RpcExtractor, WsExtractor, WsStats, WsDispatcher};
pub use authcodes::{AuthCodes, TimeProvider};
pub use http_common::HttpMetaExtractor;

use std::net::SocketAddr;

/// RPC HTTP Server instance
pub type HttpServer = http::Server;

/// Start http server asynchronously and returns result with `Server` handle on success or an error.
pub fn start_http<M, S, H, T>(
	addr: &SocketAddr,
	cors_domains: http::DomainsValidation<http::AccessControlAllowOrigin>,
	allowed_hosts: http::DomainsValidation<http::Host>,
	handler: H,
	extractor: T,
	threads: usize,
	max_payload: usize,
	keep_alive: bool,
) -> ::std::io::Result<HttpServer> where
	M: tetsy_jsonrpc_core::Metadata,
	S: tetsy_jsonrpc_core::Middleware<M>,
	H: Into<tetsy_jsonrpc_core::MetaIoHandler<M, S>>,
	T: HttpMetaExtractor<Metadata=M>,
{
	let extractor = http_common::MetaExtractor::new(extractor);
	Ok(http::ServerBuilder::with_meta_extractor(handler, extractor)
		.keep_alive(keep_alive)
		.threads(threads)
		.cors(cors_domains)
		.allowed_hosts(allowed_hosts)
		.health_api(("/api/health", "tetsy_nodeStatus"))
		.cors_allow_headers(AccessControlAllowHeaders::Any)
		.max_request_body_size(max_payload * 1024 * 1024)
		.start_http(addr)?)
}

/// Same as `start_http`, but takes an additional `middleware` parameter that is introduced as a
/// hyper middleware.
pub fn start_http_with_middleware<M, S, H, T, R>(
	addr: &SocketAddr,
	cors_domains: http::DomainsValidation<http::AccessControlAllowOrigin>,
	allowed_hosts: http::DomainsValidation<http::Host>,
	handler: H,
	extractor: T,
	middleware: R,
	threads: usize,
	max_payload: usize,
	keep_alive: bool,
) -> ::std::io::Result<HttpServer> where
	M: tetsy_jsonrpc_core::Metadata,
	S: tetsy_jsonrpc_core::Middleware<M>,
	H: Into<tetsy_jsonrpc_core::MetaIoHandler<M, S>>,
	T: HttpMetaExtractor<Metadata=M>,
	R: RequestMiddleware,
{
	let extractor = http_common::MetaExtractor::new(extractor);
	Ok(http::ServerBuilder::with_meta_extractor(handler, extractor)
		.keep_alive(keep_alive)
		.threads(threads)
		.cors(cors_domains)
		.allowed_hosts(allowed_hosts)
		.cors_allow_headers(AccessControlAllowHeaders::Any)
		.max_request_body_size(max_payload * 1024 * 1024)
		.request_middleware(middleware)
		.start_http(addr)?)
}

/// Start ipc server asynchronously and returns result with `Server` handle on success or an error.
pub fn start_ipc<M, S, H, T>(
	addr: &str,
	handler: H,
	extractor: T,
	chmod: u16
) -> ::std::io::Result<ipc::Server> where
	M: tetsy_jsonrpc_core::Metadata,
	S: tetsy_jsonrpc_core::Middleware<M>,
	H: Into<tetsy_jsonrpc_core::MetaIoHandler<M, S>>,
	T: IpcMetaExtractor<M>,
{
	let attr = SecurityAttributes::empty()
		.set_mode(chmod as _)?;

	ipc::ServerBuilder::with_meta_extractor(handler, extractor)
		.set_security_attributes(attr)
		.start(addr)
}

/// Start WS server and return `Server` handle.
pub fn start_ws<M, S, H, T, U, V>(
	addr: &SocketAddr,
	handler: H,
	allowed_origins: ws::DomainsValidation<ws::Origin>,
	allowed_hosts: ws::DomainsValidation<ws::Host>,
	max_connections: usize,
	extractor: T,
	middleware: V,
	stats: U,
) -> Result<ws::Server, ws::Error> where
	M: tetsy_jsonrpc_core::Metadata,
	S: tetsy_jsonrpc_core::Middleware<M>,
	H: Into<tetsy_jsonrpc_core::MetaIoHandler<M, S>>,
	T: ws::MetaExtractor<M>,
	U: ws::SessionStats,
	V: ws::RequestMiddleware,
{
	ws::ServerBuilder::with_meta_extractor(handler, extractor)
		.request_middleware(middleware)
		.allowed_origins(allowed_origins)
		.allowed_hosts(allowed_hosts)
		.max_connections(max_connections)
		.session_stats(stats)
		.start(addr)
}
