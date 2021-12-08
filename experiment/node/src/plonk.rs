use jsonrpc_core::Result;
use jsonrpc_derive::rpc;

#[rpc]
pub trait PlonkApi {
    #[rpc(name = "plonk_generatePublicParameters")]
    fn generate_public_parameters(&self, val: u64) -> Result<u64>;

    #[rpc(name = "plonk_getPublicParameters")]
    fn get_public_parameters(&self) -> Result<u64>;
}

/// A struct that implements the `SillyRpc`
pub struct Plonk;

impl PlonkApi for Plonk {
    fn generate_public_parameters(&self, val: u64) -> Result<u64> {
        Ok(2 * val)
    }

    fn get_public_parameters(&self) -> Result<u64> {
        Ok(7)
    }
}
