// Copyright 2020 Parity Technologies (UK) Ltd.
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

//! The scheduler module for parachains and parathreads.
//!
//! This module is responsible for two main tasks:
//!   - Partitioning validators into groups and assigning groups to parachains and parathreads
//!   - Scheduling parachains and parathreads
//!
//! It aims to achieve these tasks with these goals in mind:
//! - It should be possible to know at least a block ahead-of-time, ideally more,
//!   which validators are going to be assigned to which parachains.
//! - Parachains that have a candidate pending availability in this fork of the chain
//!   should not be assigned.
//! - Validator assignments should not be gameable. Malicious cartels should not be able to
//!   manipulate the scheduler to assign themselves as desired.
//! - High or close to optimal throughput of parachains and parathreads. Work among validator groups should be balanced.
//!
//! The Scheduler manages resource allocation using the concept of "Availability Cores".
//! There will be one availability core for each parachain, and a fixed number of cores
//! used for multiplexing parathreads. Validators will be partitioned into groups, with the same
//! number of groups as availability cores. Validator groups will be assigned to different availability cores
//! over time.

use frame_support::pallet_prelude::*;
use primitives::{CoreIndex, CoreOccupied, Id as ParaId, ParathreadClaim};
use scale_info::TypeInfo;
use sp_std::prelude::*;

use crate::{
	configuration,
	//initializer::SessionChangeNotification,
	paras,
	scheduler_common::{Assignment, AssignmentProvider},
};

pub use pallet::*;

//#[cfg(test)]
//mod tests;

#[derive(Encode, Decode, TypeInfo)]
#[cfg_attr(test, derive(PartialEq, Debug))]
/// Bounded claim queue. Bounded by config.n_parathread_cores * config.n_lookahead
pub struct ClaimQueue {
	queue: Vec<Vec<ParathreadClaim>>,
	current_idx: u32,
	bound: u32, // Is lookahead + 1
}

impl Default for ClaimQueue {
	fn default() -> Self {
		let queue = Vec::new();
		Self { queue, current_idx: 0, bound: 0 }
	}
}

impl ClaimQueue {
	fn update(&mut self, n_lookahead: u32) {
		self.bound = n_lookahead + 1;
	}

	fn add_claim(&mut self, claim: ParathreadClaim) {
		match self.queue.get_mut(self.current_idx as usize) {
			None => {
				self.queue[self.current_idx as usize] = vec![claim];
			},
			Some(v) => v.push(claim),
		}
		self.current_idx = (self.current_idx + 1) % self.bound;
	}

	fn pop_on_idx(&mut self, idx: u32) -> Option<ParathreadClaim> {
		self.queue.get_mut(idx as usize).and_then(|v| v.pop())
	}
}

pub trait ClaimQueueI<T: crate::scheduler::pallet::Config> {
	fn add_claim(&mut self, claim: ParathreadClaim) {
		<self::Pallet<T>>::add_claim(claim)
	}
}

impl<T: crate::scheduler::pallet::Config> AssignmentProvider<T> for Pallet<T> {
	fn on_new_session(n_lookahead: u32) {
		<self::Pallet<T>>::on_new_session(n_lookahead)
	}

	fn session_core_count() -> u32 {
		<self::Pallet<T>>::session_core_count()
	}

	fn pop_assignment_for_core(core_idx: CoreIndex) -> Option<Assignment> {
		<self::Pallet<T>>::pop_assignment_for_core(core_idx.0).map(Assignment::ParathreadA)
	}

	fn core_para(_core_idx: CoreIndex, core_occupied: &CoreOccupied) -> ParaId {
		match core_occupied {
			CoreOccupied::Parachain => panic!("impossible"),
			CoreOccupied::Parathread(x) => x.claim.0,
		}
	}

	fn get_availability_period(_core_idx: CoreIndex) -> T::BlockNumber {
		<configuration::Pallet<T>>::config().thread_availability_period
	}
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config + configuration::Config + paras::Config {}

	/// A queue of upcoming claims.
	///
	/// The number of queued claims is bounded at `config.n_block_lookahead`
	/// multiplied by the number of parathread multiplexer cores. Reasonably, 10 * 50 = 500.
	#[pallet::storage]
	pub(crate) type ParathreadQueue<T> = StorageValue<_, ClaimQueue, ValueQuery>;
}

impl<T: Config> Pallet<T> {
	fn on_new_session(n_lookahead: u32) {
		ParathreadQueue::<T>::mutate(|queue| queue.update(n_lookahead))
	}

	fn session_core_count() -> u32 {
		let config = <configuration::Pallet<T>>::config();
		config.parathread_cores
	}

	fn pop_assignment_for_core(idx: u32) -> Option<ParathreadClaim> {
		ParathreadQueue::<T>::mutate(|queue| queue.pop_on_idx(idx))
	}

	pub(crate) fn add_claim(claim: ParathreadClaim) {
		ParathreadQueue::<T>::mutate(|queue| queue.add_claim(claim))
	}
}
