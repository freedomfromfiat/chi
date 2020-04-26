// Copyright 2018-2020 Parity Technologies (UK) Ltd.
// This file is part of Substrate.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate.  If not, see <http://www.gnu.org/licenses/>.

//! Substrate chain configurations.

use sc_chain_spec::ChainSpecExtension;
use sp_core::{Pair, Public, crypto::UncheckedInto, sr25519};
use serde::{Serialize, Deserialize};
use node_runtime::{
	AuthorityDiscoveryConfig, BabeConfig, BalancesConfig, ContractsConfig, CouncilConfig,
	DemocracyConfig,GrandpaConfig, ImOnlineConfig, SessionConfig, SessionKeys, StakerStatus,
	StakingConfig, ElectionsConfig, IndicesConfig, SocietyConfig, SudoConfig, SystemConfig,
	TechnicalCommitteeConfig, WASM_BINARY,
};
use node_runtime::Block;
use node_runtime::constants::currency::*;
use sc_service::ChainType;
use hex_literal::hex;
use sc_telemetry::TelemetryEndpoints;
use grandpa_primitives::{AuthorityId as GrandpaId};
use sp_consensus_babe::{AuthorityId as BabeId};
use pallet_im_online::sr25519::{AuthorityId as ImOnlineId};
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_runtime::{Perbill, traits::{Verify, IdentifyAccount}};

pub use node_primitives::{AccountId, Balance, Signature};
pub use node_runtime::GenesisConfig;

type AccountPublic = <Signature as Verify>::Signer;

const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Node `ChainSpec` extensions.
///
/// Additional parameters for some Substrate core modules,
/// customizable from the chain spec.
#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
	/// Block numbers with known hashes.
	pub fork_blocks: sc_client::ForkBlocks<Block>,
	/// Known bad block hashes.
	pub bad_blocks: sc_client::BadBlocks<Block>,
}

/// Specialized `ChainSpec`.
pub type ChainSpec = sc_service::GenericChainSpec<
	GenesisConfig,
	Extensions,
>;
/// Chi testnet generator
pub fn chi_config() -> Result<ChainSpec, String> {
	ChainSpec::from_json_bytes(&include_bytes!("../res/chi.json")[..])
}

fn session_keys(
	grandpa: GrandpaId,
	babe: BabeId,
	im_online: ImOnlineId,
	authority_discovery: AuthorityDiscoveryId,
) -> SessionKeys {
	SessionKeys { grandpa, babe, im_online, authority_discovery }
}

