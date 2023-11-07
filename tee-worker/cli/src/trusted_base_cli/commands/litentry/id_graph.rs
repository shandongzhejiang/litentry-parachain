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

use crate::{
	trusted_cli::TrustedCli, trusted_command_utils::get_pair_from_str,
	trusted_operation::perform_trusted_operation, Cli, CliResult, CliResultOk,
};
use codec::Decode;
use ita_stf::{IDGraph, Runtime, TrustedGetter, TrustedOperation};
use itp_stf_primitives::types::KeyPair;
use litentry_primitives::Identity;

// usage example:
//
// ./bin/litentry-cli trusted -m <mrenclave> -d id-graph did:litentry:substrate:0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48
//
// returns:
//
// [(Twitter(IdentityString { inner: BoundedVec([109, 121, 104, 97, 110, 100, 108, 101], 64) }), IdentityContext { link_block: 193, web3networks: [], status: Active }), (Evm(Address20([13, 155, 253, 31, 24, 245, 244, 253, 8, 36, 125, 197, 74, 211, 82, 137, 9, 196, 179, 233])), IdentityContext { link_block: 84, web3networks: [Ethereum, Bsc], status: Active }), (Substrate(Address32([142, 175, 4, 21, 22, 135, 115, 99, 38, 201, 254, 161, 126, 37, 252, 82, 135, 97, 54, 147, 201, 18, 144, 156, 178, 38, 170, 71, 148, 242, 106, 72])), IdentityContext { link_block: 40, web3networks: [Polkadot, Kusama, Litentry, Litmus, LitentryRococo, Khala, SubstrateTestnet], status: Active })]

#[derive(Parser)]
pub struct IDGraphCommand {
	// did format - will be converted to `Identity`
	// not using e.g. ss58 format as we support both evm and substrate
	did: String,
}

impl IDGraphCommand {
	pub(crate) fn run(&self, cli: &Cli, trusted_cli: &TrustedCli) -> CliResult {
		let alice = get_pair_from_str(trusted_cli, "//Alice", cli);
		let id: Identity = Identity::from_did(self.did.as_str()).unwrap();

		let top: TrustedOperation =
			TrustedGetter::id_graph(id).sign(&KeyPair::Sr25519(Box::new(alice))).into();
		let idgraph = perform_trusted_operation(cli, trusted_cli, &top)
			.map(|v| IDGraph::<Runtime>::decode(&mut v.unwrap().as_slice()).ok());
		println!("{:?}", idgraph.unwrap().unwrap());

		Ok(CliResultOk::None)
	}
}