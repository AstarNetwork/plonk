use crate::{
    Circuit, Proof, PublicInputValue, PublicParameters, Transcript, VerifierData, XorShiftRng,
};
use frame_support::pallet_prelude::DispatchResultWithPostInfo;

pub trait Plonk<AccountId> {
    type CustomCircuit: Circuit;

    fn trusted_setup(who: &AccountId, val: u32, rng: XorShiftRng) -> DispatchResultWithPostInfo;

    fn get_public_parameters() -> Option<PublicParameters>;

    fn verify(
        who: &AccountId,
        vd: VerifierData,
        proof: Proof,
        public_inputs: Vec<PublicInputValue>,
        transcript_init: Transcript,
    ) -> DispatchResultWithPostInfo;
}
