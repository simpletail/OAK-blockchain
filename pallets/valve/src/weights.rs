// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_valve
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-09-15, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `actions-runner-1`, CPU: `Intel(R) Xeon(R) E-2388G CPU @ 3.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("turing-dev"), DB CACHE: 1024

// Executed Command:
// ./oak-collator
// benchmark
// pallet
// --chain
// turing-dev
// --execution
// wasm
// --wasm-execution
// compiled
// --pallet
// pallet_valve
// --extrinsic
// *
// --repeat
// 20
// --steps
// 50
// --output
// ./valve-raw-weights.rs
// --template
// ./.maintain/frame-weight-template.hbs

// Summary:
//:close_valve 17_690_000
//:open_valve 17_999_000
//:close_pallet_gate_new 23_010_000
//:close_pallet_gate_existing 11_142_000
//:open_pallet_gate 23_062_000
//:open_pallet_gates 23_877_000
//:stop_scheduled_tasks 17_241_000
//:start_scheduled_tasks 17_663_000

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_valve.
pub trait WeightInfo {
	fn close_valve() -> Weight;
	fn open_valve() -> Weight;
	fn close_pallet_gate_new() -> Weight;
	fn close_pallet_gate_existing() -> Weight;
	fn open_pallet_gate() -> Weight;
	fn open_pallet_gates() -> Weight;
	fn stop_scheduled_tasks() -> Weight;
	fn start_scheduled_tasks() -> Weight;
}

/// Weights for pallet_valve using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: TechnicalMembership Members (r:1 w:0)
	// Storage: Valve ValveClosed (r:1 w:1)
	fn close_valve() -> Weight {
		(17_690_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: TechnicalMembership Members (r:1 w:0)
	// Storage: Valve ValveClosed (r:1 w:1)
	fn open_valve() -> Weight {
		(17_999_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: TechnicalMembership Members (r:1 w:0)
	// Storage: Valve ValveClosed (r:1 w:0)
	// Storage: Valve ClosedPallets (r:1 w:1)
	// Storage: Valve ClosedPalletCount (r:1 w:1)
	fn close_pallet_gate_new() -> Weight {
		(23_010_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: TechnicalMembership Members (r:1 w:0)
	// Storage: Valve ValveClosed (r:1 w:0)
	// Storage: Valve ClosedPallets (r:1 w:1)
	fn close_pallet_gate_existing() -> Weight {
		(11_142_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: TechnicalMembership Members (r:1 w:0)
	// Storage: Valve ValveClosed (r:1 w:0)
	// Storage: Valve ClosedPallets (r:1 w:1)
	// Storage: Valve ClosedPalletCount (r:1 w:1)
	fn open_pallet_gate() -> Weight {
		(23_062_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: TechnicalMembership Members (r:1 w:0)
	// Storage: Valve ClosedPalletCount (r:1 w:1)
	// Storage: Valve ClosedPallets (r:0 w:5)
	fn open_pallet_gates() -> Weight {
		(23_877_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	// Storage: TechnicalMembership Members (r:1 w:0)
	// Storage: AutomationTime Shutdown (r:1 w:1)
	fn stop_scheduled_tasks() -> Weight {
		(17_241_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: TechnicalMembership Members (r:1 w:0)
	// Storage: AutomationTime Shutdown (r:1 w:1)
	fn start_scheduled_tasks() -> Weight {
		(17_663_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: TechnicalMembership Members (r:1 w:0)
	// Storage: Valve ValveClosed (r:1 w:1)
	fn close_valve() -> Weight {
		(17_690_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: TechnicalMembership Members (r:1 w:0)
	// Storage: Valve ValveClosed (r:1 w:1)
	fn open_valve() -> Weight {
		(17_999_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: TechnicalMembership Members (r:1 w:0)
	// Storage: Valve ValveClosed (r:1 w:0)
	// Storage: Valve ClosedPallets (r:1 w:1)
	// Storage: Valve ClosedPalletCount (r:1 w:1)
	fn close_pallet_gate_new() -> Weight {
		(23_010_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: TechnicalMembership Members (r:1 w:0)
	// Storage: Valve ValveClosed (r:1 w:0)
	// Storage: Valve ClosedPallets (r:1 w:1)
	fn close_pallet_gate_existing() -> Weight {
		(11_142_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: TechnicalMembership Members (r:1 w:0)
	// Storage: Valve ValveClosed (r:1 w:0)
	// Storage: Valve ClosedPallets (r:1 w:1)
	// Storage: Valve ClosedPalletCount (r:1 w:1)
	fn open_pallet_gate() -> Weight {
		(23_062_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: TechnicalMembership Members (r:1 w:0)
	// Storage: Valve ClosedPalletCount (r:1 w:1)
	// Storage: Valve ClosedPallets (r:0 w:5)
	fn open_pallet_gates() -> Weight {
		(23_877_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(6 as Weight))
	}
	// Storage: TechnicalMembership Members (r:1 w:0)
	// Storage: AutomationTime Shutdown (r:1 w:1)
	fn stop_scheduled_tasks() -> Weight {
		(17_241_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: TechnicalMembership Members (r:1 w:0)
	// Storage: AutomationTime Shutdown (r:1 w:1)
	fn start_scheduled_tasks() -> Weight {
		(17_663_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
}
