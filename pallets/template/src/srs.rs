use max_encoded_len::MaxEncodedLen;
use codec::{Encode, Decode};
use serde::{Serialize, Deserialize};
use jsonrpc_core::Result;
use jsonrpc_derive::rpc;

#[derive(Debug, PartialEq, Encode, Decode, MaxEncodedLen, Serialize, Deserialize)]
pub struct SrsContents {
	pub pairing: u64
}

#[rpc]
pub trait SrsContentsRPC {
    #[rpc(name = "get_srs")]
    fn get_srs(&self) -> Result<u64>;
}
