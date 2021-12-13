pub use parity_plonk::prelude::{
    BlsScalar, Circuit, Constraint, Error as PlonkError, JubJubAffine, JubJubScalar, Proof,
    PublicInputValue, PublicParameters, TurboComposer, VerifierData,
};
pub use parity_rand_xorshift::XorShiftRng as ParityRng;
use parity_scale_codec::{Decode, Encode};
pub use plonk_jubjub::GENERATOR_EXTENDED;

/// The struct for Merlin transcript and used for proof verify
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
