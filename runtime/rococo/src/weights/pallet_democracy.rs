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
//! Autogenerated weights for `pallet_democracy`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-10-20, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bm5`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("rococo-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=rococo-dev
// --steps=50
// --repeat=20
// --pallet=pallet_democracy
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/rococo/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_democracy`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_democracy::WeightInfo for WeightInfo<T> {
	// Storage: Democracy PublicPropCount (r:1 w:1)
	// Storage: Democracy PublicProps (r:1 w:1)
	// Storage: Democracy Blacklist (r:1 w:0)
	// Storage: Democracy DepositOf (r:0 w:1)
	fn propose() -> Weight {
		Weight::from_ref_time(42_316_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Democracy DepositOf (r:1 w:1)
	fn second() -> Weight {
		Weight::from_ref_time(39_589_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Democracy VotingOf (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	fn vote_new() -> Weight {
		Weight::from_ref_time(50_033_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Democracy VotingOf (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	fn vote_existing() -> Weight {
		Weight::from_ref_time(50_280_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Democracy Cancellations (r:1 w:1)
	fn emergency_cancel() -> Weight {
		Weight::from_ref_time(20_570_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Democracy PublicProps (r:1 w:1)
	// Storage: Democracy DepositOf (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Democracy NextExternal (r:1 w:1)
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Democracy Blacklist (r:0 w:1)
	fn blacklist() -> Weight {
		Weight::from_ref_time(78_492_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	// Storage: Democracy NextExternal (r:1 w:1)
	// Storage: Democracy Blacklist (r:1 w:0)
	fn external_propose() -> Weight {
		Weight::from_ref_time(16_095_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Democracy NextExternal (r:0 w:1)
	fn external_propose_majority() -> Weight {
		Weight::from_ref_time(5_147_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Democracy NextExternal (r:0 w:1)
	fn external_propose_default() -> Weight {
		Weight::from_ref_time(4_998_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Democracy NextExternal (r:1 w:1)
	// Storage: Democracy ReferendumCount (r:1 w:1)
	// Storage: Democracy ReferendumInfoOf (r:0 w:1)
	fn fast_track() -> Weight {
		Weight::from_ref_time(20_461_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Democracy NextExternal (r:1 w:1)
	// Storage: Democracy Blacklist (r:1 w:1)
	fn veto_external() -> Weight {
		Weight::from_ref_time(26_559_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Democracy PublicProps (r:1 w:1)
	// Storage: Democracy DepositOf (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn cancel_proposal() -> Weight {
		Weight::from_ref_time(66_363_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Democracy ReferendumInfoOf (r:0 w:1)
	fn cancel_referendum() -> Weight {
		Weight::from_ref_time(13_386_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Democracy LowestUnbaked (r:1 w:1)
	// Storage: Democracy ReferendumCount (r:1 w:0)
	// Storage: Democracy ReferendumInfoOf (r:2 w:0)
	/// The range of component `r` is `[0, 99]`.
	fn on_initialize_base(r: u32, ) -> Weight {
		Weight::from_ref_time(6_135_000 as u64)
			// Standard Error: 1_982
			.saturating_add(Weight::from_ref_time(2_147_180 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(r as u64)))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Democracy LowestUnbaked (r:1 w:1)
	// Storage: Democracy ReferendumCount (r:1 w:0)
	// Storage: Democracy LastTabledWasExternal (r:1 w:0)
	// Storage: Democracy NextExternal (r:1 w:0)
	// Storage: Democracy PublicProps (r:1 w:0)
	// Storage: Democracy ReferendumInfoOf (r:2 w:0)
	/// The range of component `r` is `[0, 99]`.
	fn on_initialize_base_with_launch_period(r: u32, ) -> Weight {
		Weight::from_ref_time(8_464_000 as u64)
			// Standard Error: 2_514
			.saturating_add(Weight::from_ref_time(2_157_490 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(r as u64)))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Democracy VotingOf (r:3 w:3)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: Democracy ReferendumInfoOf (r:2 w:2)
	/// The range of component `r` is `[0, 99]`.
	fn delegate(r: u32, ) -> Weight {
		Weight::from_ref_time(42_257_000 as u64)
			// Standard Error: 3_365
			.saturating_add(Weight::from_ref_time(3_142_233 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(r as u64)))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(r as u64)))
	}
	// Storage: Democracy VotingOf (r:2 w:2)
	// Storage: Democracy ReferendumInfoOf (r:2 w:2)
	/// The range of component `r` is `[0, 99]`.
	fn undelegate(r: u32, ) -> Weight {
		Weight::from_ref_time(24_565_000 as u64)
			// Standard Error: 3_213
			.saturating_add(Weight::from_ref_time(3_026_252 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(r as u64)))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(r as u64)))
	}
	// Storage: Democracy PublicProps (r:0 w:1)
	fn clear_public_proposals() -> Weight {
		Weight::from_ref_time(5_324_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Democracy VotingOf (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `r` is `[0, 99]`.
	fn unlock_remove(r: u32, ) -> Weight {
		Weight::from_ref_time(23_086_000 as u64)
			// Standard Error: 3_144
			.saturating_add(Weight::from_ref_time(146_522 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Democracy VotingOf (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `r` is `[0, 99]`.
	fn unlock_set(r: u32, ) -> Weight {
		Weight::from_ref_time(29_381_000 as u64)
			// Standard Error: 504
			.saturating_add(Weight::from_ref_time(87_574 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Democracy VotingOf (r:1 w:1)
	/// The range of component `r` is `[1, 100]`.
	fn remove_vote(r: u32, ) -> Weight {
		Weight::from_ref_time(15_887_000 as u64)
			// Standard Error: 1_047
			.saturating_add(Weight::from_ref_time(115_123 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	// Storage: Democracy VotingOf (r:1 w:1)
	/// The range of component `r` is `[1, 100]`.
	fn remove_other_vote(r: u32, ) -> Weight {
		Weight::from_ref_time(16_051_000 as u64)
			// Standard Error: 1_010
			.saturating_add(Weight::from_ref_time(112_844 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
}