fn staging_testnet_config_genesis() -> GenesisConfig {
	// stash, controller, session-key
	// generated with secret:
	// for i in 1 2 3 4 ; do for j in stash controller; do subkey inspect "$secret"/fir/$j/$i; done; done
	// and
	// for i in 1 2 3 4 ; do for j in session; do subkey --ed25519 inspect "$secret"//fir//$j//$i; done; done
	let initial_authorities: Vec<(AccountId, AccountId, GrandpaId, BabeId, ImOnlineId, AuthorityDiscoveryId)> = vec![(
		// 5EyUe5ZBCfgwrK9nWXqKyNF8eGEhu87heWJxPV3Kji8rYSLX
		hex!["80b7fb128680a50de836ea72ac60b1ce8083758ca50ac571ae7035c4009d386e"].into(),
		// 5FHAkVXwJPweZEqBbpJ1WKWbm45xCnaCJRR9GKm61t8FvuNK
		hex!["8e362d46cc4cce4999ebc020fad2fd99eee731786e5fafc88b896e9e5b5d0812"].into(),
		// 5Ho8jSdgdKzbiYFt6s4FUvzjEns6rYAjv79yJqB5NWKZVDiU
		hex!["fd89365cf0975ad489c9cd4e59146ee4af7755257a7a4db5047a8e4996f6fd99"].unchecked_into(),
		// 5GbKnc4M9bUGKkicgRd6eK6BtV81PRptb25eBpK5QrNbXx5V
		hex!["c84b3f34f511081c96afdf8cbd7b245da8502e532e615d58ab5544cf90c4f014"].unchecked_into(),
		// 5D2PisUYABGbHa7BZWisJaZwqkBKbimudU47DV2dFaJBLudw
		hex!["2a78b258d939c0c74efcce6f3b75d6d31064b6edf2cbbddc980f2f9cea0a9a71"].unchecked_into(),
		// 5D7Tw7VUVcnnJ9jjY418VmSnrXRhzqyGXEyJnCmR1KsAoBKd
		hex!["2e571971c01a31919f6176b522ebf30caa8ab0a4c42d75fc97302b92062a710b"].unchecked_into(),
	),(
		// 5H5u8AVf2FoTY6E8T8N5d5TDZNEsw9v4YyFRRx8ri3ZiDkBG
		hex!["de165761f47010d6ed0e0f1398a9a7dc9a6a6adf66a39284a33b282e0fa9fa34"].into(),
		// 5FU3TmLofuXRfPgNGzj8J1KyDiYcsCTXiyjbE4mDv14azgE4
		hex!["96815762f5197337f08eb7f6ae2396ad216b5457964aa3db9db7939a1d0fda0a"].into(),
		// 5CgUbTx3PaLgMWfqxSLeNNf25AobFUu2a4V8ChrS6MocFQGF
		hex!["1b4830b500cde8dcb7ac759ec4c0835eac332cb0eb9c1839757b4c9079cce79f"].unchecked_into(),
		// 5H4BDeTuMnfohP3XsByHTmxdPhnYwMGbr64WBBXJgMssticj
		hex!["dcc6078e7f533631334e2fb6e1b5ff8f9660aed74fc5899fcd13cc4d65daad3b"].unchecked_into(),
		// 5F4zUtV6STrNMAkJymeZBToi6otFN8tDer1Hi1fk3BU4yqR9
		hex!["84eca9d0b10d77dc9250564663f699553c4a86dfe8f4b0b1b710782291854e1f"].unchecked_into(),
		// 5GvZcpz9pZunbbB9p43yZ7KsJXkGuRimPa47V1FGx2KMuCi8
		hex!["d6f77867438a4699a33d4ba4bdc8120b59055c49b3b1009fb698b587e118364a"].unchecked_into(),
	),(
		// 5CD4EuDsdqNw5u2xptTBjQkNyrFrNF3L7N8YS5ETLqd7AaDV
		hex!["065e9460a69ae326804aaace448e2103df3d8aff9e505b4089c64670c33f9443"].into(),
		// 5CATvrruub73PzgUa9KEnCAEg4PVQM9bGTdpZMtACwm8jzoH
		hex!["046496672a2d2fb84c7ac4b8f5b3bd7a507b7c6a682fab08e33fa15495e3837f"].into(),
		// 5DQddFYX4qjUTj6YAFLTsTRkudsf36admknsWSD2rx4DwXLx
		hex!["3b6ee5c9d60368a61363c67aa0d7cb728454ff647a323b78e888428a98b06d04"].unchecked_into(),
		// 5CUVJsACd5mDodwWn5vXwR7W1rM8YPg7LML53mnAmd1niNMJ
		hex!["1223a5f46e618813a001a55aa01057ec678de11f3e70f3fc4d3b39ab58b4a863"].unchecked_into(),
		// 5CvnjCz8EwRD2er1VZkL4KXF9yZ5hutcTEe3GrxxMPCiGpWm
		hex!["2632ab2dc6e963ba96f5e2ffac784060b575e0789115ad5f0fa417d7c90df060"].unchecked_into(),
		// 5Ey8uf2MRV6BbUGxrsFyvUNLUmPAfMxKahFkJZYjbFRrmP9d
		hex!["80758f0fe3c21f2e855369675f60a3885875f29b8ed3a0f97d6ad21ddf52415d"].unchecked_into(),
	),(
		// 5Exz65MXqK6PxhKf4sHGEMZx432guGPbYV9Z4Zn8u5fkGceC
		hex!["8057ddebcf234ed4dbbadfcea683814192c282ea53d7a1c49564e2847fc4bb24"].into(),
		// 5He9HxdG6jh4RJJiuAqSUVqLU1Q4M5n3PVmQWFGWd9g1Sbwc
		hex!["f6ade4187c3dfb8be7f1ec07be5ddbec799e3b59d186732a8bc9d4285f19f258"].into(),
		// 5EMQdQxDpH7nY54TKnfSMjLLPAU4HtUxvERVWpVcscYfvNdH
		hex!["6535a47208fc1abb03fb41010d6f6cd74bf277857693b146b121bfb50a6f109c"].unchecked_into(),
		// 5CXMQnHdgLDeMLF2rjfMGrTXQZwT6JEzdFZPo8GRc2pJkvb3
		hex!["1452cc24c76c760263df1f957749492a7d292588aa7eee805ff843fe16159443"].unchecked_into(),
		// 5EsvrJd2cS8DUsZf9xRj8Bd3HT3VvM4GKg6rLTmYFexQkTwT
		hex!["7c7cbddea348966da57025570cd138f3f59b6caeda72efd7554d7452f920f027"].unchecked_into(),
		// 5DP1ytZWLfm8qevhYsQsxUgipd9jg4z6rJnA9hvffR2dLKES
		hex!["3a33a9ca0efa6198c3d7c17ca41492ba90e6d979b84034c05263f230bdbe7e1e"].unchecked_into(),
	)];

	// generated with secret: subkey inspect "$secret"/fir
	let root_key: AccountId = hex![
		// 5D4cRQFYe9qnoJvna7fZpbuM2WSfyRYNHaHfSmtR4iVephEF
		"2c29ef7a61d5d642736bc14a6da67c2a322c99b8ad5d4679bf3da08d3fdf1841"
	].into();

	let endowed_accounts: Vec<AccountId> = vec![root_key.clone()];

	testnet_genesis(
		initial_authorities,
		root_key,
		Some(endowed_accounts),
		false,
	)
}

