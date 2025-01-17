// Copyright 2020-2024 Trust Computing GmbH.
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

//! Autogenerated weights for `pallet_teebag`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-07-31, STEPS: `20`, REPEAT: `50`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `litentry-benchmark-server`, CPU: `Intel(R) Xeon(R) CPU E5-2686 v4 @ 2.30GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("litmus-dev"), DB CACHE: 20

// Executed Command:
// ./litentry-collator
// benchmark
// pallet
// --chain=litmus-dev
// --execution=wasm
// --db-cache=20
// --wasm-execution=compiled
// --pallet=pallet_teebag
// --extrinsic=*
// --heap-pages=4096
// --steps=20
// --repeat=50
// --header=./LICENSE_HEADER
// --output=./runtime/litmus/src/weights/pallet_teebag.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_teebag`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_teebag::WeightInfo for WeightInfo<T> {
	/// Storage: Teebag EnclaveRegistry (r:1 w:1)
	/// Proof Skipped: Teebag EnclaveRegistry (max_values: None, max_size: None, mode: Measured)
	/// Storage: Teebag EnclaveIdentifier (r:1 w:1)
	/// Proof Skipped: Teebag EnclaveIdentifier (max_values: None, max_size: None, mode: Measured)
	fn force_add_enclave() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `180`
		//  Estimated: `3645`
		// Minimum execution time: 27_240_000 picoseconds.
		Weight::from_parts(28_067_000, 0)
			.saturating_add(Weight::from_parts(0, 3645))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Teebag EnclaveRegistry (r:1 w:1)
	/// Proof Skipped: Teebag EnclaveRegistry (max_values: None, max_size: None, mode: Measured)
	/// Storage: Teebag EnclaveIdentifier (r:1 w:1)
	/// Proof Skipped: Teebag EnclaveIdentifier (max_values: None, max_size: None, mode: Measured)
	fn force_remove_enclave() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `508`
		//  Estimated: `3973`
		// Minimum execution time: 36_149_000 picoseconds.
		Weight::from_parts(37_475_000, 0)
			.saturating_add(Weight::from_parts(0, 3973))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Teebag EnclaveRegistry (r:4 w:3)
	/// Proof Skipped: Teebag EnclaveRegistry (max_values: None, max_size: None, mode: Measured)
	/// Storage: Teebag EnclaveIdentifier (r:1 w:1)
	/// Proof Skipped: Teebag EnclaveIdentifier (max_values: None, max_size: None, mode: Measured)
	fn force_remove_enclave_by_mrenclave() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `687`
		//  Estimated: `11577`
		// Minimum execution time: 86_498_000 picoseconds.
		Weight::from_parts(88_261_000, 0)
			.saturating_add(Weight::from_parts(0, 11577))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Teebag EnclaveRegistry (r:4 w:3)
	/// Proof Skipped: Teebag EnclaveRegistry (max_values: None, max_size: None, mode: Measured)
	/// Storage: Teebag EnclaveIdentifier (r:1 w:1)
	/// Proof Skipped: Teebag EnclaveIdentifier (max_values: None, max_size: None, mode: Measured)
	fn force_remove_enclave_by_worker_type() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `687`
		//  Estimated: `11577`
		// Minimum execution time: 84_131_000 picoseconds.
		Weight::from_parts(85_447_000, 0)
			.saturating_add(Weight::from_parts(0, 11577))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Teebag AuthorizedEnclave (r:1 w:1)
	/// Proof Skipped: Teebag AuthorizedEnclave (max_values: None, max_size: None, mode: Measured)
	fn force_add_authorized_enclave() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `350`
		//  Estimated: `3815`
		// Minimum execution time: 26_471_000 picoseconds.
		Weight::from_parts(27_320_000, 0)
			.saturating_add(Weight::from_parts(0, 3815))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Teebag AuthorizedEnclave (r:1 w:1)
	/// Proof Skipped: Teebag AuthorizedEnclave (max_values: None, max_size: None, mode: Measured)
	/// Storage: Teebag EnclaveRegistry (r:1 w:0)
	/// Proof Skipped: Teebag EnclaveRegistry (max_values: None, max_size: None, mode: Measured)
	fn force_remove_authorized_enclave() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `382`
		//  Estimated: `3847`
		// Minimum execution time: 37_847_000 picoseconds.
		Weight::from_parts(38_431_000, 0)
			.saturating_add(Weight::from_parts(0, 3847))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Teebag Mode (r:1 w:0)
	/// Proof Skipped: Teebag Mode (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Teebag AuthorizedEnclave (r:1 w:1)
	/// Proof Skipped: Teebag AuthorizedEnclave (max_values: None, max_size: None, mode: Measured)
	/// Storage: Teebag EnclaveRegistry (r:1 w:1)
	/// Proof Skipped: Teebag EnclaveRegistry (max_values: None, max_size: None, mode: Measured)
	/// Storage: Teebag EnclaveIdentifier (r:1 w:1)
	/// Proof Skipped: Teebag EnclaveIdentifier (max_values: None, max_size: None, mode: Measured)
	fn register_enclave_with_ias_attestation() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `357`
		//  Estimated: `3822`
		// Minimum execution time: 2_239_780_000 picoseconds.
		Weight::from_parts(2_263_119_000, 0)
			.saturating_add(Weight::from_parts(0, 3822))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Teebag QuotingEnclaveRegistry (r:1 w:0)
	/// Proof Skipped: Teebag QuotingEnclaveRegistry (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Teebag TcbInfo (r:1 w:0)
	/// Proof Skipped: Teebag TcbInfo (max_values: None, max_size: None, mode: Measured)
	/// Storage: Teebag Mode (r:1 w:0)
	/// Proof Skipped: Teebag Mode (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Teebag AuthorizedEnclave (r:1 w:1)
	/// Proof Skipped: Teebag AuthorizedEnclave (max_values: None, max_size: None, mode: Measured)
	/// Storage: Teebag EnclaveRegistry (r:1 w:1)
	/// Proof Skipped: Teebag EnclaveRegistry (max_values: None, max_size: None, mode: Measured)
	/// Storage: Teebag EnclaveIdentifier (r:1 w:1)
	/// Proof Skipped: Teebag EnclaveIdentifier (max_values: None, max_size: None, mode: Measured)
	fn register_enclave_with_dcap_attestation() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `574`
		//  Estimated: `4039`
		// Minimum execution time: 4_910_006_000 picoseconds.
		Weight::from_parts(4_932_015_000, 0)
			.saturating_add(Weight::from_parts(0, 4039))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Teebag EnclaveRegistry (r:1 w:1)
	/// Proof Skipped: Teebag EnclaveRegistry (max_values: None, max_size: None, mode: Measured)
	/// Storage: Teebag EnclaveIdentifier (r:1 w:1)
	/// Proof Skipped: Teebag EnclaveIdentifier (max_values: None, max_size: None, mode: Measured)
	fn unregister_enclave() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `425`
		//  Estimated: `3890`
		// Minimum execution time: 37_770_000 picoseconds.
		Weight::from_parts(38_544_000, 0)
			.saturating_add(Weight::from_parts(0, 3890))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Teebag QuotingEnclaveRegistry (r:0 w:1)
	/// Proof Skipped: Teebag QuotingEnclaveRegistry (max_values: Some(1), max_size: None, mode: Measured)
	fn register_quoting_enclave() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `94`
		//  Estimated: `1493`
		// Minimum execution time: 2_452_576_000 picoseconds.
		Weight::from_parts(2_478_082_000, 0)
			.saturating_add(Weight::from_parts(0, 1493))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Teebag TcbInfo (r:0 w:1)
	/// Proof Skipped: Teebag TcbInfo (max_values: None, max_size: None, mode: Measured)
	fn register_tcb_info() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `94`
		//  Estimated: `1493`
		// Minimum execution time: 2_668_929_000 picoseconds.
		Weight::from_parts(2_696_537_000, 0)
			.saturating_add(Weight::from_parts(0, 1493))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	fn post_opaque_task() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 13_301_000 picoseconds.
		Weight::from_parts(13_830_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	/// Storage: Teebag EnclaveRegistry (r:1 w:0)
	/// Proof Skipped: Teebag EnclaveRegistry (max_values: None, max_size: None, mode: Measured)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	fn parentchain_block_processed() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `374`
		//  Estimated: `3839`
		// Minimum execution time: 29_965_000 picoseconds.
		Weight::from_parts(30_830_000, 0)
			.saturating_add(Weight::from_parts(0, 3839))
			.saturating_add(T::DbWeight::get().reads(2))
	}
	/// Storage: Teebag EnclaveRegistry (r:1 w:0)
	/// Proof Skipped: Teebag EnclaveRegistry (max_values: None, max_size: None, mode: Measured)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Teebag EnclaveIdentifier (r:1 w:0)
	/// Proof Skipped: Teebag EnclaveIdentifier (max_values: None, max_size: None, mode: Measured)
	/// Storage: Teebag SidechainBlockFinalizationCandidate (r:1 w:1)
	/// Proof Skipped: Teebag SidechainBlockFinalizationCandidate (max_values: None, max_size: None, mode: Measured)
	/// Storage: Teebag LatestSidechainBlockConfirmation (r:0 w:1)
	/// Proof Skipped: Teebag LatestSidechainBlockConfirmation (max_values: None, max_size: None, mode: Measured)
	fn sidechain_block_imported() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `495`
		//  Estimated: `3960`
		// Minimum execution time: 45_486_000 picoseconds.
		Weight::from_parts(46_282_000, 0)
			.saturating_add(Weight::from_parts(0, 3960))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
