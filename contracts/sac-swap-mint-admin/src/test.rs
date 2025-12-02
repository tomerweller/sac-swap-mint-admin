#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::Address as _,
    token::{StellarAssetClient, TokenClient},
    Address, Env,
};

#[test]
fn test_swap_mint() {
    let env = Env::default();
    env.mock_all_auths();

    // Create admin for the SACs
    let sac_issuer = Address::generate(&env);

    // Create test user
    let tester = Address::generate(&env);

    // Create TOMAllow SAC
    let tom_allow_sac = env.register_stellar_asset_contract_v2(sac_issuer.clone());
    let tom_allow_address = tom_allow_sac.address();
    let tom_allow_admin = StellarAssetClient::new(&env, &tom_allow_address);
    let tom_allow_token = TokenClient::new(&env, &tom_allow_address);

    // Create TOM SAC
    let tom_sac = env.register_stellar_asset_contract_v2(sac_issuer.clone());
    let tom_address = tom_sac.address();
    let tom_admin = StellarAssetClient::new(&env, &tom_address);
    let tom_token = TokenClient::new(&env, &tom_address);

    // Deploy SacSwapMintAdmin contract with TOM as admin_asset and TOMAllow as allow_asset
    let swap_mint_admin_id = env.register(
        SacSwapMintAdmin,
        (tom_address.clone(), tom_allow_address.clone()),
    );
    let swap_mint_client = SacSwapMintAdminClient::new(&env, &swap_mint_admin_id);

    // Set SacSwapMintAdmin as admin of TOM
    tom_admin.set_admin(&swap_mint_admin_id);

    // Issue 1000 TOMAllow to tester
    tom_allow_admin.mint(&tester, &1000);
    assert_eq!(tom_allow_token.balance(&tester), 1000);
    assert_eq!(tom_token.balance(&tester), 0);

    // Perform swap_mint: tester converts 1000 TOMAllow to 1000 TOM
    swap_mint_client.swap_mint(&tester, &1000);

    // Verify tester now has 1000 TOM and 0 TOMAllow
    assert_eq!(tom_allow_token.balance(&tester), 0);
    assert_eq!(tom_token.balance(&tester), 1000);
}
