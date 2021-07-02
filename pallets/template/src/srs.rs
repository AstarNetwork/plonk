use max_encoded_len::MaxEncodedLen;
use codec::{Encode, Decode};
use serde_derive::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Encode, Decode, MaxEncodedLen, Serialize, Deserialize)]
pub struct SrsContents {
    pub pairing: u64
}
