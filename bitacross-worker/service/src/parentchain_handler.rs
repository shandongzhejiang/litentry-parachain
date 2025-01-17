/*
	Copyright 2021 Integritee AG and Supercomputing Systems AG
	Copyright (C) 2017-2019 Baidu, Inc. All Rights Reserved.

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

use crate::error::{Error, ServiceResult};
use codec::{Decode, Encode};
use humantime::format_duration;
use ita_parentchain_interface::integritee::Header;
use itc_parentchain::{
	light_client::light_client_init_params::{GrandpaParams, SimpleParams},
	primitives::{ParentchainId, ParentchainInitParams},
};
use itp_api_client_types::ParentchainApi;
use itp_enclave_api::{enclave_base::EnclaveBase, sidechain::Sidechain};
use itp_node_api::api_client::ChainApi;
use itp_storage::StorageProof;
use itp_time_utils::duration_now;
use itp_types::ShardIdentifier;
use log::*;
use rayon::prelude::*;
use sp_consensus_grandpa::VersionedAuthorityList;
use sp_runtime::traits::Header as HeaderTrait;
use std::{cmp::min, sync::Arc, time::Duration};
use substrate_api_client::{
	ac_primitives::{Block, Header as HeaderT},
	GetChainInfo,
};

const BLOCK_SYNC_BATCH_SIZE: u32 = 1000;

pub trait HandleParentchain {
	/// Initializes all parentchain specific components on the enclave side.
	/// Returns the latest synced block header.
	fn init_parentchain_components(&self) -> ServiceResult<Header>;

	/// Fetches the parentchain blocks to sync from the parentchain and feeds them to the enclave.
	/// Returns the latest synced block header.
	///
	/// Litentry: `overriden_start_block` to forcibly start from the given parentchain block number
	fn sync_parentchain_until_latest_finalized(
		&self,
		last_synced_header: Header,
		overriden_start_block: u32,
		shard: ShardIdentifier,
		immediate_import: bool,
	) -> ServiceResult<Header>;

	/// Syncs and directly imports parentchain blocks from the latest synced header
	/// until at least the specified until_header.
	///
	/// Litentry: `overriden_start_block` to forcibly start from the given parentchain block number
	fn await_sync_and_import_parentchain_until_at_least(
		&self,
		last_synced_header: &Header,
		until_header: &Header,
		overriden_start_block: u32,
		shard: ShardIdentifier,
	) -> ServiceResult<Header>;
}

/// Handles the interaction between parentchain and enclave.
pub(crate) struct ParentchainHandler<ParentchainApi, EnclaveApi> {
	parentchain_api: ParentchainApi,
	enclave_api: Arc<EnclaveApi>,
	pub parentchain_init_params: ParentchainInitParams,
}

// #TODO: #1451: Reintroduce `ParentchainApi: ChainApi` once there is no trait bound conflict
// any more with the api-clients own trait definitions.
impl<EnclaveApi> ParentchainHandler<ParentchainApi, EnclaveApi>
where
	EnclaveApi: EnclaveBase,
{
	pub fn new(
		parentchain_api: ParentchainApi,
		enclave_api: Arc<EnclaveApi>,
		parentchain_init_params: ParentchainInitParams,
	) -> Self {
		Self { parentchain_api, enclave_api, parentchain_init_params }
	}

	// FIXME: Necessary in the future? Fix with #1080
	pub fn new_with_automatic_light_client_allocation(
		parentchain_api: ParentchainApi,
		enclave_api: Arc<EnclaveApi>,
		id: ParentchainId,
		shard: ShardIdentifier,
	) -> ServiceResult<Self> {
		let genesis_hash = parentchain_api.get_genesis_hash()?;
		let genesis_header =
			parentchain_api.header(Some(genesis_hash))?.ok_or(Error::MissingGenesisHeader)?;

		let parentchain_init_params: ParentchainInitParams = if parentchain_api
			.is_grandpa_available()?
		{
			let grandpas = parentchain_api.grandpa_authorities(Some(genesis_hash))?;
			let grandpa_proof = parentchain_api.grandpa_authorities_proof(Some(genesis_hash))?;

			debug!("[{:?}] Grandpa Authority List: \n {:?} \n ", id, grandpas);

			let authority_list = VersionedAuthorityList::from(grandpas);

			(
				id,
				shard,
				GrandpaParams::new(
					// #TODO: #1451: clean up type hacks
					Header::decode(&mut genesis_header.encode().as_slice())?,
					authority_list.into(),
					grandpa_proof,
				),
			)
				.into()
		} else {
			(
				id,
				shard,
				SimpleParams::new(
					// #TODO: #1451: clean up type hacks
					Header::decode(&mut genesis_header.encode().as_slice())?,
				),
			)
				.into()
		};

		Ok(Self::new(parentchain_api, enclave_api, parentchain_init_params))
	}

	pub fn parentchain_api(&self) -> &ParentchainApi {
		&self.parentchain_api
	}

	pub fn parentchain_id(&self) -> &ParentchainId {
		self.parentchain_init_params.id()
	}
}

impl<EnclaveApi> HandleParentchain for ParentchainHandler<ParentchainApi, EnclaveApi>
where
	EnclaveApi: Sidechain + EnclaveBase,
{
	fn init_parentchain_components(&self) -> ServiceResult<Header> {
		Ok(self
			.enclave_api
			.init_parentchain_components(self.parentchain_init_params.clone())?)
	}

	fn sync_parentchain_until_latest_finalized(
		&self,
		last_synced_header: Header,
		overriden_start_block: u32,
		shard: ShardIdentifier,
		immediate_import: bool,
	) -> ServiceResult<Header> {
		let id = self.parentchain_id();
		trace!("[{:?}] Getting current head", id);
		let curr_block = self
			.parentchain_api
			.last_finalized_block()?
			.ok_or(Error::MissingLastFinalizedBlock)?;
		let curr_block_number = curr_block.block.header().number();
		let last_synced_header_number = last_synced_header.number;
		// verify that the last_synced_header is indeed a block from this chain
		self.parentchain_api
			.get_block(Some(last_synced_header.hash()))?
			.ok_or_else(|| Error::UnknownBlockHeader(last_synced_header.hash()))?;

		info!(
			"[{:?}] Syncing blocks from {} to {}",
			id, last_synced_header_number, curr_block_number
		);
		let creation_info = self.enclave_api.get_shard_creation_info(&shard)?;
		let maybe_creation_block = if let Some(creation_block) = creation_info.for_parentchain(*id)
		{
			trace!("[{:?}] shard creation block: {:?}", id, creation_block);
			Some(creation_block)
		} else {
			None
		};

		let start_time = duration_now();
		let mut until_synced_header = last_synced_header;
		let mut start_block = until_synced_header.number + 1;
		if overriden_start_block > start_block {
			start_block = overriden_start_block;
			// ask the enclave to ignore the parentchain block import validation until `overriden_start_block`
			// TODO: maybe ignoring the next block import is enough, since the given `overriden_start_block`
			//       should be the very first parentchain block to be imported
			self.enclave_api
				.ignore_parentchain_block_import_validation_until(overriden_start_block)?;
		}

		loop {
			let chunk_range =
				start_block..min(start_block + BLOCK_SYNC_BATCH_SIZE, curr_block_number);

			let start_fetch_time = duration_now();

			let block_chunk_to_sync = chunk_range
				.into_par_iter()
				.filter_map(|block_number| {
					self.parentchain_api
						.get_block_by_number(block_number)
						.expect("failed to get block")
				})
				.collect::<Vec<_>>();

			debug!(
				"[{:?}] Fetched {} blocks in {}",
				id,
				block_chunk_to_sync.len(),
				format_duration(duration_now().saturating_sub(start_fetch_time))
			);

			if block_chunk_to_sync.len() == BLOCK_SYNC_BATCH_SIZE as usize {
				let now = duration_now();
				let total_blocks = curr_block_number.saturating_sub(last_synced_header_number);
				let remaining_blocks = curr_block_number.saturating_sub(until_synced_header.number);
				let remaining_time_estimate: Duration = (now.saturating_sub(start_time))
					.saturating_mul(remaining_blocks)
					/ (total_blocks.saturating_sub(remaining_blocks) + 1);
				info!(
					"[{:?}] syncing parentchain to {}. already synced until block {}. immediate import={}. est. remaining: {}",
					id, curr_block_number, until_synced_header.number, immediate_import, format_duration(remaining_time_estimate)
				);
			}
			debug!(
				"[{:?}] Found {} block(s) to sync in this chunk. immediate import={} ",
				id,
				block_chunk_to_sync.len(),
				immediate_import
			);
			if block_chunk_to_sync.is_empty() {
				return Ok(until_synced_header)
			}

			let skip_invocations = if let Some(creation_block) = maybe_creation_block {
				let max_blocknumber_in_chunk =
					block_chunk_to_sync.last().map_or_else(|| 0, |b| b.block.header.number());
				if max_blocknumber_in_chunk < creation_block.number {
					trace!("skipping invocations for fast-sync for blocks older than shard creation: {} < {}", max_blocknumber_in_chunk, creation_block.number);
					true
				} else {
					false
				}
			} else {
				false
			};

			let events_chunk_to_sync: Vec<Vec<u8>> = if skip_invocations {
				vec![]
			} else {
				let evs = block_chunk_to_sync
					.par_iter()
					.map(|block| {
						self.parentchain_api.get_events_for_block(Some(block.block.header.hash()))
					})
					.collect::<Result<Vec<_>, _>>()?;
				debug!("[{:?}] Found {} event vector(s) to sync in this chunk", id, evs.len());
				evs
			};

			let events_proofs_chunk_to_sync: Vec<StorageProof> = if skip_invocations {
				vec![]
			} else {
				block_chunk_to_sync
					.par_iter()
					.map(|block| {
						self.parentchain_api.get_events_value_proof(Some(block.block.header.hash()))
					})
					.collect::<Result<Vec<_>, _>>()?
			};

			let sync_start_time = duration_now();

			self.enclave_api.sync_parentchain(
				block_chunk_to_sync.as_slice(),
				events_chunk_to_sync.as_slice(),
				events_proofs_chunk_to_sync.as_slice(),
				self.parentchain_id(),
				immediate_import,
			)?;

			info!(
				"[{:?}] Synced parentchain batch in {}",
				id,
				format_duration(duration_now().saturating_sub(sync_start_time))
			);

			let api_client_until_synced_header = block_chunk_to_sync
				.last()
				.map(|b| b.block.header.clone())
				.ok_or(Error::EmptyChunk)?;

			// #TODO: #1451: fix api/client types
			until_synced_header =
				Header::decode(&mut api_client_until_synced_header.encode().as_slice())
					.expect("Can decode previously encoded header; qed");

			start_block = until_synced_header.number + 1;
			info!(
				"[{:?}] Synced {} out of {} finalized parentchain blocks",
				id, until_synced_header.number, curr_block_number,
			);
		}
	}

	fn await_sync_and_import_parentchain_until_at_least(
		&self,
		last_synced_header: &Header,
		until_header: &Header,
		overriden_start_block: u32,
		shard: ShardIdentifier,
	) -> ServiceResult<Header> {
		let id = self.parentchain_id();

		trace!(
			"[{:?}] last synced block number: {}. synching until {}",
			id,
			last_synced_header.number,
			until_header.number
		);
		let mut last_synced_header = last_synced_header.clone();

		while last_synced_header.number() < until_header.number() {
			last_synced_header = self.sync_parentchain_until_latest_finalized(
				last_synced_header,
				overriden_start_block,
				shard,
				true,
			)?;
			info!("[{:?}] synced block number: #{}", id, last_synced_header.number);
			std::thread::sleep(std::time::Duration::from_secs(1));
		}
		Ok(last_synced_header)
	}
}
