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

use crate::hooks::use_wallet;

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
    let keychain_data = get_or_set();

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
    // Load config
    let config = load_config()?;
    // Read current selected keypair
    let current_index = config.current_wallet_index;
    let multisig_authority = get_keypair(current_index)?;
    Ok((multisig_authority, config))
}

pub fn get_or_set() -> Result<(MultisigAuthority, WalletState), Error> {
    // Try reading the config
    let config = load_config();
    match config {
        // Config exists,
        // try reading current keypair that it points to
        Ok(config) => {
            // If this fails just early exit instead of defaulting to reading the first keypair.
            // This is a case where we would need to reset the config to match the actual state of
            // the keypairs in the keychain.
            let current_keypair = get_keypair(config.current_wallet_index)?;
            Ok((current_keypair, config))
        }
        // No config was found
        Err(_err) => {
            // First, try reading the first keypair,
            // Otherwise create a new keypair
            let multisig_authority = match get_keypair(0) {
                Ok(first_keypair) => first_keypair,
                Err(err) => {
                    // If there error explicitly says there is no keypair at this index, then
                    // create new keypair.
                    if let Error::KeyringNoEntry = err {
                        // Create a new keypair
                        let creator = Keypair::new();
                        let create_key = Keypair::new();
                        let multisig_authority = MultisigAuthority {
                            creator,
                            create_key,
                        };
                        // Write keypair to keychain
                        let bytes = bincode::serialize(&multisig_authority).map_err(|err| {
                            println!("{:?}", err);
                            Error::BincodeSerialize
                        })?;
                        set(bytes.as_slice(), 0)?;
                        multisig_authority
                    } else {
                        // Other error that doesn't indicate that there definitely is *not* already a
                        // wallet stored on the keychain. Just return error and invoke retry later
                        // to avoid overwrite.
                        return Err(err);
                    }
                }
            };
            // Initialize new config with first keypair
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
            // Write config
            save_config(&wallet_state)?;
            Ok((multisig_authority, wallet_state))
        }
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

        // Set the secret in the keyring first
        let mut wallet_state = wallet_state.write();
        let key_index = wallet_state.num_wallets_used;
        set(bytes.as_slice(), key_index)?;

        // Get the pubkeys for the current wallets
        let mut wallet_pubkeys = wallet_state.wallet_pubkeys.clone();

        // Obtain the index we'll assign to this keypair
        wallet_state.current_wallet_index = key_index;
        wallet_state.num_wallets_used += 1;

        // Add the new keypair to the config
        wallet_pubkeys.push(WalletKey {
            name: wallet_name,
            pubkey: pubkey_string,
            index: key_index,
        });
        wallet_state.wallet_pubkeys = wallet_pubkeys;

        // Save the config
        // If this fails just return error,
        // as this is a case where we would need to reset the config to match the actual state of
        // the keypairs in the keychain.
        save_config(&wallet_state)?;

        // Set the new keypair as the current wallet
        current_wallet.set(Wallet::Connected(multisig_authority.creator.pubkey()));
        Ok(())
    } else {
        log::info!("Max number of wallets reached");
        Err(Error::UpdateWalletConfig)
    }
}

pub fn save_config(config: &WalletState) -> Result<(), Error> {
    let path = get_config_path()?;
    // Create parent directory if it doesn't exist
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Serialize the config
    let json = serde_json::to_string_pretty(config)?;

    // Write the config to the file
    fs::write(&path, &json)?;
    Ok(())
}

fn get_keyring_values_by_index(index: u8) -> Result<(&'static str, &'static str), Error> {
    match index {
        0 => Ok(("ORE", "user-device-key")),
        1 => Ok(("ORE-two", "user-device-key-two")),
        2 => Ok(("ORE-three", "user-device-key-three")),
        _ => Err(Error::UnableToDeriveKeypair),
    }
}

fn get_keypair(index: u8) -> Result<MultisigAuthority, Error> {
    let (service, user_device_key) = get_keyring_values_by_index(index)?;
    let keyring = Entry::new(service, user_device_key)?;
    let secret = keyring.get_secret()?;
    let multisig_authority =
        bincode::deserialize(secret.as_slice()).map_err(|_err| Error::BincodeDeserialize)?;
    Ok(multisig_authority)
}

fn set(secret: &[u8], index: u8) -> Result<(), Error> {
    let (service, user_device_key) = get_keyring_values_by_index(index)?;
    let keyring = Entry::new(service, user_device_key)?;
    keyring.set_secret(secret).map_err(From::from)
}

fn get_config_path() -> Result<PathBuf, Error> {
    ProjectDirs::from("", "", "Ore")
        .map(|dirs| {
            let config_dir = dirs.config_dir();
            fs::create_dir_all(config_dir).ok();
            config_dir.join("config.json")
        })
        .ok_or(Error::ConfigNotFound)
}

fn load_config() -> Result<WalletState, Error> {
    let path = get_config_path()?;
    if path.exists() {
        let data = fs::read_to_string(&path)?;
        let config: WalletState = serde_json::from_str(&data)?;
        Ok(config)
    } else {
        Err(Error::ConfigNotFound)
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
