// Copyright 2020-2023 Trust Computing GmbH.
// This file is part of Litentry.
//
// Litentry is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Litentry is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Litentry.  If not, see <https://www.gnu.org/licenses/>.

use crate::utils::DecodeRaw;
use itp_component_container::ComponentGetter;
use itp_sgx_crypto::key_repository::AccessKey;
use lc_data_providers::{DataProviderConfig, GLOBAL_DATA_PROVIDER_CONFIG};
use lc_stf_task_receiver::StfTaskContext;
use lc_vc_task_receiver::run_vc_handler_runner;
use log::*;
use sgx_types::sgx_status_t;
use std::sync::Arc;

use crate::{
	error::{Error, Result},
	initialization::global_components::{
		EnclaveStfEnclaveSigner, GLOBAL_OCALL_API_COMPONENT,
		GLOBAL_SHIELDING_KEY_REPOSITORY_COMPONENT, GLOBAL_STATE_OBSERVER_COMPONENT,
		GLOBAL_TOP_POOL_AUTHOR_COMPONENT,
	},
	utils::{
		get_extrinsic_factory_from_litentry_solo_or_parachain,
		get_node_metadata_repository_from_litentry_solo_or_parachain,
	},
	GLOBAL_STATE_HANDLER_COMPONENT,
};

#[no_mangle]
pub unsafe extern "C" fn run_vc_issuance(dpc: *const u8, dpc_size: usize) -> sgx_status_t {
	let data_provider_config = match DataProviderConfig::decode_raw(dpc, dpc_size) {
		Ok(data_provider_config) => data_provider_config,
		Err(e) => return Error::Codec(e).into(),
	};

	match GLOBAL_DATA_PROVIDER_CONFIG.write() {
		Ok(mut dpc) => {
			dpc.set_twitter_official_url(data_provider_config.twitter_official_url);
			dpc.set_twitter_litentry_url(data_provider_config.twitter_litentry_url);
			dpc.set_twitter_auth_token_v2(data_provider_config.twitter_auth_token_v2);
			dpc.set_discord_official_url(data_provider_config.discord_official_url);
			dpc.set_discord_litentry_url(data_provider_config.discord_litentry_url);
			dpc.set_discord_auth_token(data_provider_config.discord_auth_token);
			dpc.set_achainable_url(data_provider_config.achainable_url);
			dpc.set_achainable_auth_key(data_provider_config.achainable_auth_key);
			dpc.set_credential_endpoint(data_provider_config.credential_endpoint);
			dpc.set_oneblock_notion_key(data_provider_config.oneblock_notion_key);
			dpc.set_oneblock_notion_url(data_provider_config.oneblock_notion_url);
			dpc.set_sora_quiz_master_id(data_provider_config.sora_quiz_master_id);
			dpc.set_sora_quiz_attendee_id(data_provider_config.sora_quiz_attendee_id);
			dpc.set_nodereal_api_key(data_provider_config.nodereal_api_key);
			dpc.set_nodereal_api_url(data_provider_config.nodereal_api_url);
			dpc.set_contest_legend_discord_role_id(
				data_provider_config.contest_legend_discord_role_id,
			);
			dpc.set_contest_popularity_discord_role_id(
				data_provider_config.contest_popularity_discord_role_id,
			);
			dpc.set_contest_participant_discord_role_id(
				data_provider_config.contest_participant_discord_role_id,
			);
			dpc.set_vip3_url(data_provider_config.vip3_url);
		},
		Err(e) => {
			error!("Error while setting data provider config: {:?}", e);
			return Error::MutexAccess.into()
		},
	}

	println!("[+] Starting to Run VC Issuance Internal");
	if let Err(e) = run_vc_issuance_internal() {
		error!("Error while running stf task handler thread: {:?}", e);
		return e.into()
	}

	sgx_status_t::SGX_SUCCESS
}

/// Internal [`run_stf_task_handler`] function to be able to use the `?` operator.
///
/// Runs an extrinsic request inside the enclave, opening a channel and waiting for
/// senders to send requests.
fn run_vc_issuance_internal() -> Result<()> {
	let author_api = GLOBAL_TOP_POOL_AUTHOR_COMPONENT.get()?;
	let state_handler = GLOBAL_STATE_HANDLER_COMPONENT.get()?;
	let state_observer = GLOBAL_STATE_OBSERVER_COMPONENT.get()?;

	let shielding_key_repository = GLOBAL_SHIELDING_KEY_REPOSITORY_COMPONENT.get()?;
	#[allow(clippy::unwrap_used)]
	let shielding_key = shielding_key_repository.retrieve_key().unwrap();

	let ocall_api = GLOBAL_OCALL_API_COMPONENT.get()?;
	let stf_enclave_signer = Arc::new(EnclaveStfEnclaveSigner::new(
		state_observer,
		ocall_api.clone(),
		shielding_key_repository,
		author_api.clone(),
	));

	let stf_task_context = StfTaskContext::new(
		shielding_key,
		author_api,
		stf_enclave_signer,
		state_handler,
		ocall_api,
	);
	let extrinsic_factory = get_extrinsic_factory_from_litentry_solo_or_parachain()?;
	let node_metadata_repo = get_node_metadata_repository_from_litentry_solo_or_parachain()?;
	run_vc_handler_runner(Arc::new(stf_task_context), extrinsic_factory, node_metadata_repo);
	Ok(())
}