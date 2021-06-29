#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use max_encoded_len::MaxEncodedLen;
    use frame_system::pallet_prelude::*;
	use super::*;
	use codec::{Encode, Decode};
	use serde_derive::{Serialize, Deserialize};

	#[derive(Debug, PartialEq, Encode, Decode, MaxEncodedLen, Serialize, Deserialize)]
	pub struct SrsContents {
		pairing: u64
	}

	#[pallet::config]
	pub trait Config: frame_system::Config {}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::generate_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn srs)]
	pub type Srs<T: Config> = StorageValue<_, SrsContents>;

	#[pallet::genesis_config]
	pub struct GenesisConfig {
		srs: SrsContents,
	}

	#[cfg(feature = "std")]
	impl Default for GenesisConfig {
		fn default() -> Self {
			Self {
				srs: SrsContents {
					pairing: 12345
				},
			}
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig {
		fn build(&self) {
			<Srs<T>>::put(&self.srs);
		}
	}
}
