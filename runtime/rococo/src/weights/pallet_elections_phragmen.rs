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
//! Autogenerated weights for `pallet_elections_phragmen`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-10-21, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bm5`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("rococo-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=rococo-dev
// --steps=50
// --repeat=20
// --pallet=pallet_elections_phragmen
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

/// Weight functions for `pallet_elections_phragmen`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_elections_phragmen::WeightInfo for WeightInfo<T> {
	// Storage: PhragmenElection Candidates (r:1 w:0)
	// Storage: PhragmenElection Members (r:1 w:0)
	// Storage: PhragmenElection RunnersUp (r:1 w:0)
	// Storage: PhragmenElection Voting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	/// The range of component `v` is `[1, 16]`.
	fn vote_equal(v: u32, ) -> Weight {
		Weight::from_ref_time(30_905_000 as u64)
			// Standard Error: 2_727
			.saturating_add(Weight::from_ref_time(248_050 as u64).saturating_mul(v as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: PhragmenElection Candidates (r:1 w:0)
	// Storage: PhragmenElection Members (r:1 w:0)
	// Storage: PhragmenElection RunnersUp (r:1 w:0)
	// Storage: PhragmenElection Voting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	/// The range of component `v` is `[2, 16]`.
	fn vote_more(v: u32, ) -> Weight {
		Weight::from_ref_time(40_552_000 as u64)
			// Standard Error: 2_984
			.saturating_add(Weight::from_ref_time(275_380 as u64).saturating_mul(v as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: PhragmenElection Candidates (r:1 w:0)
	// Storage: PhragmenElection Members (r:1 w:0)
	// Storage: PhragmenElection RunnersUp (r:1 w:0)
	// Storage: PhragmenElection Voting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	/// The range of component `v` is `[2, 16]`.
	fn vote_less(v: u32, ) -> Weight {
		Weight::from_ref_time(40_536_000 as u64)
			// Standard Error: 2_983
			.saturating_add(Weight::from_ref_time(274_075 as u64).saturating_mul(v as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: PhragmenElection Voting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	fn remove_voter() -> Weight {
		Weight::from_ref_time(39_571_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: PhragmenElection Candidates (r:1 w:1)
	// Storage: PhragmenElection Members (r:1 w:0)
	// Storage: PhragmenElection RunnersUp (r:1 w:0)
	/// The range of component `c` is `[1, 1000]`.
	fn submit_candidacy(c: u32, ) -> Weight {
		Weight::from_ref_time(36_704_000 as u64)
			// Standard Error: 624
			.saturating_add(Weight::from_ref_time(82_076 as u64).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: PhragmenElection Candidates (r:1 w:1)
	/// The range of component `c` is `[1, 1000]`.
	fn renounce_candidacy_candidate(c: u32, ) -> Weight {
		Weight::from_ref_time(31_599_000 as u64)
			// Standard Error: 601
			.saturating_add(Weight::from_ref_time(56_954 as u64).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: PhragmenElection Members (r:1 w:1)
	// Storage: PhragmenElection RunnersUp (r:1 w:1)
	// Storage: Council Prime (r:1 w:1)
	// Storage: Council Proposals (r:1 w:0)
	// Storage: Council Members (r:0 w:1)
	fn renounce_candidacy_members() -> Weight {
		Weight::from_ref_time(45_045_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: PhragmenElection RunnersUp (r:1 w:1)
	fn renounce_candidacy_runners_up() -> Weight {
		Weight::from_ref_time(34_670_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Benchmark Override (r:0 w:0)
	fn remove_member_without_replacement() -> Weight {
		Weight::from_ref_time(2_000_000_000_000 as u64)
	}
	// Storage: PhragmenElection Members (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: PhragmenElection RunnersUp (r:1 w:1)
	// Storage: Council Prime (r:1 w:1)
	// Storage: Council Proposals (r:1 w:0)
	// Storage: Council Members (r:0 w:1)
	fn remove_member_with_replacement() -> Weight {
		Weight::from_ref_time(61_129_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: PhragmenElection Voting (r:5001 w:5000)
	// Storage: PhragmenElection Members (r:1 w:0)
	// Storage: PhragmenElection RunnersUp (r:1 w:0)
	// Storage: PhragmenElection Candidates (r:1 w:0)
	// Storage: Balances Locks (r:5000 w:5000)
	// Storage: System Account (r:5000 w:5000)
	/// The range of component `v` is `[5000, 10000]`.
	/// The range of component `d` is `[0, 5000]`.
	fn clean_defunct_voters(v: u32, _d: u32, ) -> Weight {
		Weight::from_ref_time(276_336_905_000 as u64)
			// Standard Error: 239_513
			.saturating_add(Weight::from_ref_time(34_546_162 as u64).saturating_mul(v as u64))
			.saturating_add(T::DbWeight::get().reads(15004 as u64))
			.saturating_add(T::DbWeight::get().reads((2 as u64).saturating_mul(v as u64)))
			.saturating_add(T::DbWeight::get().writes(15000 as u64))
			.saturating_add(T::DbWeight::get().writes((2 as u64).saturating_mul(v as u64)))
	}
	// Storage: PhragmenElection Candidates (r:1 w:1)
	// Storage: PhragmenElection Members (r:1 w:1)
	// Storage: PhragmenElection RunnersUp (r:1 w:1)
	// Storage: PhragmenElection Voting (r:10001 w:0)
	// Storage: Council Proposals (r:1 w:0)
	// Storage: PhragmenElection ElectionRounds (r:1 w:1)
	// Storage: Council Members (r:0 w:1)
	// Storage: Council Prime (r:0 w:1)
	// Storage: System Account (r:3 w:3)
	/// The range of component `c` is `[1, 1000]`.
	/// The range of component `v` is `[1, 10000]`.
	/// The range of component `e` is `[10000, 160000]`.
	fn election_phragmen(c: u32, v: u32, e: u32, ) -> Weight {
		Weight::from_ref_time(28_700_053_000 as u64)
			// Standard Error: 546_733
			.saturating_add(Weight::from_ref_time(45_497_244 as u64).saturating_mul(v as u64))
			// Standard Error: 35_086
			.saturating_add(Weight::from_ref_time(2_338_263 as u64).saturating_mul(e as u64))
			.saturating_add(T::DbWeight::get().reads(1968 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(v as u64)))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(c as u64)))
	}
}
