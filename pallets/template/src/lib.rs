pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use max_encoded_len::MaxEncodedLen;
    use frame_system::pallet_prelude::*;
	use super::*;
	use codec::{Encode, Decode};

	#[derive(Debug, PartialEq, Encode, Decode, MaxEncodedLen)]
	pub struct Hello {
		greet: u64
	}

	#[pallet::config]
	pub trait Config: frame_system::Config {}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::generate_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn srs)]
	pub type Srs<T: Config> = StorageValue<_, Hello>;

	// #[pallet::genesis_config]
	// pub struct GenesisConfig {
	// 	srs: String,
	// }

	// #[pallet::genesis_build]
	// impl<T: Config> GenesisBuild<T> for GenesisConfig {
	// 	fn build(&self) {
	// 		<Srs<T>>::put(&self.srs);
	// 	}
	// }
}
