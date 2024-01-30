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
use itc_parentchain::{
	light_client::light_client_init_params::{GrandpaParams, SimpleParams},
	primitives::{ParentchainId, ParentchainInitParams},
};
use itp_api_client_types::ParentchainApi;
use itp_enclave_api::{enclave_base::EnclaveBase, sidechain::Sidechain};
use itp_node_api::api_client::ChainApi;
use itp_storage::StorageProof;
use litentry_primitives::ParentchainHeader as Header;
use log::*;
use sp_consensus_grandpa::VersionedAuthorityList;
use sp_runtime::traits::Header as HeaderTrait;
use std::{cmp::min, sync::Arc};
use substrate_api_client::ac_primitives::{Block, Header as HeaderT};

const BLOCK_SYNC_BATCH_SIZE: u32 = 1000;

pub trait HandleParentchain {
	/// Initializes all parentchain specific components on the enclave side.
	/// Returns the latest synced block header.
	fn init_parentchain_components(&self) -> ServiceResult<Header>;

	/// Fetches the parentchain blocks to sync from the parentchain and feeds them to the enclave.
	/// Returns the latest synced block header.
	///
	/// Litentry: `overriden_start_block` to forcibly start from the given parentchain block number
	fn sync_parentchain(
		&self,
		last_synced_header: Header,
		overriden_start_block: u32,
		is_syncing: bool,
	) -> ServiceResult<Header>;

	/// Syncs and directly imports parentchain blocks from the latest synced header
	/// until the specified until_header.
	///
	/// Litentry: `overriden_start_block` to forcibly start from the given parentchain block number
	fn sync_and_import_parentchain_until(
		&self,
		last_synced_header: &Header,
		until_header: &Header,
		overriden_start_block: u32,
	) -> ServiceResult<Header>;
}

/// Handles the interaction between parentchain and enclave.
pub(crate) struct ParentchainHandler<ParentchainApi, EnclaveApi> {
	parentchain_api: ParentchainApi,
	enclave_api: Arc<EnclaveApi>,
	parentchain_init_params: ParentchainInitParams,
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

	fn sync_parentchain(
		&self,
		last_synced_header: Header,
		overriden_start_block: u32,
		is_syncing: bool,
	) -> ServiceResult<Header> {
		let id = self.parentchain_id();
		trace!("[{:?}] Getting current head", id);
		let curr_block = self
			.parentchain_api
			.last_finalized_block()?
			.ok_or(Error::MissingLastFinalizedBlock)?;
		let curr_block_number = curr_block.block.header().number();

		info!(
			"[{:?}] Syncing blocks from {} to {}",
			id, last_synced_header.number, curr_block_number
		);

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
			let block_chunk_to_sync = self.parentchain_api.get_blocks(
				start_block,
				min(start_block + BLOCK_SYNC_BATCH_SIZE, curr_block_number),
			)?;
			info!("[{:?}] Found {} block(s) to sync", id, block_chunk_to_sync.len());
			if block_chunk_to_sync.is_empty() {
				return Ok(until_synced_header)
			}

			let events_chunk_to_sync: Vec<Vec<u8>> = block_chunk_to_sync
				.iter()
				.map(|block| {
					self.parentchain_api.get_events_for_block(Some(block.block.header.hash()))
				})
				.collect::<Result<Vec<_>, _>>()?;

			info!("[{:?}] Found {} event vector(s) to sync", id, events_chunk_to_sync.len());

			let events_proofs_chunk_to_sync: Vec<StorageProof> = block_chunk_to_sync
				.iter()
				.map(|block| {
					self.parentchain_api.get_events_value_proof(Some(block.block.header.hash()))
				})
				.collect::<Result<Vec<_>, _>>()?;

			self.enclave_api.sync_parentchain(
				block_chunk_to_sync.as_slice(),
				events_chunk_to_sync.as_slice(),
				events_proofs_chunk_to_sync.as_slice(),
				self.parentchain_id(),
				is_syncing,
			)?;

			let api_client_until_synced_header = block_chunk_to_sync
				.last()
				.map(|b| b.block.header.clone())
				.ok_or(Error::EmptyChunk)?;
			info!(
				"[{:?}] Synced {} out of {} finalized parentchain blocks",
				id, until_synced_header.number, curr_block_number,
			);

			// #TODO: #1451: fix api/client types
			until_synced_header =
				Header::decode(&mut api_client_until_synced_header.encode().as_slice())
					.expect("Can decode previously encoded header; qed");

			start_block = until_synced_header.number + 1;
			println!(
				"[{:?}] Synced {} out of {} finalized parentchain blocks",
				id, until_synced_header.number, curr_block_number,
			);
		}
	}

	fn sync_and_import_parentchain_until(
		&self,
		last_synced_header: &Header,
		until_header: &Header,
		overriden_start_block: u32,
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
			last_synced_header =
				self.sync_parentchain(last_synced_header, overriden_start_block, true)?;
			println!("[{:?}] synced block number: #{}", id, last_synced_header.number);
			std::thread::sleep(std::time::Duration::from_secs(1));
		}
		Ok(last_synced_header)
	}
}