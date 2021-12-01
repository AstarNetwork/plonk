#![cfg_attr(not(feature = "std"), no_std)]

// Copyright (C) 2020-2021 Artree (JP) LLC.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # Plonk Pallet
//!
//! - [`Config`]
//! - [`Call`]
//! - [`Pallet`]
//!
//! ## Overview
//!
//! The Plonk pallet provides functions for:
//!
//! - Setup parameters
//! - Verify zkp proof
//!
//! ### Terminology
//!
//! - **Custome Circuit** The circuit type should be replaced with your own circuit.
//! This circuit should be defined on both blockchain runtime and offchain client.
//!
//! - **Public Parameter** The parameter generated during setup. The users can use
//! this parameter to prove their transaction validity. This parameter can be gotten
//! throught RPC client.

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

use dusk_plonk::prelude::{Circuit, Proof, PublicInputValue, PublicParameters, VerifierData};
use sp_std::vec::Vec;
use sp_std::vec;

#[frame_support::pallet]
pub mod pallet {
	use super::{
        Circuit, Proof, PublicInputValue, PublicParameters, Vec, VerifierData,
    };
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn public_parameter)]
    /// The setup parameter referred to as SRS
    pub type PublicParameter<T: Config> = StorageValue<_, PublicParameters>;

	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		SomethingStored(u32, T::AccountId),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T:Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			Self::deposit_event(Event::SomethingStored(something, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(().into())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let _who = ensure_signed(origin)?;

			Ok(().into())
		}
	}
}
