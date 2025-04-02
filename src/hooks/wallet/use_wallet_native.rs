use crate::gateway::GatewayError as Error;
use dioxus::prelude::*;
use directories::ProjectDirs;
use keyring::Entry;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use std::fs;
use std::path::PathBuf;

use super::Wallet;

const SERVICE: &str = "ORE";
const USER_DEVICE_KEY: &str = "user-device-key";
const SERVICE_TWO: &str = "ORE-two";
const USER_DEVICE_KEY_TWO: &str = "user-device-key-two";
const SERVICE_THREE: &str = "ORE-three";
const USER_DEVICE_KEY_THREE: &str = "user-device-key-three";
const MAX_WALLETS_ALLOWED: u8 = 3;

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    current_wallet_index: u8,
    num_wallets_used: u8,
}

pub fn use_wallet_provider() {
    // Load config first
    let config = match load_config() {
        // We found existing config
        Ok(cfg) => {
            log::info!(
                "Loaded config: current_wallet={}, num_wallets_used={}",
                cfg.current_wallet_index,
                cfg.num_wallets_used
            );
            cfg
        }
        // No config found, so we have default config
        Err(err) => {
            log::error!("Failed to load config: {:?}", err);
            // Fall back to default config
            Config {
                current_wallet_index: 0,
                num_wallets_used: 1,
            }
        }
    };

    // Create a global signal for the current wallet index
    use_context_provider(|| Signal::new(config.clone()));

    let multisig_authority = get_or_set();
    let mut wallet_signal = use_context_provider(|| Signal::new(Wallet::Disconnected));

    use_effect(move || match &multisig_authority {
        Ok(multisig_authority) => {
            wallet_signal.set(Wallet::Connected(multisig_authority.creator.pubkey()));
        }
        Err(err) => {
            log::error!("{:?}", err);
        }
    });
}

pub fn use_config() -> Signal<Config> {
    use_context::<Signal<Config>>()
}

/// embeded keypair on device.
/// field names from sqauds multisig api.
#[derive(Debug, Deserialize, Serialize)]
pub struct MultisigAuthority {
    /// signer embeded on this device
    #[serde(serialize_with = "serialize", deserialize_with = "deserialize")]
    pub creator: Keypair,
    /// ephemeral keypair used to seed multisig pda
    /// persisted to derive pda
    #[serde(serialize_with = "serialize", deserialize_with = "deserialize")]
    pub create_key: Keypair,
}

pub fn get() -> Result<MultisigAuthority, Error> {
    let current_index = use_config().read().current_wallet_index;
    let (service, user_device_key) = get_keyring_values_by_index(current_index);
    let keyring = Entry::new(service, user_device_key)?;
    let secret = keyring.get_secret()?;
    let multisig_authority = bincode::deserialize(secret.as_slice()).map_err(|err| {
        println!("{:?}", err);
        Error::BincodeDeserialize
    })?;
    Ok(multisig_authority)
}

fn set(secret: &[u8]) -> Result<(), Error> {
    let current_index = use_config().read().current_wallet_index;
    let (service, user_device_key) = get_keyring_values_by_index(current_index);
    let keyring = Entry::new(service, user_device_key)?;
    keyring.set_secret(secret).map_err(From::from)
}

pub fn get_or_set() -> Result<MultisigAuthority, Error> {
    match get() {
        // return wallet
        ok @ Ok(_) => ok,
        Err(err) => {
            // no wallet found
            if let Error::KeyringNoEntry = err {
                let creator = Keypair::new();
                let create_key = Keypair::new();
                let multisig_authority = MultisigAuthority {
                    creator,
                    create_key,
                };
                let bytes = bincode::serialize(&multisig_authority).map_err(|err| {
                    println!("{:?}", err);
                    Error::BincodeSerialize
                })?;
                set(bytes.as_slice())?;
                Ok(multisig_authority)
            } else {
                // other error that doesn't indicate that there definitely is *not* already a
                // wallet stored on the device keychain. just return error and invoke retry later
                // to avoid overwrite.
                Err(err)
            }
        }
    }
}

fn serialize<S: Serializer>(keypair: &Keypair, serializer: S) -> Result<S::Ok, S::Error> {
    let bytes = keypair.to_bytes();
    serializer.serialize_bytes(&bytes)
}

fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Keypair, D::Error> {
    let bytes: &[u8] = <&[u8]>::deserialize(deserializer)?;
    Keypair::from_bytes(bytes).map_err(|_| serde::de::Error::custom("invalid keypair bytes"))
}

pub fn get_keyring_values_by_index(index: u8) -> (&'static str, &'static str) {
    match index {
        0 => ("ORE", "user-device-key"),
        1 => ("ORE-two", "user-device-key-two"),
        2 => ("ORE-three", "user-device-key-three"),
        _ => ("ORE", "user-device-key"),
    }
}

fn get_config_path() -> Option<PathBuf> {
    ProjectDirs::from("", "", "Ore").map(|dirs| {
        let config_dir = dirs.config_dir();
        fs::create_dir_all(config_dir).ok();
        config_dir.join("config.json")
    })
}

fn save_config(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(path) = get_config_path() {
        // Create parent directory if it doesn't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let json = serde_json::to_string_pretty(config)?;
        fs::write(&path, &json)?;
        log::info!("Successfully wrote config to: {:?}", path);
    }
    Ok(())
}

fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    if let Some(path) = get_config_path() {
        if path.exists() {
            let data = fs::read_to_string(&path)?;
            let config: Config = serde_json::from_str(&data)?;
            return Ok(config);
        } else {
            // File doesn't exist, create it with default config
            let default_config = Config {
                current_wallet_index: 0,
                num_wallets_used: 1,
            };
            save_config(&default_config)?;
            return Ok(default_config);
        }
    }

    // Return default config if path couldn't be determined
    Ok(Config {
        current_wallet_index: 0,
        num_wallets_used: 1,
    })
}

pub fn update_wallet() -> Result<(), Error> {
    // get the current number of wallets used
    let num_wallets_used = use_config().read().num_wallets_used;
    if num_wallets_used < MAX_WALLETS_ALLOWED {
        // update the number of wallets used
        // we need to write the keys to the store
        // set()
        let current_index = use_config().read().current_wallet_index;
        let (service, user_device_key) = get_keyring_values_by_index(current_index);
        let keyring = Entry::new(service, user_device_key)?;
        let secret = keyring.get_secret()?;
        let multisig_authority = bincode::deserialize(secret.as_slice()).map_err(|err| {
            println!("{:?}", err);
            Error::BincodeDeserialize
        })?;
        let mut config = use_config().write();
        config.num_wallets_used += 1;
        config.current_wallet_index += 1;
        save_config(&config)?;
        Ok(())
    } else {
        log::info!("Max number of wallets reached");
        Ok(())
    }
}

/*
    store vec of pubkeys in the config

*/

// // Helper function to use the current wallet index from any component
// pub fn use_current_wallet() -> Signal<u8> {
//     use_context().unwrap()
// }

// // Helper function to switch the current wallet
// pub fn switch_wallet(index: u8) -> Result<(), Box<dyn std::error::Error>> {
//     // Get current wallet signal
//     let mut current_wallet = use_current_wallet();

//     // Update the config
//     let mut config = load_config()?;
//     config.current_wallet = index;
//     save_config(&config)?;

//     // Update the signal
//     current_wallet.set(index);

//     Ok(())
// }
