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
//! Autogenerated weights for `pallet_collective`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-10-20, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bm6`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("kusama-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=kusama-dev
// --steps=50
// --repeat=20
// --pallet=pallet_collective
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/kusama/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_collective`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_collective::WeightInfo for WeightInfo<T> {
	// Storage: TechnicalCommittee Members (r:1 w:1)
	// Storage: TechnicalCommittee Proposals (r:1 w:0)
	// Storage: TechnicalCommittee Prime (r:0 w:1)
	// Storage: TechnicalCommittee Voting (r:100 w:100)
	/// The range of component `m` is `[0, 100]`.
	/// The range of component `n` is `[0, 100]`.
	/// The range of component `p` is `[0, 100]`.
	/// The range of component `m` is `[0, 100]`.
	/// The range of component `n` is `[0, 100]`.
	/// The range of component `p` is `[0, 100]`.
	fn set_members(m: u32, _n: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(16_918_000 as u64)
			// Standard Error: 47_641
			.saturating_add(Weight::from_ref_time(5_467_169 as u64).saturating_mul(m as u64))
			// Standard Error: 47_641
			.saturating_add(Weight::from_ref_time(7_509_448 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(p as u64)))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(p as u64)))
	}
	// Storage: TechnicalCommittee Members (r:1 w:0)
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	fn execute(b: u32, m: u32, ) -> Weight {
		Weight::from_ref_time(20_138_000 as u64)
			// Standard Error: 11
			.saturating_add(Weight::from_ref_time(969 as u64).saturating_mul(b as u64))
			// Standard Error: 119
			.saturating_add(Weight::from_ref_time(8_227 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
	// Storage: TechnicalCommittee Members (r:1 w:0)
	// Storage: TechnicalCommittee ProposalOf (r:1 w:0)
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	fn propose_execute(b: u32, m: u32, ) -> Weight {
		Weight::from_ref_time(22_120_000 as u64)
			// Standard Error: 11
			.saturating_add(Weight::from_ref_time(1_023 as u64).saturating_mul(b as u64))
			// Standard Error: 115
			.saturating_add(Weight::from_ref_time(14_536 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
	}
	// Storage: TechnicalCommittee Members (r:1 w:0)
	// Storage: TechnicalCommittee ProposalOf (r:1 w:1)
	// Storage: TechnicalCommittee Proposals (r:1 w:1)
	// Storage: TechnicalCommittee ProposalCount (r:1 w:1)
	// Storage: TechnicalCommittee Voting (r:0 w:1)
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `m` is `[2, 100]`.
	/// The range of component `p` is `[1, 100]`.
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `m` is `[2, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn propose_proposed(b: u32, m: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(28_232_000 as u64)
			// Standard Error: 36
			.saturating_add(Weight::from_ref_time(3_483 as u64).saturating_mul(b as u64))
			// Standard Error: 377
			.saturating_add(Weight::from_ref_time(17_323 as u64).saturating_mul(m as u64))
			// Standard Error: 375
			.saturating_add(Weight::from_ref_time(112_365 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: TechnicalCommittee Members (r:1 w:0)
	// Storage: TechnicalCommittee Voting (r:1 w:1)
	/// The range of component `m` is `[5, 100]`.
	/// The range of component `m` is `[5, 100]`.
	fn vote(m: u32, ) -> Weight {
		Weight::from_ref_time(28_655_000 as u64)
			// Standard Error: 226
			.saturating_add(Weight::from_ref_time(43_783 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: TechnicalCommittee Voting (r:1 w:1)
	// Storage: TechnicalCommittee Members (r:1 w:0)
	// Storage: TechnicalCommittee Proposals (r:1 w:1)
	// Storage: TechnicalCommittee ProposalOf (r:0 w:1)
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_disapproved(m: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(31_893_000 as u64)
			// Standard Error: 218
			.saturating_add(Weight::from_ref_time(26_663 as u64).saturating_mul(m as u64))
			// Standard Error: 219
			.saturating_add(Weight::from_ref_time(86_194 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: TechnicalCommittee Voting (r:1 w:1)
	// Storage: TechnicalCommittee Members (r:1 w:0)
	// Storage: TechnicalCommittee ProposalOf (r:1 w:1)
	// Storage: TechnicalCommittee Proposals (r:1 w:1)
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_approved(b: u32, m: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(41_631_000 as u64)
			// Standard Error: 31
			.saturating_add(Weight::from_ref_time(1_511 as u64).saturating_mul(b as u64))
			// Standard Error: 326
			.saturating_add(Weight::from_ref_time(22_506 as u64).saturating_mul(m as u64))
			// Standard Error: 322
			.saturating_add(Weight::from_ref_time(93_442 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: TechnicalCommittee Voting (r:1 w:1)
	// Storage: TechnicalCommittee Members (r:1 w:0)
	// Storage: TechnicalCommittee Prime (r:1 w:0)
	// Storage: TechnicalCommittee Proposals (r:1 w:1)
	// Storage: TechnicalCommittee ProposalOf (r:0 w:1)
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_disapproved(m: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(33_958_000 as u64)
			// Standard Error: 222
			.saturating_add(Weight::from_ref_time(31_797 as u64).saturating_mul(m as u64))
			// Standard Error: 223
			.saturating_add(Weight::from_ref_time(85_339 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: TechnicalCommittee Voting (r:1 w:1)
	// Storage: TechnicalCommittee Members (r:1 w:0)
	// Storage: TechnicalCommittee Prime (r:1 w:0)
	// Storage: TechnicalCommittee ProposalOf (r:1 w:1)
	// Storage: TechnicalCommittee Proposals (r:1 w:1)
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_approved(b: u32, m: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(43_314_000 as u64)
			// Standard Error: 33
			.saturating_add(Weight::from_ref_time(1_683 as u64).saturating_mul(b as u64))
			// Standard Error: 345
			.saturating_add(Weight::from_ref_time(25_291 as u64).saturating_mul(m as u64))
			// Standard Error: 341
			.saturating_add(Weight::from_ref_time(96_873 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: TechnicalCommittee Proposals (r:1 w:1)
	// Storage: TechnicalCommittee Voting (r:0 w:1)
	// Storage: TechnicalCommittee ProposalOf (r:0 w:1)
	/// The range of component `p` is `[1, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn disapprove_proposal(p: u32, ) -> Weight {
		Weight::from_ref_time(19_133_000 as u64)
			// Standard Error: 870
			.saturating_add(Weight::from_ref_time(140_476 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
}
