// Copyright 2020-2022 Litentry Technologies GmbH.
// This file is part of Litentry.
//
// Litentry is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Litentry is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Litentry.  If not, see <https://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_bridge_transfer`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-11-14, STEPS: `20`, REPEAT: 50, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `parachain-benchmark`, CPU: `Intel(R) Xeon(R) Platinum 8175M CPU @ 2.50GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("litmus-dev"), DB CACHE: 20

// Executed Command:
// ./litentry-collator
// benchmark
// pallet
// --chain=litmus-dev
// --execution=wasm
// --db-cache=20
// --wasm-execution=compiled
// --pallet=pallet_bridge_transfer
// --extrinsic=*
// --heap-pages=4096
// --steps=20
// --repeat=50
// --header=./LICENSE_HEADER
// --output=./runtime/litmus/src/weights/pallet_bridge_transfer.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_bridge_transfer`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_bridge_transfer::WeightInfo for WeightInfo<T> {
	// Storage: BridgeTransfer ExternalBalances (r:1 w:1)
	// Storage: ChainBridge ChainNonces (r:1 w:1)
	// Storage: ChainBridge BridgeFee (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: ChainBridge BridgeEvents (r:1 w:1)
	fn transfer_native() -> Weight {
		Weight::from_ref_time(101_785_000 as u64)
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: BridgeTransfer MaximumIssuance (r:1 w:0)
	// Storage: BridgeTransfer ExternalBalances (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn transfer() -> Weight {
		Weight::from_ref_time(51_586_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: BridgeTransfer MaximumIssuance (r:1 w:1)
	fn set_maximum_issuance() -> Weight {
		Weight::from_ref_time(24_771_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: BridgeTransfer ExternalBalances (r:0 w:1)
	fn set_external_balances() -> Weight {
		Weight::from_ref_time(7_761_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
}
