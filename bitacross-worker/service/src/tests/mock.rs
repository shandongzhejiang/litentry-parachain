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

use itp_node_api::api_client::{ApiResult, PalletTeebagApi};
use itp_types::{AccountId, Enclave, MrEnclave, ShardIdentifier, WorkerType, H256 as Hash};
use std::collections::HashSet;

pub struct TestNodeApi;

pub const W1_URL: &str = "127.0.0.1:22222";
pub const W2_URL: &str = "127.0.0.1:33333";

pub fn enclaves() -> Vec<Enclave> {
	vec![
		Enclave::new(WorkerType::BitAcross).with_url(W1_URL.into()),
		Enclave::new(WorkerType::BitAcross).with_url(W2_URL.into()),
	]
}

impl PalletTeebagApi for TestNodeApi {
	type Hash = Hash;

	fn enclave(&self, _account: &AccountId, _at_block: Option<Hash>) -> ApiResult<Option<Enclave>> {
		unreachable!()
	}
	fn enclave_count(&self, _worker_type: WorkerType, _at_block: Option<Hash>) -> ApiResult<u64> {
		unreachable!()
	}

	fn all_enclaves(
		&self,
		_worker_type: WorkerType,
		_at_block: Option<Hash>,
	) -> ApiResult<Vec<Enclave>> {
		Ok(enclaves())
	}

	fn primary_enclave_identifier_for_shard(
		&self,
		worker_type: WorkerType,
		shard: &ShardIdentifier,
		at_block: Option<Self::Hash>,
	) -> ApiResult<Option<AccountId>> {
		unreachable!()
	}

	fn primary_enclave_for_shard(
		&self,
		worker_type: WorkerType,
		shard: &ShardIdentifier,
		at_block: Option<Self::Hash>,
	) -> ApiResult<Option<Enclave>> {
		unreachable!()
	}
}
