// Copyright 2020-2024 Trust Computing GmbH.
// This file is part of Litentry.
//
// Litentry is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Litentry is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Litentry.  If not, see <https://www.gnu.org/licenses/>.

use std::thread;
use tokio::{
	sync::oneshot::{channel, error::RecvError},
	task::LocalSet,
};
use warp::Filter;

pub mod achainable;
pub mod discord_litentry;
pub mod discord_official;
pub mod litentry_archive;
pub mod nodereal_jsonrpc;
pub mod twitter_litentry;
pub mod twitter_official;

// It should only works on UNIX.
async fn shutdown_signal() {
	let mut hangup_stream = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::hangup())
		.expect("Cannot install SIGINT signal handler");
	let mut sigint_stream =
		tokio::signal::unix::signal(tokio::signal::unix::SignalKind::interrupt())
			.expect("Cannot install SIGINT signal handler");
	let mut sigterm_stream =
		tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
			.expect("Cannot install SIGINT signal handler");

	tokio::select! {
		_val = hangup_stream.recv() => log::warn!("Received SIGINT"),
		_val = sigint_stream.recv() => log::warn!("Received SIGINT"),
		_val = sigterm_stream.recv() => log::warn!("Received SIGTERM"),
	}
}

pub fn run(port: u16) -> Result<String, RecvError> {
	let (result_in, result_out) = channel();
	thread::spawn(move || {
		let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
		LocalSet::new().block_on(&runtime, async {
			let (addr, srv) = warp::serve(
				twitter_official::query_tweet()
					.or(twitter_official::query_retweeted_by())
					.or(twitter_official::query_user_by_name())
					.or(twitter_official::query_user_by_id())
					.or(twitter_litentry::check_follow())
					.or(discord_official::query_message())
					.or(discord_litentry::check_id_hubber())
					.or(discord_litentry::check_join())
					.or(discord_litentry::has_role())
					.or(nodereal_jsonrpc::query())
					.or(achainable::query())
					.or(litentry_archive::query_user_joined_evm_campaign())
					.boxed(),
			)
			.bind_with_graceful_shutdown(([127, 0, 0, 1], port), shutdown_signal());
			log::info!("mock-server listen on addr:{:?}", addr);
			let _ = result_in.send(format!("http://{:?}", addr));
			let join = tokio::task::spawn_local(srv);
			let _ = join.await;
		});
	});
	result_out.blocking_recv()
}
