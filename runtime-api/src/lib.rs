#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::unnecessary_mut_passed)]

use dusk_plonk::prelude::PublicParameters;

sp_api::decl_runtime_apis! {
	pub trait PlonkApi {
		fn get_public_parameters() -> Option<PublicParameters>;
	}
}
