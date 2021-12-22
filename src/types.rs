pub use fullcodec_jubjub::GENERATOR_EXTENDED;
pub use fullcodec_plonk::prelude::{
    BlsScalar, Circuit, Constraint, Error as PlonkError, JubJubAffine, JubJubScalar, Proof,
    PublicInputValue, PublicParameters, TurboComposer, VerifierData,
};
pub use fullcodec_rand_xorshift::XorShiftRng as FullcodecRng;
use parity_scale_codec::{Decode, Encode};

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
