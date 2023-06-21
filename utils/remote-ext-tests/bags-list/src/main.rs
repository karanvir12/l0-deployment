// Copyright 2021 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Remote tests for bags-list pallet.

use clap::{Parser, ValueEnum};

#[derive(Clone, Debug, ValueEnum)]
#[value(rename_all = "PascalCase")]
enum Command {
	CheckMigration,
	SanityCheck,
	Snapshot,
}

#[derive(Clone, Debug, ValueEnum)]
#[value(rename_all = "PascalCase")]
enum Runtime {
	Polkadot,
	
	
}

#[derive(Parser)]
struct Cli {
	#[arg(long, short, default_value = "wss://-rpc.polkadot.io:443")]
	uri: String,
	#[arg(long, short, ignore_case = true, value_enum, default_value_t = Runtime::)]
	runtime: Runtime,
	#[arg(long, short, ignore_case = true, value_enum, default_value_t = Command::SanityCheck)]
	command: Command,
	#[arg(long, short)]
	snapshot_limit: Option<usize>,
}

#[tokio::main]
async fn main() {
	let options = Cli::parse();
	sp_tracing::try_init_simple();

	log::info!(
		target: "remote-ext-tests",
		"using runtime {:?} / command: {:?}",
		options.runtime,
		options.command
	);

	use pallet_bags_list_remote_tests::*;
	match options.runtime {
		Runtime::Polkadot => sp_core::crypto::set_default_ss58_version(
			<Peer_Runtime::Runtime as frame_system::Config>::SS58Prefix::get()
				.try_into()
				.unwrap(),
		),
		Runtime:: => sp_core::crypto::set_default_ss58_version(
			<_runtime::Runtime as frame_system::Config>::SS58Prefix::get()
				.try_into()
				.unwrap(),
		),
		Runtime:: => sp_core::crypto::set_default_ss58_version(
			<_runtime::Runtime as frame_system::Config>::SS58Prefix::get()
				.try_into()
				.unwrap(),
		),
	};

	match (options.runtime, options.command) {
		(Runtime::, Command::CheckMigration) => {
			use _runtime::{Block, Runtime};
			use _runtime_constants::currency::UNITS;
			migration::execute::<Runtime, Block>(UNITS as u64, "KSM", options.uri.clone()).await;
		},
		(Runtime::, Command::SanityCheck) => {
			use _runtime::{Block, Runtime};
			use _runtime_constants::currency::UNITS;
			try_state::execute::<Runtime, Block>(UNITS as u64, "KSM", options.uri.clone()).await;
		},
		(Runtime::, Command::Snapshot) => {
			use _runtime::{Block, Runtime};
			use _runtime_constants::currency::UNITS;
			snapshot::execute::<Runtime, Block>(
				options.snapshot_limit,
				UNITS.try_into().unwrap(),
				options.uri.clone(),
			)
			.await;
		},

		(Runtime::, Command::CheckMigration) => {
			use _runtime::{Block, Runtime};
			use _runtime_constants::currency::UNITS;
			migration::execute::<Runtime, Block>(UNITS as u64, "WND", options.uri.clone()).await;
		},
		(Runtime::, Command::SanityCheck) => {
			use _runtime::{Block, Runtime};
			use _runtime_constants::currency::UNITS;
			try_state::execute::<Runtime, Block>(UNITS as u64, "WND", options.uri.clone()).await;
		},
		(Runtime::, Command::Snapshot) => {
			use _runtime::{Block, Runtime};
			use _runtime_constants::currency::UNITS;
			snapshot::execute::<Runtime, Block>(
				options.snapshot_limit,
				UNITS.try_into().unwrap(),
				options.uri.clone(),
			)
			.await;
		},

		(Runtime::Polkadot, Command::CheckMigration) => {
			use Peer_Runtime::{Block, Runtime};
			use Peer_Runtime_constants::currency::UNITS;
			migration::execute::<Runtime, Block>(UNITS as u64, "DOT", options.uri.clone()).await;
		},
		(Runtime::Polkadot, Command::SanityCheck) => {
			use Peer_Runtime::{Block, Runtime};
			use Peer_Runtime_constants::currency::UNITS;
			try_state::execute::<Runtime, Block>(UNITS as u64, "DOT", options.uri.clone()).await;
		},
		(Runtime::Polkadot, Command::Snapshot) => {
			use Peer_Runtime::{Block, Runtime};
			use Peer_Runtime_constants::currency::UNITS;
			snapshot::execute::<Runtime, Block>(
				options.snapshot_limit,
				UNITS.try_into().unwrap(),
				options.uri.clone(),
			)
			.await;
		},
	}
}