/// Staging testnet config.
pub fn staging_testnet_config() -> ChainSpec {
	let boot_nodes = vec![];
	ChainSpec::from_genesis(
		"Staging Testnet",
		"staging_testnet",
		ChainType::Live,
		staging_testnet_config_genesis,
		boot_nodes,
		Some(TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 0)])
			.expect("Staging telemetry url is valid; qed")),
		None,
		None,
		Default::default(),
	)
}

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate stash, controller and session key from seed
pub fn authority_keys_from_seed(seed: &str) -> (
	AccountId,
	AccountId,
	GrandpaId,
	BabeId,
	ImOnlineId,
	AuthorityDiscoveryId,
) {
	(
		get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
		get_account_id_from_seed::<sr25519::Public>(seed),
		get_from_seed::<GrandpaId>(seed),
		get_from_seed::<BabeId>(seed),
		get_from_seed::<ImOnlineId>(seed),
		get_from_seed::<AuthorityDiscoveryId>(seed),
	)
}

/// Helper function to create GenesisConfig for testing
pub fn testnet_genesis(
	initial_authorities: Vec<(
		AccountId,
		AccountId,
		GrandpaId,
		BabeId,
		ImOnlineId,
		AuthorityDiscoveryId,
	)>,
	root_key: AccountId,
	endowed_accounts: Option<Vec<AccountId>>,
	enable_println: bool,
) -> GenesisConfig {
	let endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(|| {
		vec![
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			get_account_id_from_seed::<sr25519::Public>("Bob"),
			get_account_id_from_seed::<sr25519::Public>("Charlie"),
			get_account_id_from_seed::<sr25519::Public>("Dave"),
			get_account_id_from_seed::<sr25519::Public>("Eve"),
			get_account_id_from_seed::<sr25519::Public>("Ferdie"),
			get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
			get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
			get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
			get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
			get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
			get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
		]
	});
	let num_endowed_accounts = endowed_accounts.len();

	const ENDOWMENT: Balance = 7_777_777 * CHI;
	const STASH: Balance = 100 * CHI;

	GenesisConfig {
		frame_system: Some(SystemConfig {
			code: WASM_BINARY.to_vec(),
			changes_trie_config: Default::default(),
		}),
		pallet_balances: Some(BalancesConfig {
			balances: endowed_accounts.iter().cloned()
				.map(|k| (k, ENDOWMENT))
				.chain(initial_authorities.iter().map(|x| (x.0.clone(), STASH)))
				.collect(),
		}),
		pallet_indices: Some(IndicesConfig {
			indices: vec![],
		}),
		pallet_session: Some(SessionConfig {
			keys: initial_authorities.iter().map(|x| {
				(x.0.clone(), x.0.clone(), session_keys(
					x.2.clone(),
					x.3.clone(),
					x.4.clone(),
					x.5.clone(),
				))
			}).collect::<Vec<_>>(),
		}),
		pallet_staking: Some(StakingConfig {
			validator_count: initial_authorities.len() as u32 * 2,
			minimum_validator_count: initial_authorities.len() as u32,
			stakers: initial_authorities.iter().map(|x| {
				(x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator)
			}).collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			slash_reward_fraction: Perbill::from_percent(10),
			.. Default::default()
		}),
		pallet_democracy: Some(DemocracyConfig::default()),
		pallet_elections_phragmen: Some(ElectionsConfig {
			members: endowed_accounts.iter()
						.take((num_endowed_accounts + 1) / 2)
						.cloned()
						.map(|member| (member, STASH))
						.collect(),
		}),
		pallet_collective_Instance1: Some(CouncilConfig::default()),
		pallet_collective_Instance2: Some(TechnicalCommitteeConfig {
			members: endowed_accounts.iter()
						.take((num_endowed_accounts + 1) / 2)
						.cloned()
						.collect(),
			phantom: Default::default(),
		}),
		pallet_contracts: Some(ContractsConfig {
			current_schedule: pallet_contracts::Schedule {
				enable_println, // this should only be enabled on development chains
				..Default::default()
			},
			gas_price: 1 * MICROCHI,
		}),
		pallet_sudo: Some(SudoConfig {
			key: root_key,
		}),
		pallet_babe: Some(BabeConfig {
			authorities: vec![],
		}),
		pallet_im_online: Some(ImOnlineConfig {
			keys: vec![],
		}),
		pallet_authority_discovery: Some(AuthorityDiscoveryConfig {
			keys: vec![],
		}),
		pallet_grandpa: Some(GrandpaConfig {
			authorities: vec![],
		}),
		pallet_membership_Instance1: Some(Default::default()),
		pallet_treasury: Some(Default::default()),
		pallet_society: Some(SocietyConfig {
			members: endowed_accounts.iter()
						.take((num_endowed_accounts + 1) / 2)
						.cloned()
						.collect(),
			pot: 0,
			max_members: 999,
		}),
		pallet_vesting: Some(Default::default()),
	}
}

