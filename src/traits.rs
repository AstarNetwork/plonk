use crate::types::*;
use frame_support::pallet_prelude::DispatchResultWithPostInfo;
use sp_std::vec::Vec;

pub trait Plonk<AccountId> {
    type CustomCircuit: Circuit;

    fn trusted_setup(who: &AccountId, val: u32, rng: ParityRng) -> DispatchResultWithPostInfo;

    fn verify(
        who: &AccountId,
        vd: VerifierData,
        proof: Proof,
        public_inputs: Vec<PublicInputValue>,
        transcript_init: Transcript,
    ) -> DispatchResultWithPostInfo;
}
