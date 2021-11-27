#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]

pub use pallet::*;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
    use dusk_plonk::prelude::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use rand_core::OsRng;

    #[pallet::config]
    pub trait Config: frame_system::Config {
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
            let pp = PublicParameters::setup(1 << val, &mut OsRng).unwrap();
            PublicParameter::<T>::put(&pp);

            Event::<T>::TrustedSetup(pp);
            Ok(().into())
        }

        #[pallet::weight(10_000)]
        pub fn verify(
            origin: OriginFor<T>,
            pp: PublicParameters,
            vd: VerifierData,
            proof: Proof,
            // public_inputs: &'static [PublicInputValue],
            // transcript_init: &'static [u8],
        ) -> DispatchResultWithPostInfo {
            let _ = ensure_signed(origin)?;
            Ok(().into())
        }
    }
}

impl<T: Config> Pallet<T> {}
