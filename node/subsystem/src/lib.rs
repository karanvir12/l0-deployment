// Copyright 2017-2020 Parity Technologies (UK) Ltd.
// This file is part of peer.

// peer is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// peer is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with peer.  If not, see <http://www.gnu.org/licenses/>.

//! Subsystem accumulation.
//!
//! Node-side types and generated overseer.

#![deny(missing_docs)]
#![deny(unused_crate_dependencies)]

pub use jaeger::*;
pub use peer_node_jaeger as jaeger;

pub use peer_overseer::{self as overseer, *};

pub use peer_node_subsystem_types::{
	errors::{self, *},
	ActivatedLeaf, LeafStatus,
};

/// Re-export of all messages type, including the wrapper type.
pub mod messages {
	pub use super::overseer::AllMessages;
	pub use peer_node_subsystem_types::messages::*;
}

/// A `Result` type that wraps [`SubsystemError`].
///
/// [`SubsystemError`]: struct.SubsystemError.html
pub type SubsystemResult<T> = Result<T, SubsystemError>;

// Simplify usage without having to do large scale modifications of all
// subsystems at once.

/// Specialized message type originating from the overseer.
pub type FromOrchestra<M> = peer_overseer::gen::FromOrchestra<M, OverseerSignal>;

/// Specialized subsystem instance type of subsystems consuming a particular message type.
pub type SubsystemInstance<Message> =
	peer_overseer::gen::SubsystemInstance<Message, OverseerSignal>;

/// Spawned subsystem.
pub type SpawnedSubsystem = peer_overseer::gen::SpawnedSubsystem<SubsystemError>;
