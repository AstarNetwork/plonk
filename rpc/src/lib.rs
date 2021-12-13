// Copyright (C) 2020-2021 Artree (JP) LLC.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! RPC interface for the plonk pallet.

use parity_plonk::prelude::PublicParameters;
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

/// A struct that implements the [`PlonkApi`].
pub struct Plonk<C, M> {
    client: Arc<C>,
    _marker: std::marker::PhantomData<M>,
}

impl<C, M> Plonk<C, M> {
    /// Create new `Plonk` with the given reference to the client.
    pub fn new(client: Arc<C>) -> Self {
        Self {
            client,
            _marker: Default::default(),
        }
    }
}

/// Error type of this RPC api.
#[derive(Debug)]
pub enum Error {
    /// The trusted setup was not done
    SetupNotYetError,
    /// The server response failed
    ServerError,
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
                        code: ErrorCode::ServerError(Error::SetupNotYetError as i64),
                        message: "setup not yet error".into(),
                        data: Some(format!("{:?}", Error::SetupNotYetError).into()),
                    })
                }
            },
            Err(e) => {
                return Err(RpcError {
                    code: ErrorCode::ServerError(Error::ServerError as i64),
                    message: "server error".into(),
                    data: Some(format!("{:?}", e).into()),
                })
            }
        }
    }
}
