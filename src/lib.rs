#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]

pub use pallet::*;
use parity_scale_codec::{Decode, Encode};

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
    use super::Transcript;
    use dusk_plonk::prelude::*;
    use frame_support::dispatch::{DispatchErrorWithPostInfo, PostDispatchInfo};
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use rand_core::OsRng;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type CustomCircuit: Circuit;
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::storage]
    #[pallet::getter(fn public_parameter)]
    pub type PublicParameter<T: Config> = StorageValue<_, PublicParameters>;

    #[pallet::event]
    #[pallet::metadata(u32 = "Metadata")]
    pub enum Event<T: Config> {
        TrustedSetup(PublicParameters),
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn trusted_setup(origin: OriginFor<T>, val: u32) -> DispatchResultWithPostInfo {
            let _ = ensure_signed(origin)?;
            match Self::public_parameter() {
                Some(_) => {
                    return Err(DispatchErrorWithPostInfo {
                        post_info: PostDispatchInfo::from(()),
                        error: DispatchError::Other("already setup"),
                    })
                }
                None => {
                    let pp = PublicParameters::setup(1 << val, &mut OsRng).unwrap();
                    PublicParameter::<T>::put(&pp);
                    Event::<T>::TrustedSetup(pp);
                    return Ok(().into());
                }
            }
        }

        #[pallet::weight(10_000)]
        pub fn verify(
            origin: OriginFor<T>,
            vd: VerifierData,
            proof: Proof,
            public_inputs: Vec<PublicInputValue>,
            transcript_init: Transcript,
        ) -> DispatchResultWithPostInfo {
            let _ = ensure_signed(origin)?;
            match Self::public_parameter() {
                Some(pp) => {
                    T::CustomCircuit::verify(&pp, &vd, &proof, &public_inputs, transcript_init.0)
                        .expect("verify process is in valid");
                    Ok(().into())
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
}

impl<T: Config> Pallet<T> {}

#[derive(Debug, PartialEq, Clone, Encode)]
pub struct Transcript(&'static [u8]);

#[allow(unconditional_recursion)]
impl Decode for Transcript {
    fn decode<I: parity_scale_codec::Input>(
        input: &mut I,
    ) -> Result<Self, parity_scale_codec::Error> {
        Decode::decode(input)
    }
}
