// Copyright (C) Parity Technologies (UK) Ltd.
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

//! Autogenerated weights for `pallet_referenda`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-26, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bm6`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("polkadot-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=polkadot-dev
// --steps=50
// --repeat=20
// --pallet=pallet_referenda
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/polkadot/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_referenda`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_referenda::WeightInfo for WeightInfo<T> {
	/// Storage: Referenda ReferendumCount (r:1 w:1)
	/// Proof: Referenda ReferendumCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	/// Storage: Referenda ReferendumInfoFor (r:0 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	fn submit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `291`
		//  Estimated: `42428`
		// Minimum execution time: 40_270_000 picoseconds.
		Weight::from_parts(40_759_000, 0)
			.saturating_add(Weight::from_parts(0, 42428))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn place_decision_deposit_preparing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `544`
		//  Estimated: `83866`
		// Minimum execution time: 51_786_000 picoseconds.
		Weight::from_parts(52_344_000, 0)
			.saturating_add(Weight::from_parts(0, 83866))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Referenda DecidingCount (r:1 w:0)
	/// Proof: Referenda DecidingCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: Referenda TrackQueue (r:1 w:1)
	/// Proof: Referenda TrackQueue (max_values: None, max_size: Some(2012), added: 4487, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn place_decision_deposit_queued() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3331`
		//  Estimated: `42428`
		// Minimum execution time: 60_191_000 picoseconds.
		Weight::from_parts(61_033_000, 0)
			.saturating_add(Weight::from_parts(0, 42428))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Referenda DecidingCount (r:1 w:0)
	/// Proof: Referenda DecidingCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: Referenda TrackQueue (r:1 w:1)
	/// Proof: Referenda TrackQueue (max_values: None, max_size: Some(2012), added: 4487, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn place_decision_deposit_not_queued() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3351`
		//  Estimated: `42428`
		// Minimum execution time: 60_037_000 picoseconds.
		Weight::from_parts(60_745_000, 0)
			.saturating_add(Weight::from_parts(0, 42428))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Referenda DecidingCount (r:1 w:1)
	/// Proof: Referenda DecidingCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: Balances InactiveIssuance (r:1 w:0)
	/// Proof: Balances InactiveIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn place_decision_deposit_passing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `544`
		//  Estimated: `83866`
		// Minimum execution time: 62_358_000 picoseconds.
		Weight::from_parts(63_007_000, 0)
			.saturating_add(Weight::from_parts(0, 83866))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Referenda DecidingCount (r:1 w:1)
	/// Proof: Referenda DecidingCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: Balances InactiveIssuance (r:1 w:0)
	/// Proof: Balances InactiveIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn place_decision_deposit_failing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `544`
		//  Estimated: `83866`
		// Minimum execution time: 61_443_000 picoseconds.
		Weight::from_parts(62_121_000, 0)
			.saturating_add(Weight::from_parts(0, 83866))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	fn refund_decision_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `384`
		//  Estimated: `4401`
		// Minimum execution time: 30_518_000 picoseconds.
		Weight::from_parts(30_877_000, 0)
			.saturating_add(Weight::from_parts(0, 4401))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	fn refund_submission_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `374`
		//  Estimated: `4401`
		// Minimum execution time: 30_439_000 picoseconds.
		Weight::from_parts(30_898_000, 0)
			.saturating_add(Weight::from_parts(0, 4401))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn cancel() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `452`
		//  Estimated: `83866`
		// Minimum execution time: 38_311_000 picoseconds.
		Weight::from_parts(38_772_000, 0)
			.saturating_add(Weight::from_parts(0, 83866))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	/// Storage: Referenda MetadataOf (r:1 w:0)
	/// Proof: Referenda MetadataOf (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	fn kill() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `693`
		//  Estimated: `83866`
		// Minimum execution time: 98_273_000 picoseconds.
		Weight::from_parts(99_232_000, 0)
			.saturating_add(Weight::from_parts(0, 83866))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Referenda TrackQueue (r:1 w:0)
	/// Proof: Referenda TrackQueue (max_values: None, max_size: Some(2012), added: 4487, mode: MaxEncodedLen)
	/// Storage: Referenda DecidingCount (r:1 w:1)
	/// Proof: Referenda DecidingCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	fn one_fewer_deciding_queue_empty() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `207`
		//  Estimated: `5477`
		// Minimum execution time: 9_503_000 picoseconds.
		Weight::from_parts(9_707_000, 0)
			.saturating_add(Weight::from_parts(0, 5477))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Referenda TrackQueue (r:1 w:1)
	/// Proof: Referenda TrackQueue (max_values: None, max_size: Some(2012), added: 4487, mode: MaxEncodedLen)
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Balances InactiveIssuance (r:1 w:0)
	/// Proof: Balances InactiveIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn one_fewer_deciding_failing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3221`
		//  Estimated: `42428`
		// Minimum execution time: 44_039_000 picoseconds.
		Weight::from_parts(44_580_000, 0)
			.saturating_add(Weight::from_parts(0, 42428))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Referenda TrackQueue (r:1 w:1)
	/// Proof: Referenda TrackQueue (max_values: None, max_size: Some(2012), added: 4487, mode: MaxEncodedLen)
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Balances InactiveIssuance (r:1 w:0)
	/// Proof: Balances InactiveIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn one_fewer_deciding_passing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3221`
		//  Estimated: `42428`
		// Minimum execution time: 46_392_000 picoseconds.
		Weight::from_parts(47_042_000, 0)
			.saturating_add(Weight::from_parts(0, 42428))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:0)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Referenda TrackQueue (r:1 w:1)
	/// Proof: Referenda TrackQueue (max_values: None, max_size: Some(2012), added: 4487, mode: MaxEncodedLen)
	fn nudge_referendum_requeued_insertion() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3044`
		//  Estimated: `5477`
		// Minimum execution time: 19_629_000 picoseconds.
		Weight::from_parts(19_992_000, 0)
			.saturating_add(Weight::from_parts(0, 5477))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:0)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Referenda TrackQueue (r:1 w:1)
	/// Proof: Referenda TrackQueue (max_values: None, max_size: Some(2012), added: 4487, mode: MaxEncodedLen)
	fn nudge_referendum_requeued_slide() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3044`
		//  Estimated: `5477`
		// Minimum execution time: 19_523_000 picoseconds.
		Weight::from_parts(19_916_000, 0)
			.saturating_add(Weight::from_parts(0, 5477))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Referenda DecidingCount (r:1 w:0)
	/// Proof: Referenda DecidingCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: Referenda TrackQueue (r:1 w:1)
	/// Proof: Referenda TrackQueue (max_values: None, max_size: Some(2012), added: 4487, mode: MaxEncodedLen)
	fn nudge_referendum_queued() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3048`
		//  Estimated: `5477`
		// Minimum execution time: 24_190_000 picoseconds.
		Weight::from_parts(24_524_000, 0)
			.saturating_add(Weight::from_parts(0, 5477))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Referenda DecidingCount (r:1 w:0)
	/// Proof: Referenda DecidingCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: Referenda TrackQueue (r:1 w:1)
	/// Proof: Referenda TrackQueue (max_values: None, max_size: Some(2012), added: 4487, mode: MaxEncodedLen)
	fn nudge_referendum_not_queued() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3068`
		//  Estimated: `5477`
		// Minimum execution time: 23_911_000 picoseconds.
		Weight::from_parts(24_264_000, 0)
			.saturating_add(Weight::from_parts(0, 5477))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn nudge_referendum_no_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `404`
		//  Estimated: `42428`
		// Minimum execution time: 25_600_000 picoseconds.
		Weight::from_parts(26_011_000, 0)
			.saturating_add(Weight::from_parts(0, 42428))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn nudge_referendum_preparing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `452`
		//  Estimated: `42428`
		// Minimum execution time: 25_959_000 picoseconds.
		Weight::from_parts(26_277_000, 0)
			.saturating_add(Weight::from_parts(0, 42428))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	fn nudge_referendum_timed_out() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `311`
		//  Estimated: `4401`
		// Minimum execution time: 17_654_000 picoseconds.
		Weight::from_parts(17_886_000, 0)
			.saturating_add(Weight::from_parts(0, 4401))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Referenda DecidingCount (r:1 w:1)
	/// Proof: Referenda DecidingCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: Balances InactiveIssuance (r:1 w:0)
	/// Proof: Balances InactiveIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn nudge_referendum_begin_deciding_failing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `452`
		//  Estimated: `42428`
		// Minimum execution time: 35_251_000 picoseconds.
		Weight::from_parts(35_709_000, 0)
			.saturating_add(Weight::from_parts(0, 42428))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Referenda DecidingCount (r:1 w:1)
	/// Proof: Referenda DecidingCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: Balances InactiveIssuance (r:1 w:0)
	/// Proof: Balances InactiveIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn nudge_referendum_begin_deciding_passing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `452`
		//  Estimated: `42428`
		// Minimum execution time: 37_334_000 picoseconds.
		Weight::from_parts(37_625_000, 0)
			.saturating_add(Weight::from_parts(0, 42428))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Balances InactiveIssuance (r:1 w:0)
	/// Proof: Balances InactiveIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn nudge_referendum_begin_confirming() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `505`
		//  Estimated: `42428`
		// Minimum execution time: 30_728_000 picoseconds.
		Weight::from_parts(31_178_000, 0)
			.saturating_add(Weight::from_parts(0, 42428))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Balances InactiveIssuance (r:1 w:0)
	/// Proof: Balances InactiveIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn nudge_referendum_end_confirming() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `488`
		//  Estimated: `42428`
		// Minimum execution time: 31_411_000 picoseconds.
		Weight::from_parts(31_672_000, 0)
			.saturating_add(Weight::from_parts(0, 42428))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Balances InactiveIssuance (r:1 w:0)
	/// Proof: Balances InactiveIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn nudge_referendum_continue_not_confirming() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `505`
		//  Estimated: `42428`
		// Minimum execution time: 28_828_000 picoseconds.
		Weight::from_parts(29_108_000, 0)
			.saturating_add(Weight::from_parts(0, 42428))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Balances InactiveIssuance (r:1 w:0)
	/// Proof: Balances InactiveIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn nudge_referendum_continue_confirming() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `509`
		//  Estimated: `42428`
		// Minimum execution time: 27_377_000 picoseconds.
		Weight::from_parts(27_868_000, 0)
			.saturating_add(Weight::from_parts(0, 42428))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Balances InactiveIssuance (r:1 w:0)
	/// Proof: Balances InactiveIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	/// Storage: Scheduler Lookup (r:1 w:1)
	/// Proof: Scheduler Lookup (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	fn nudge_referendum_approved() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `509`
		//  Estimated: `83866`
		// Minimum execution time: 42_303_000 picoseconds.
		Weight::from_parts(42_647_000, 0)
			.saturating_add(Weight::from_parts(0, 83866))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Balances InactiveIssuance (r:1 w:0)
	/// Proof: Balances InactiveIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn nudge_referendum_rejected() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `505`
		//  Estimated: `42428`
		// Minimum execution time: 30_819_000 picoseconds.
		Weight::from_parts(31_121_000, 0)
			.saturating_add(Weight::from_parts(0, 42428))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:0)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Preimage StatusFor (r:1 w:0)
	/// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	/// Storage: Referenda MetadataOf (r:0 w:1)
	/// Proof: Referenda MetadataOf (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	fn set_some_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `455`
		//  Estimated: `4401`
		// Minimum execution time: 20_928_000 picoseconds.
		Weight::from_parts(21_267_000, 0)
			.saturating_add(Weight::from_parts(0, 4401))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:0)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Referenda MetadataOf (r:1 w:1)
	/// Proof: Referenda MetadataOf (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	fn clear_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `388`
		//  Estimated: `4401`
		// Minimum execution time: 18_504_000 picoseconds.
		Weight::from_parts(18_786_000, 0)
			.saturating_add(Weight::from_parts(0, 4401))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
