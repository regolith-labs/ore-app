use crate::gateway::GatewayError as Error;
use dioxus::prelude::*;
use directories::ProjectDirs;
use keyring::Entry;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use std::fs;
use std::path::PathBuf;

use super::Wallet;

use crate::hooks::use_wallet;

const SERVICE: &str = "ORE";
const USER_DEVICE_KEY: &str = "user-device-key";
const SERVICE_TWO: &str = "ORE-two";
const USER_DEVICE_KEY_TWO: &str = "user-device-key-two";
const SERVICE_THREE: &str = "ORE-three";
const USER_DEVICE_KEY_THREE: &str = "user-device-key-three";
pub const MAX_WALLETS_ALLOWED: u8 = 3;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WalletKey {
    pub name: String,
    pub pubkey: String,
    pub index: u8,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WalletState {
    pub current_wallet_index: u8,
    pub num_wallets_used: u8,
    pub wallet_pubkeys: Vec<WalletKey>,
}

pub fn use_wallet_provider() {
    let mut wallet_state = use_context_provider(|| {
        Signal::new(WalletState {
            current_wallet_index: 0,
            num_wallets_used: 1,
            wallet_pubkeys: Vec::with_capacity(MAX_WALLETS_ALLOWED as usize),
        })
    });

    let mut wallet_signal: Signal<Wallet> =
        use_context_provider(|| Signal::new(Wallet::Disconnected));

    // Try to load keypair and config
    let mut keychain_data = get_or_set();

    // Update the signals with the loaded values
    use_effect(move || match &keychain_data {
        Ok((multisig_authority, config)) => {
            wallet_state.set(config.clone());
            wallet_signal.set(Wallet::Connected(multisig_authority.creator.pubkey()));
        }
        Err(err) => {
            log::error!("Error in wallet provider: {:?}", err);
        }
    });
}

pub fn use_wallet_state() -> Signal<WalletState> {
    use_context::<Signal<WalletState>>()
}

pub fn get_keyring_values_by_index(index: u8) -> (&'static str, &'static str) {
    match index {
        0 => ("ORE", "user-device-key"),
        1 => ("ORE-two", "user-device-key-two"),
        2 => ("ORE-three", "user-device-key-three"),
        _ => ("ORE", "user-device-key"),
    }
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

pub fn get() -> Result<(MultisigAuthority, WalletState), Error> {
    let mut wallet_state = use_wallet_state();
    // Let's try to load the config if it exists
    if let Ok(config) = load_config() {
        // Get the current wallet index & load it up in the keypair
        let current_index = config.current_wallet_index;
        let (service, user_device_key) = get_keyring_values_by_index(current_index);
        let keyring = Entry::new(service, user_device_key)?;
        let secret = keyring.get_secret()?;
        let multisig_authority = bincode::deserialize(secret.as_slice()).map_err(|err| {
            println!("{:?}", err);
            Error::BincodeDeserialize
        })?;

        let mut writable_wallet_state = wallet_state.write();

        // Let's get all the pubkeys for the current wallets & add it to global state
        for i in 0..config.num_wallets_used {
            writable_wallet_state.wallet_pubkeys.push(WalletKey {
                name: format!("Wallet {}", i + 1),
                pubkey: config.wallet_pubkeys[i as usize].pubkey.to_string(),
                index: config.wallet_pubkeys[i as usize].index,
            });
        }
        drop(writable_wallet_state);

        Ok((multisig_authority, config))
    } else {
        Err(Error::ConfigNotFound)
    }
}

fn set(secret: &[u8], index: u8) -> Result<(), Error> {
    let (service, user_device_key) = get_keyring_values_by_index(index);
    let keyring = Entry::new(service, user_device_key)?;
    keyring.set_secret(secret).map_err(From::from)
}

pub fn get_or_set() -> Result<(MultisigAuthority, WalletState), Error> {
    match get() {
        // Return wallet data if found (MultisigAuthority, WalletConfig)
        ok @ Ok(_) => ok,
        Err(err) => {
            // We didn't find a wallet on the device keychain or config
            if let Error::KeyringNoEntry | Error::ConfigNotFound = err {
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

                // This will be the first wallet we write to the keychain, so we can hardcode the index & num wallets used
                let mut wallet_state = WalletState {
                    current_wallet_index: 0,
                    num_wallets_used: 1,
                    wallet_pubkeys: Vec::with_capacity(MAX_WALLETS_ALLOWED as usize),
                };
                let wallet_name = format!("Wallet {}", wallet_state.num_wallets_used);
                wallet_state.wallet_pubkeys.push(WalletKey {
                    name: wallet_name,
                    pubkey: multisig_authority.creator.pubkey().to_string(),
                    index: 0,
                });

                // Ensure we're able to save the config
                match save_config(&wallet_state) {
                    Ok(_) => {
                        let current_index = wallet_state.current_wallet_index;
                        // Set the secret in the keyring
                        set(bytes.as_slice(), current_index)?;
                        Ok((multisig_authority, wallet_state))
                    }
                    Err(e) => {
                        log::error!("Failed to save config: {:?}", e);
                        Err(Error::SaveWalletConfig)
                    }
                }
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

fn get_config_path() -> Option<PathBuf> {
    ProjectDirs::from("", "", "Ore").map(|dirs| {
        let config_dir = dirs.config_dir();
        fs::create_dir_all(config_dir).ok();
        config_dir.join("config.json")
    })
}

pub fn save_config(config: &WalletState) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(path) = get_config_path() {
        // Create parent directory if it doesn't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Serialize the config
        let json = serde_json::to_string_pretty(config)?;

        // Write the config to the file
        fs::write(&path, &json)?;
    }
    Ok(())
}

fn load_config() -> Result<WalletState, Error> {
    if let Some(path) = get_config_path() {
        if path.exists() {
            let data = fs::read_to_string(&path)?;
            let config: WalletState = serde_json::from_str(&data)?;
            return Ok(config);
        } else {
            return Err(Error::ConfigNotFound);
        }
    } else {
        return Err(Error::ConfigNotFound);
    }
}

pub fn add_new_keypair(
    private_key_string: Option<String>,
    wallet_name: Option<String>,
) -> Result<(), Error> {
    let mut current_wallet = use_wallet();

    // Get current wallet config
    let mut wallet_state = use_wallet_state();

    // Get the number of wallets used
    let num_wallets_used = wallet_state.read().num_wallets_used;

    let private_key = match private_key_string {
        Some(private_key) => private_key,
        None => return Err(Error::InvalidPrivateKey),
    };

    let wallet_name = match wallet_name {
        Some(wallet_name) => wallet_name,
        None => return Err(Error::InvalidWalletName),
    };

    // Only add if there are available slots
    if num_wallets_used < MAX_WALLETS_ALLOWED {
        // Derive the keypair from the private key
        let keypair_from_private_key =
            match std::panic::catch_unwind(|| Keypair::from_base58_string(&private_key)) {
                Ok(keypair) => keypair,
                Err(_) => return Err(Error::UnableToDeriveKeypair),
            };

        let pubkey_string = keypair_from_private_key.pubkey().to_string();

        // Create pda keypair
        let create_key = Keypair::new();

        // Create a new multisig authority
        let multisig_authority = MultisigAuthority {
            creator: keypair_from_private_key,
            create_key,
        };

        // Serialize the multisig authority
        let bytes = bincode::serialize(&multisig_authority).map_err(|err| {
            println!("{:?}", err);
            Error::BincodeSerialize
        })?;

        // Get write access to the config and update it
        let mut wallet_state = wallet_state.write();

        // Get the pubkeys for the current wallets
        let mut wallet_pubkeys = wallet_state.wallet_pubkeys.clone();

        // Obtain the index we'll assign to this keypair
        let key_index = wallet_state.num_wallets_used;
        wallet_state.current_wallet_index = key_index;
        wallet_state.num_wallets_used += 1;

        // Add the new keypair to the config + our global state
        wallet_pubkeys.push(WalletKey {
            name: wallet_name,
            pubkey: pubkey_string,
            index: key_index,
        });
        wallet_state.wallet_pubkeys = wallet_pubkeys;

        // Save the config first
        save_config(&wallet_state).map_err(|err| {
            println!("{:?}", err);
            Error::SaveWalletConfig
        })?;

        // Drop the lock before calling set
        drop(wallet_state);

        // Set the secret in the keyring with the new index
        set(bytes.as_slice(), key_index)?;

        // Set the new keypair as the current wallet
        current_wallet.set(Wallet::Connected(multisig_authority.creator.pubkey()));

        Ok(())
    } else {
        log::info!("Max number of wallets reached");
        Err(Error::UpdateWalletConfig)
    }
}
