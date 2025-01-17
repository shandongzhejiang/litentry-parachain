/*
	Copyright 2021 Integritee AG and Supercomputing Systems AG

	Licensed under the Apache License, Version 2.0 (the "License");
	you may not use this file except in compliance with the License.
	You may obtain a copy of the License at

		http://www.apache.org/licenses/LICENSE-2.0

	Unless required by applicable law or agreed to in writing, software
	distributed under the License is distributed on an "AS IS" BASIS,
	WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
	See the License for the specific language governing permissions and
	limitations under the License.

*/

#[cfg(all(not(feature = "std"), feature = "sgx"))]
use crate::sgx_reexport_prelude::*;

use crate::traits::AuthorApi;
use codec::Encode;
use itp_sgx_crypto::ShieldingCryptoEncrypt;
use itp_stf_primitives::types::{ShardIdentifier, TrustedOperation as StfTrustedOperation};
use itp_types::RsaRequest;
use jsonrpc_core::futures::executor;
use sp_core::H256;
use std::fmt::Debug;

/// Test utility function to submit a trusted operation on an RPC author
pub fn submit_operation_to_top_pool<R, S, TCS, G>(
	author: &R,
	top: &StfTrustedOperation<TCS, G>,
	shielding_key: &S,
	shard: ShardIdentifier,
) -> Result<(H256, RsaRequest), jsonrpc_core::Error>
where
	R: AuthorApi<H256, H256, TCS, G>,
	S: ShieldingCryptoEncrypt,
	S::Error: Debug,
	TCS: PartialEq + Encode + Debug + Send + Sync,
	G: PartialEq + Encode + Debug + Send + Sync,
{
	let top_encrypted = shielding_key.encrypt(&top.encode()).unwrap();
	let submit_future =
		async { author.watch_top(RsaRequest::new(shard, top_encrypted.clone())).await };
	let hash = executor::block_on(submit_future)?;
	Ok((hash, RsaRequest::new(shard, top_encrypted)))
}
