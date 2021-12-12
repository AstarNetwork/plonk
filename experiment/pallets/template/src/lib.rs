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

mod traits;

pub use dusk_jubjub::GENERATOR_EXTENDED;
pub use dusk_plonk::prelude::{
    BlsScalar, Circuit, Constraint, Error, JubJubAffine, JubJubScalar, Proof, PublicInputValue,
    PublicParameters, TurboComposer, VerifierData,
};
use frame_support::dispatch::{DispatchErrorWithPostInfo, PostDispatchInfo};
use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
use parity_scale_codec::{Decode, Encode};
use rand_xorshift::XorShiftRng;
use sp_std::vec::Vec;
pub use traits::Plonk;

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The circuit customized by developer.
        type CustomCircuit: Circuit;

        /// The overarching event type.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::storage]
    #[pallet::getter(fn public_parameter)]
    /// The setup parameter referred to as SRS
    pub type PublicParameter<T: Config> = StorageValue<_, PublicParameters>;

    #[pallet::event]
    #[pallet::metadata(u32 = "Metadata")]
    pub enum Event<T: Config> {
        /// The event called when setup parameter
        TrustedSetup(PublicParameters),
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Error names should be descriptive.
        NoneValue,
        /// Errors should have helpful documentation associated with them.
        StorageOverflow,
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// The function called when we setup the parameters
        #[pallet::weight(10_000)]
        pub fn trusted_setup(
            origin: OriginFor<T>,
            val: u32,
            rng: XorShiftRng,
        ) -> DispatchResultWithPostInfo {
            let transactor = ensure_signed(origin)?;
            <Self as Plonk<_>>::trusted_setup(&transactor, val, rng)?;
            Ok(().into())
        }

        /// The function called when we verify the statement
        #[pallet::weight(10_000)]
        pub fn verify(
            origin: OriginFor<T>,
            vd: VerifierData,
            proof: Proof,
            public_inputs: Vec<PublicInputValue>,
            transcript_init: Transcript,
        ) -> DispatchResultWithPostInfo {
            let transactor = ensure_signed(origin)?;
            <Self as Plonk<_>>::verify(&transactor, vd, proof, public_inputs, transcript_init)?;
            Ok(().into())
        }
    }
}

impl<T: Config> Plonk<T::AccountId> for Pallet<T> {
    type CustomCircuit = T::CustomCircuit;

    fn trusted_setup(
        _who: &T::AccountId,
        val: u32,
        mut rng: XorShiftRng,
    ) -> DispatchResultWithPostInfo {
        match Self::public_parameter() {
            Some(_) => {
                return Err(DispatchErrorWithPostInfo {
                    post_info: PostDispatchInfo::from(()),
                    error: DispatchError::Other("already setup"),
                })
            }
            None => {
                let pp = PublicParameters::setup(1 << val, &mut rng).unwrap();
                PublicParameter::<T>::put(&pp);
                Event::<T>::TrustedSetup(pp);
                return Ok(().into());
            }
        }
    }

    fn get_public_parameters() -> Option<PublicParameters> {
        PublicParameter::<T>::get()
    }

    fn verify(
        _who: &T::AccountId,
        vd: VerifierData,
        proof: Proof,
        public_inputs: Vec<PublicInputValue>,
        transcript_init: Transcript,
    ) -> DispatchResultWithPostInfo {
        match Self::public_parameter() {
            Some(pp) => {
                match T::CustomCircuit::verify(&pp, &vd, &proof, &public_inputs, transcript_init.0)
                {
                    Ok(_) => return Ok(().into()),
                    Err(_) => {
                        return Err(DispatchErrorWithPostInfo {
                            post_info: PostDispatchInfo::from(()),
                            error: DispatchError::Other("invalid proof"),
                        })
                    }
                }
            }
            None => {
                return Err(DispatchErrorWithPostInfo {
                    post_info: PostDispatchInfo::from(()),
                    error: DispatchError::Other("setup not yet"),
                })
            }
        }
    }
}

/// The struct for Merlin transcript
#[derive(Debug, PartialEq, Clone, Encode)]
pub struct Transcript(pub &'static [u8]);

#[allow(unconditional_recursion)]
impl Decode for Transcript {
    fn decode<I: parity_scale_codec::Input>(
        input: &mut I,
    ) -> Result<Self, parity_scale_codec::Error> {
        Decode::decode(input)
    }
}
