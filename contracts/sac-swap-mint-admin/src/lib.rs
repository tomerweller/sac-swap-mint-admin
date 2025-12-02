#![no_std]
use soroban_sdk::{contract, contractimpl, token, Address, Env};

const ALLOW_ASSET: &str = "allow_asset";
const ADMIN_ASSET: &str = "admin_asset";

#[contract]
pub struct SacSwapMintAdmin;

#[contractimpl]
impl SacSwapMintAdmin {
    pub fn __constructor(env: Env, admin_asset: Address, allow_asset: Address) {
        env.storage().instance().set(&ALLOW_ASSET, &allow_asset);
        env.storage().instance().set(&ADMIN_ASSET, &admin_asset);
    }

    pub fn swap_mint(env: Env, sender: Address, amount: i128) {
        sender.require_auth();

        let allow_asset: Address = env.storage().instance().get(&ALLOW_ASSET).unwrap();
        let admin_asset: Address = env.storage().instance().get(&ADMIN_ASSET).unwrap();

        let allow_client = token::Client::new(&env, &allow_asset);
        let admin_client = token::StellarAssetClient::new(&env, &admin_asset);

        // Burn allow asset directly from sender
        allow_client.burn(&sender, &amount);

        // Mint equivalent amount of admin asset to sender
        admin_client.mint(&sender, &amount);
    }
}

mod test;