fn development_config_genesis() -> GenesisConfig {
	testnet_genesis(
		vec![
			authority_keys_from_seed("Alice"),
		],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
		true,
	)
}

/// Development config (single validator Alice)
pub fn development_config() -> ChainSpec {
	ChainSpec::from_genesis(
		"Development",
		"dev",
		ChainType::Development,
		development_config_genesis,
		vec![],
		None,
		None,
		None,
		Default::default(),
	)
}

fn local_testnet_genesis() -> GenesisConfig {
	testnet_genesis(
		vec![
			authority_keys_from_seed("Alice"),
			authority_keys_from_seed("Bob"),
		],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
		false,
	)
}

/// Local testnet config (multivalidator Alice + Bob)
pub fn local_testnet_config() -> ChainSpec {
	ChainSpec::from_genesis(
		"Local Testnet",
		"local_testnet",
		ChainType::Local,
		local_testnet_genesis,
		vec![],
		None,
		None,
		None,
		Default::default(),
	)
}

#[cfg(test)]
pub(crate) mod tests {
	use super::*;
	use crate::service::{new_full, new_light};
	use sc_service_test;
	use sp_runtime::BuildStorage;

	fn local_testnet_genesis_instant_single() -> GenesisConfig {
		testnet_genesis(
			vec![
				authority_keys_from_seed("Alice"),
			],
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			None,
			false,
		)
	}

	/// Local testnet config (single validator - Alice)
	pub fn integration_test_config_with_single_authority() -> ChainSpec {
		ChainSpec::from_genesis(
			"Integration Test",
			"test",
			ChainType::Development,
			local_testnet_genesis_instant_single,
			vec![],
			None,
			None,
			None,
			Default::default(),
		)
	}

	/// Local testnet config (multivalidator Alice + Bob)
	pub fn integration_test_config_with_two_authorities() -> ChainSpec {
		ChainSpec::from_genesis(
			"Integration Test",
			"test",
			ChainType::Development,
			local_testnet_genesis,
			vec![],
			None,
			None,
			None,
			Default::default(),
		)
	}

	#[test]
	#[ignore]
	fn test_connectivity() {
		sc_service_test::connectivity(
			integration_test_config_with_two_authorities(),
			|config| new_full(config),
			|config| new_light(config),
		);
	}

	#[test]
	fn test_create_development_chain_spec() {
		development_config().build_storage().unwrap();
	}

	#[test]
	fn test_create_local_testnet_chain_spec() {
		local_testnet_config().build_storage().unwrap();
	}

	#[test]
	fn test_staging_test_net_chain_spec() {
		staging_testnet_config().build_storage().unwrap();
	}
}
