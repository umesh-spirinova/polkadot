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
//! Autogenerated weights for `pallet_identity`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-09-30, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bm4`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("kusama-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=kusama-dev
// --steps=50
// --repeat=20
// --pallet=pallet_identity
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

/// Weight functions for `pallet_identity`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_identity::WeightInfo for WeightInfo<T> {
	// Storage: Identity Registrars (r:1 w:1)
	/// The range of component `r` is `[1, 19]`.
	fn add_registrar(r: u32, ) -> Weight {
		Weight::from_ref_time(17_008_000 as u64)
			// Standard Error: 3_423
			.saturating_add(Weight::from_ref_time(247_415 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Identity IdentityOf (r:1 w:1)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `x` is `[1, 100]`.
	fn set_identity(r: u32, x: u32, ) -> Weight {
		Weight::from_ref_time(26_081_000 as u64)
			// Standard Error: 4_939
			.saturating_add(Weight::from_ref_time(391_847 as u64).saturating_mul(r as u64))
			// Standard Error: 986
			.saturating_add(Weight::from_ref_time(382_269 as u64).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SubsOf (r:1 w:1)
	// Storage: Identity SuperOf (r:1 w:1)
	/// The range of component `s` is `[1, 100]`.
	fn set_subs_new(s: u32, ) -> Weight {
		Weight::from_ref_time(28_706_000 as u64)
			// Standard Error: 1_328
			.saturating_add(Weight::from_ref_time(2_114_281 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(s as u64)))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(s as u64)))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SubsOf (r:1 w:1)
	// Storage: Identity SuperOf (r:0 w:1)
	/// The range of component `p` is `[1, 100]`.
	fn set_subs_old(p: u32, ) -> Weight {
		Weight::from_ref_time(28_097_000 as u64)
			// Standard Error: 793
			.saturating_add(Weight::from_ref_time(921_899 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(p as u64)))
	}
	// Storage: Identity SubsOf (r:1 w:1)
	// Storage: Identity IdentityOf (r:1 w:1)
	// Storage: Identity SuperOf (r:0 w:100)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `s` is `[1, 100]`.
	/// The range of component `x` is `[1, 100]`.
	fn clear_identity(_r: u32, s: u32, x: u32, ) -> Weight {
		Weight::from_ref_time(54_872_000 as u64)
			// Standard Error: 1_445
			.saturating_add(Weight::from_ref_time(832_329 as u64).saturating_mul(s as u64))
			// Standard Error: 1_445
			.saturating_add(Weight::from_ref_time(100_366 as u64).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(s as u64)))
	}
	// Storage: Identity Registrars (r:1 w:0)
	// Storage: Identity IdentityOf (r:1 w:1)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `x` is `[1, 100]`.
	fn request_judgement(r: u32, x: u32, ) -> Weight {
		Weight::from_ref_time(39_206_000 as u64)
			// Standard Error: 4_732
			.saturating_add(Weight::from_ref_time(28_431 as u64).saturating_mul(r as u64))
			// Standard Error: 944
			.saturating_add(Weight::from_ref_time(322_824 as u64).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Identity IdentityOf (r:1 w:1)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `x` is `[1, 100]`.
	fn cancel_request(r: u32, x: u32, ) -> Weight {
		Weight::from_ref_time(34_505_000 as u64)
			// Standard Error: 2_012
			.saturating_add(Weight::from_ref_time(26_516 as u64).saturating_mul(r as u64))
			// Standard Error: 401
			.saturating_add(Weight::from_ref_time(332_327 as u64).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Identity Registrars (r:1 w:1)
	/// The range of component `r` is `[1, 19]`.
	fn set_fee(r: u32, ) -> Weight {
		Weight::from_ref_time(8_460_000 as u64)
			// Standard Error: 1_762
			.saturating_add(Weight::from_ref_time(199_829 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Identity Registrars (r:1 w:1)
	/// The range of component `r` is `[1, 19]`.
	fn set_account_id(r: u32, ) -> Weight {
		Weight::from_ref_time(8_835_000 as u64)
			// Standard Error: 1_731
			.saturating_add(Weight::from_ref_time(178_499 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Identity Registrars (r:1 w:1)
	/// The range of component `r` is `[1, 19]`.
	fn set_fields(r: u32, ) -> Weight {
		Weight::from_ref_time(8_479_000 as u64)
			// Standard Error: 1_980
			.saturating_add(Weight::from_ref_time(202_338 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Identity Registrars (r:1 w:0)
	// Storage: Identity IdentityOf (r:1 w:1)
	/// The range of component `r` is `[1, 19]`.
	/// The range of component `x` is `[1, 100]`.
	fn provide_judgement(r: u32, x: u32, ) -> Weight {
		Weight::from_ref_time(30_075_000 as u64)
			// Standard Error: 4_819
			.saturating_add(Weight::from_ref_time(24_968 as u64).saturating_mul(r as u64))
			// Standard Error: 914
			.saturating_add(Weight::from_ref_time(556_563 as u64).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Identity SubsOf (r:1 w:1)
	// Storage: Identity IdentityOf (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Identity SuperOf (r:0 w:100)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `s` is `[1, 100]`.
	/// The range of component `x` is `[1, 100]`.
	fn kill_identity(_r: u32, s: u32, x: u32, ) -> Weight {
		Weight::from_ref_time(65_811_000 as u64)
			// Standard Error: 1_497
			.saturating_add(Weight::from_ref_time(847_620 as u64).saturating_mul(s as u64))
			// Standard Error: 1_497
			.saturating_add(Weight::from_ref_time(89_931 as u64).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(s as u64)))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SuperOf (r:1 w:1)
	// Storage: Identity SubsOf (r:1 w:1)
	/// The range of component `s` is `[1, 99]`.
	fn add_sub(s: u32, ) -> Weight {
		Weight::from_ref_time(35_152_000 as u64)
			// Standard Error: 1_173
			.saturating_add(Weight::from_ref_time(110_030 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SuperOf (r:1 w:1)
	/// The range of component `s` is `[1, 100]`.
	fn rename_sub(s: u32, ) -> Weight {
		Weight::from_ref_time(13_431_000 as u64)
			// Standard Error: 827
			.saturating_add(Weight::from_ref_time(55_554 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SuperOf (r:1 w:1)
	// Storage: Identity SubsOf (r:1 w:1)
	/// The range of component `s` is `[1, 100]`.
	fn remove_sub(s: u32, ) -> Weight {
		Weight::from_ref_time(34_702_000 as u64)
			// Standard Error: 1_554
			.saturating_add(Weight::from_ref_time(119_611 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Identity SuperOf (r:1 w:1)
	// Storage: Identity SubsOf (r:1 w:1)
	/// The range of component `s` is `[1, 99]`.
	fn quit_sub(s: u32, ) -> Weight {
		Weight::from_ref_time(25_895_000 as u64)
			// Standard Error: 984
			.saturating_add(Weight::from_ref_time(97_684 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
}
