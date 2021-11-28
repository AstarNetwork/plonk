use dusk_plonk::prelude::PublicParameters;
use jsonrpc_core::{Error as RpcError, ErrorCode, Result};
use jsonrpc_derive::rpc;
use plonk_runtime_api::PlonkApi as PlonkRuntimeApi;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};
use std::sync::Arc;

#[rpc]
pub trait PlonkApi<BlockHash> {
    #[rpc(name = "plonk_getPublicParameters")]
    fn get_public_parameters(&self, at: Option<BlockHash>) -> Result<PublicParameters>;
}

pub struct Plonk<C, M> {
    client: Arc<C>,
    _marker: std::marker::PhantomData<M>,
}

impl<C, M> Plonk<C, M> {
    pub fn new(client: Arc<C>) -> Self {
        Self {
            client,
            _marker: Default::default(),
        }
    }
}
