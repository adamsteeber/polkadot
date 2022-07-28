// Copyright 2017-2022 Parity Technologies (UK) Ltd.
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
//! Autogenerated weights for `runtime_parachains::paras_inherent`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-07-07, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bm5`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("kusama-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=kusama-dev
// --steps=50
// --repeat=20
// --pallet=runtime_parachains::paras_inherent
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/kusama/src/weights/runtime_parachains_paras_inherent.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `runtime_parachains::paras_inherent`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> runtime_parachains::paras_inherent::WeightInfo for WeightInfo<T> {
	// Storage: ParaInherent Included (r:1 w:1)
	// Storage: System ParentHash (r:1 w:0)
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Babe AuthorVrfRandomness (r:1 w:0)
	// Storage: ParaSessionInfo Sessions (r:1 w:0)
	// Storage: ParasDisputes Disputes (r:1 w:1)
	// Storage: ParasDisputes Included (r:1 w:1)
	// Storage: ParasDisputes SpamSlots (r:1 w:1)
	// Storage: ParaScheduler AvailabilityCores (r:1 w:1)
	// Storage: ParasDisputes Frozen (r:1 w:0)
	// Storage: ParaInclusion PendingAvailability (r:2 w:1)
	// Storage: ParasShared ActiveValidatorKeys (r:1 w:0)
	// Storage: Paras Parachains (r:1 w:0)
	// Storage: ParaInclusion PendingAvailabilityCommitments (r:1 w:1)
	// Storage: ParaSessionInfo AccountKeys (r:1 w:0)
	// Storage: Staking ActiveEra (r:1 w:0)
	// Storage: Staking ErasRewardPoints (r:1 w:1)
	// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	// Storage: Hrmp HrmpChannelDigests (r:1 w:1)
	// Storage: Paras FutureCodeUpgrades (r:1 w:0)
	// Storage: ParaInherent OnChainVotes (r:1 w:1)
	// Storage: ParaScheduler SessionStartBlock (r:1 w:0)
	// Storage: ParaScheduler ParathreadQueue (r:1 w:1)
	// Storage: ParaScheduler Scheduled (r:1 w:1)
	// Storage: ParaScheduler ValidatorGroups (r:1 w:0)
	// Storage: Ump NeedsDispatch (r:1 w:1)
	// Storage: Ump NextDispatchRoundStartWith (r:1 w:1)
	// Storage: Hrmp HrmpWatermarks (r:0 w:1)
	// Storage: Paras Heads (r:0 w:1)
	// Storage: Paras UpgradeGoAheadSignal (r:0 w:1)
	/// The range of component `v` is `[10, 200]`.
	fn enter_variable_disputes(v: u32, ) -> Weight {
		(456_006_000 as Weight)
			// Standard Error: 23_000
			.saturating_add((48_684_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads(28 as Weight))
			.saturating_add(T::DbWeight::get().writes(18 as Weight))
	}
	// Storage: ParaInherent Included (r:1 w:1)
	// Storage: System ParentHash (r:1 w:0)
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Babe AuthorVrfRandomness (r:1 w:0)
	// Storage: ParaScheduler AvailabilityCores (r:1 w:1)
	// Storage: ParasDisputes Frozen (r:1 w:0)
	// Storage: ParasShared ActiveValidatorKeys (r:1 w:0)
	// Storage: Paras Parachains (r:1 w:0)
	// Storage: ParaInclusion PendingAvailability (r:2 w:1)
	// Storage: ParaInclusion PendingAvailabilityCommitments (r:1 w:1)
	// Storage: ParaSessionInfo AccountKeys (r:1 w:0)
	// Storage: Staking ActiveEra (r:1 w:0)
	// Storage: Staking ErasRewardPoints (r:1 w:1)
	// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	// Storage: Hrmp HrmpChannelDigests (r:1 w:1)
	// Storage: Paras FutureCodeUpgrades (r:1 w:0)
	// Storage: ParaInherent OnChainVotes (r:1 w:1)
	// Storage: ParasDisputes Disputes (r:1 w:0)
	// Storage: ParaScheduler SessionStartBlock (r:1 w:0)
	// Storage: ParaScheduler ParathreadQueue (r:1 w:1)
	// Storage: ParaScheduler Scheduled (r:1 w:1)
	// Storage: ParaScheduler ValidatorGroups (r:1 w:0)
	// Storage: Ump NeedsDispatch (r:1 w:1)
	// Storage: Ump NextDispatchRoundStartWith (r:1 w:1)
	// Storage: ParaInclusion AvailabilityBitfields (r:0 w:1)
	// Storage: ParasDisputes Included (r:0 w:1)
	// Storage: Hrmp HrmpWatermarks (r:0 w:1)
	// Storage: Paras Heads (r:0 w:1)
	// Storage: Paras UpgradeGoAheadSignal (r:0 w:1)
	fn enter_bitfields() -> Weight {
		(439_453_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(25 as Weight))
			.saturating_add(T::DbWeight::get().writes(17 as Weight))
	}
	// Storage: ParaInherent Included (r:1 w:1)
	// Storage: System ParentHash (r:1 w:0)
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Babe AuthorVrfRandomness (r:1 w:0)
	// Storage: ParaScheduler AvailabilityCores (r:1 w:1)
	// Storage: ParasDisputes Frozen (r:1 w:0)
	// Storage: ParasShared ActiveValidatorKeys (r:1 w:0)
	// Storage: Paras Parachains (r:1 w:0)
	// Storage: ParaInclusion PendingAvailability (r:2 w:1)
	// Storage: ParaInclusion PendingAvailabilityCommitments (r:1 w:1)
	// Storage: ParaSessionInfo AccountKeys (r:1 w:0)
	// Storage: Staking ActiveEra (r:1 w:0)
	// Storage: Staking ErasRewardPoints (r:1 w:1)
	// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	// Storage: Hrmp HrmpChannelDigests (r:1 w:1)
	// Storage: Paras FutureCodeUpgrades (r:1 w:0)
	// Storage: ParaInherent OnChainVotes (r:1 w:1)
	// Storage: ParasDisputes Disputes (r:2 w:0)
	// Storage: ParaScheduler SessionStartBlock (r:1 w:0)
	// Storage: ParaScheduler ParathreadQueue (r:1 w:1)
	// Storage: ParaScheduler Scheduled (r:1 w:1)
	// Storage: ParaScheduler ValidatorGroups (r:1 w:0)
	// Storage: Paras CurrentCodeHash (r:1 w:0)
	// Storage: Ump RelayDispatchQueueSize (r:1 w:0)
	// Storage: Ump NeedsDispatch (r:1 w:1)
	// Storage: Ump NextDispatchRoundStartWith (r:1 w:1)
	// Storage: ParasDisputes Included (r:0 w:1)
	// Storage: Hrmp HrmpWatermarks (r:0 w:1)
	// Storage: Paras Heads (r:0 w:1)
	// Storage: Paras UpgradeGoAheadSignal (r:0 w:1)
	/// The range of component `v` is `[101, 200]`.
	fn enter_backed_candidates_variable(v: u32, ) -> Weight {
		(1_109_095_000 as Weight)
			// Standard Error: 46_000
			.saturating_add((48_263_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads(28 as Weight))
			.saturating_add(T::DbWeight::get().writes(16 as Weight))
	}
	// Storage: ParaInherent Included (r:1 w:1)
	// Storage: System ParentHash (r:1 w:0)
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Babe AuthorVrfRandomness (r:1 w:0)
	// Storage: ParaScheduler AvailabilityCores (r:1 w:1)
	// Storage: ParasDisputes Frozen (r:1 w:0)
	// Storage: ParasShared ActiveValidatorKeys (r:1 w:0)
	// Storage: Paras Parachains (r:1 w:0)
	// Storage: ParaInclusion PendingAvailability (r:2 w:1)
	// Storage: ParaInclusion PendingAvailabilityCommitments (r:1 w:1)
	// Storage: ParaSessionInfo AccountKeys (r:1 w:0)
	// Storage: Staking ActiveEra (r:1 w:0)
	// Storage: Staking ErasRewardPoints (r:1 w:1)
	// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	// Storage: Hrmp HrmpChannelDigests (r:1 w:1)
	// Storage: Paras FutureCodeUpgrades (r:1 w:0)
	// Storage: ParaInherent OnChainVotes (r:1 w:1)
	// Storage: ParasDisputes Disputes (r:2 w:0)
	// Storage: ParaScheduler SessionStartBlock (r:1 w:0)
	// Storage: ParaScheduler ParathreadQueue (r:1 w:1)
	// Storage: ParaScheduler Scheduled (r:1 w:1)
	// Storage: ParaScheduler ValidatorGroups (r:1 w:0)
	// Storage: Paras CurrentCodeHash (r:1 w:0)
	// Storage: Paras FutureCodeHash (r:1 w:0)
	// Storage: Paras UpgradeRestrictionSignal (r:1 w:0)
	// Storage: Ump RelayDispatchQueueSize (r:1 w:0)
	// Storage: Ump NeedsDispatch (r:1 w:1)
	// Storage: Ump NextDispatchRoundStartWith (r:1 w:1)
	// Storage: ParasDisputes Included (r:0 w:1)
	// Storage: Hrmp HrmpWatermarks (r:0 w:1)
	// Storage: Paras Heads (r:0 w:1)
	// Storage: Paras UpgradeGoAheadSignal (r:0 w:1)
	fn enter_backed_candidate_code_upgrade() -> Weight {
		(46_457_619_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(30 as Weight))
			.saturating_add(T::DbWeight::get().writes(16 as Weight))
	}
}
