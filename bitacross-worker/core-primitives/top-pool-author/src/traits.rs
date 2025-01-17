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
use codec::Encode;
use core::fmt::Debug;

use crate::error::Result;
use itp_stf_primitives::types::{
	AccountId, TrustedOperation as StfTrustedOperation, TrustedOperationOrHash,
};
use itp_top_pool::primitives::{PoolFuture, PoolStatus};
use itp_types::{BlockHash as SidechainBlockHash, DecryptableRequest, ShardIdentifier, H256};
use jsonrpc_core::Error as RpcError;
use std::vec::Vec;

/// Trait alias for a full STF author API
pub trait FullAuthor<
	TCS: PartialEq + Encode + Debug + Send + Sync + 'static,
	G: PartialEq + Encode + Debug + Send + Sync + 'static,
> = AuthorApi<H256, H256, TCS, G> + OnBlockImported<Hash = H256> + Send + Sync + 'static;

/// Authoring RPC API
pub trait AuthorApi<Hash, BlockHash, TCS, G>
where
	TCS: PartialEq + Encode + Debug + Send + Sync,
	G: PartialEq + Encode + Debug + Send + Sync,
{
	/// Submit encoded extrinsic for inclusion in block.
	fn submit_top<R: DecryptableRequest + Encode>(&self, req: R) -> PoolFuture<Hash, RpcError>;

	/// Return hash of Trusted Operation
	fn hash_of(&self, xt: &StfTrustedOperation<TCS, G>) -> Hash;

	/// Returns all pending operations, potentially grouped by sender.
	fn pending_tops(&self, shard: ShardIdentifier) -> Result<Vec<Vec<u8>>>;

	/// Returns all pending trusted getters.
	fn get_pending_getters(&self, shard: ShardIdentifier) -> Vec<StfTrustedOperation<TCS, G>>;

	/// Returns all pending trusted calls (in ready state).
	fn get_pending_trusted_calls(&self, shard: ShardIdentifier)
		-> Vec<StfTrustedOperation<TCS, G>>;

	/// Returns pool status
	fn get_status(&self, shard: ShardIdentifier) -> PoolStatus;

	/// Returns all pending trusted calls for a given `account`
	fn get_pending_trusted_calls_for(
		&self,
		shard: ShardIdentifier,
		account: &AccountId,
	) -> Vec<StfTrustedOperation<TCS, G>>;

	/// returns all shards which are currently present in the tops in the pool
	fn get_shards(&self) -> Vec<ShardIdentifier>;

	/// returns all shards which are handled by our worker
	fn list_handled_shards(&self) -> Vec<ShardIdentifier>;

	/// Remove a collection of trusted operations from the pool.
	/// Return operations that were not successfully removed.
	fn remove_calls_from_pool(
		&self,
		shard: ShardIdentifier,
		executed_calls: Vec<(TrustedOperationOrHash<TCS, G>, bool)>,
	) -> Vec<TrustedOperationOrHash<TCS, G>>;

	/// Submit a request to watch.
	///
	/// See [`TrustedOperationStatus`](sp_transaction_pool::TrustedOperationStatus) for details on transaction
	/// life cycle.
	fn watch_top<R: DecryptableRequest + Encode>(&self, request: R) -> PoolFuture<Hash, RpcError>;

	/// Litentry: set the rpc response value
	fn update_connection_state(&self, updates: Vec<(Hash, (Vec<u8>, bool))>);

	/// Litentry: swap the old hash with the new one in rpc connection registry
	fn swap_rpc_connection_hash(&self, old_hash: Hash, new_hash: Hash);
}

/// Trait to notify listeners/observer of a newly created block
pub trait OnBlockImported {
	type Hash;

	fn on_block_imported(&self, hashes: &[Self::Hash], block_hash: SidechainBlockHash);
}
