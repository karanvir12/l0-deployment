// Copyright 2021 Parity Technologies (UK) Ltd.
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

/// A priority assigned to execution of a PVF.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
	/// Normal priority for things that do not require immediate response, but still need to be
	/// done pretty quick.
	///
	/// Approvals and disputes fall into this category.
	Normal,
	/// This priority is used for requests that are required to be processed as soon as possible.
	///
	/// For example, backing is on a critical path and requires execution as soon as possible.
	Critical,
}

impl Priority {
	/// Returns `true` if `self` is `Crticial`
	pub fn is_critical(self) -> bool {
		self == Priority::Critical
	}
}
