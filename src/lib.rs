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

    /// The module's configuration trait.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::storage]
    #[pallet::getter(fn public_parameter)]
    pub type PublicParameter<T: Config> = StorageValue<_, PublicParameters>;

    #[pallet::genesis_config]
    pub struct GenesisConfig {
        pub public_parameter: PublicParameters,
    }

    #[cfg(feature = "std")]
    impl Default for GenesisConfig {
        fn default() -> Self {
            Self {
                public_parameter: PublicParameters::setup(1 << 12, &mut OsRng).unwrap(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig {
        fn build(&self) {
            <PublicParameter<T>>::put(PublicParameters::setup(1 << 12, &mut OsRng).unwrap());
            todo!()
        }
    }

    #[pallet::event]
    #[pallet::metadata(u32 = "Metadata")]
    pub enum Event<T: Config> {}

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {}
}

impl<T: Config> Pallet<T> {}
