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
//! Autogenerated weights for `pallet_fast_unstake`
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
// --pallet=pallet_fast_unstake
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

/// Weight functions for `pallet_fast_unstake`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_fast_unstake::WeightInfo for WeightInfo<T> {
	// Storage: FastUnstake ErasToCheckPerBlock (r:1 w:0)
	// Storage: Staking ValidatorCount (r:1 w:0)
	// Storage: ElectionProviderMultiPhase CurrentPhase (r:1 w:0)
	// Storage: FastUnstake Head (r:1 w:1)
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Staking SlashingSpans (r:1 w:0)
	// Storage: Staking Bonded (r:1 w:1)
	// Storage: Staking Validators (r:1 w:0)
	// Storage: Staking Nominators (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: Staking Ledger (r:0 w:1)
	// Storage: Staking Payee (r:0 w:1)
	fn on_idle_unstake() -> Weight {
		Weight::from_ref_time(72_565_000 as u64)
			.saturating_add(T::DbWeight::get().reads(11 as u64))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	// Storage: FastUnstake ErasToCheckPerBlock (r:1 w:0)
	// Storage: Staking ValidatorCount (r:1 w:0)
	// Storage: ElectionProviderMultiPhase CurrentPhase (r:1 w:0)
	// Storage: FastUnstake Head (r:1 w:1)
	// Storage: FastUnstake Queue (r:2 w:1)
	// Storage: FastUnstake CounterForQueue (r:1 w:1)
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Staking ErasStakers (r:56 w:0)
	/// The range of component `x` is `[28, 3584]`.
	fn on_idle_check(x: u32, ) -> Weight {
		Weight::from_ref_time(431_232_000 as u64)
			// Standard Error: 17_209
			.saturating_add(Weight::from_ref_time(11_304_676 as u64).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(64 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(x as u64)))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: FastUnstake ErasToCheckPerBlock (r:1 w:0)
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: FastUnstake Queue (r:1 w:1)
	// Storage: FastUnstake Head (r:1 w:0)
	// Storage: Staking Validators (r:1 w:0)
	// Storage: Staking Nominators (r:1 w:1)
	// Storage: Staking CounterForNominators (r:1 w:1)
	// Storage: VoterList ListNodes (r:1 w:1)
	// Storage: VoterList ListBags (r:1 w:1)
	// Storage: VoterList CounterForListNodes (r:1 w:1)
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: FastUnstake CounterForQueue (r:1 w:1)
	fn register_fast_unstake() -> Weight {
		Weight::from_ref_time(94_396_000 as u64)
			.saturating_add(T::DbWeight::get().reads(13 as u64))
			.saturating_add(T::DbWeight::get().writes(9 as u64))
	}
	// Storage: FastUnstake ErasToCheckPerBlock (r:1 w:0)
	// Storage: Staking Ledger (r:1 w:0)
	// Storage: FastUnstake Queue (r:1 w:1)
	// Storage: FastUnstake Head (r:1 w:0)
	// Storage: FastUnstake CounterForQueue (r:1 w:1)
	fn deregister() -> Weight {
		Weight::from_ref_time(41_022_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: FastUnstake ErasToCheckPerBlock (r:0 w:1)
	fn control() -> Weight {
		Weight::from_ref_time(3_813_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
}
