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

#[derive(Debug)]
pub enum Error {
    SetupNotYetError,
}

impl<C, Block> PlonkApi<<Block as BlockT>::Hash> for Plonk<C, Block>
where
    Block: BlockT,
    C: Send + Sync + 'static,
    C: ProvideRuntimeApi<Block>,
    C: HeaderBackend<Block>,
    C::Api: PlonkRuntimeApi<Block>,
{
    fn get_public_parameters(
        &self,
        at: Option<<Block as BlockT>::Hash>,
    ) -> Result<PublicParameters> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));

        let runtime_api_result = api.get_public_parameters(&at);
        match runtime_api_result {
            Ok(r) => match r {
                Some(p) => return Ok(p),
                None => {
                    return Err(RpcError {
                        code: ErrorCode::ServerError(9875),
                        message: "setup not yet error".into(),
                        data: Some(format!("{:?}", Error::SetupNotYetError).into()),
                    })
                }
            },
            Err(e) => {
                return Err(RpcError {
                    code: ErrorCode::ServerError(9876),
                    message: "server error".into(),
                    data: Some(format!("{:?}", e).into()),
                })
            }
        }
    }
}
