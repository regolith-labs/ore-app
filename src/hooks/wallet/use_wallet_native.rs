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
const MAX_WALLETS_ALLOWED: u8 = 3;

/*
    read all keychains that exists by hardcoded keys
    only populate keypairs that exist in the keychain
        try to read all 3 keypairs
            use the config to which keypair is selected
            default to the first keypair
    get the amount of keys in use by the keychain
*/

#[derive(Serialize, Deserialize, Clone)]
pub struct WalletConfig {
    pub current_wallet_index: u8,
    pub num_wallets_used: u8,
    pub wallet_pubkeys: Vec<String>,
}

pub fn use_wallet_provider() {
    // TODO: vec of Vec<Wallet
    let mut wallet_config_signal = use_context_provider(|| {
        Signal::new(WalletConfig {
            current_wallet_index: 0,
            num_wallets_used: 1,
            wallet_pubkeys: vec![],
        })
    });

    let mut wallet_signal: Signal<Wallet> =
        use_context_provider(|| Signal::new(Wallet::Disconnected));

    // Try to load keypair and config
    let mut keychain_data = get_or_set();

    // Update the signals with the loaded values
    use_effect(move || match &keychain_data {
        Ok((multisig_authority, config)) => {
            wallet_config_signal.set(config.clone());
            wallet_signal.set(Wallet::Connected(multisig_authority.creator.pubkey()));
        }
        Err(err) => {
            log::error!("Error in wallet provider: {:?}", err);
        }
    });
}

pub fn use_wallet_config() -> Signal<WalletConfig> {
    use_context::<Signal<WalletConfig>>()
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

pub fn get() -> Result<(MultisigAuthority, WalletConfig), Error> {
    // Let's try to load the config if it exists
    if let Ok(config) = load_config() {
        let current_index = config.current_wallet_index;
        let (service, user_device_key) = get_keyring_values_by_index(current_index);
        let keyring = Entry::new(service, user_device_key)?;
        let secret = keyring.get_secret()?;
        let multisig_authority = bincode::deserialize(secret.as_slice()).map_err(|err| {
            println!("{:?}", err);
            Error::BincodeDeserialize
        })?;
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

pub fn get_or_set() -> Result<(MultisigAuthority, WalletConfig), Error> {
    match get() {
        /*
        read all keypairs,
        return keypairnoentry for those keypairs that aren't ok
        only return keypairs that are ok
         */
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

                let mut config = WalletConfig {
                    current_wallet_index: 0,
                    num_wallets_used: 1,
                    wallet_pubkeys: vec![],
                };
                log::info!("pubkey: {:?}", multisig_authority.creator.pubkey());
                config
                    .wallet_pubkeys
                    .push(multisig_authority.creator.pubkey().to_string());

                // Ensure we're able to save the config
                match save_config(&config) {
                    Ok(_) => {
                        let current_index = config.current_wallet_index;
                        // Set the secret in the keyring
                        set(bytes.as_slice(), current_index)?;
                        Ok((multisig_authority, config))
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

fn save_config(config: &WalletConfig) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(path) = get_config_path() {
        // Create parent directory if it doesn't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Serialize the config
        let json = serde_json::to_string_pretty(config)?;

        // Write the config to the file
        fs::write(&path, &json)?;
        log::info!("Successfully wrote config to: {:?}", path);
    }
    Ok(())
}

fn load_config() -> Result<WalletConfig, Error> {
    if let Some(path) = get_config_path() {
        if path.exists() {
            let data = fs::read_to_string(&path)?;
            let config: WalletConfig = serde_json::from_str(&data)?;
            return Ok(config);
        } else {
            return Err(Error::ConfigNotFound);
        }
    } else {
        return Err(Error::ConfigNotFound);
    }
}

pub fn add_new_keypair(private_key_string: Option<String>) -> Result<(), Error> {
    let mut current_wallet = use_wallet();
    //TODO: check if user has already imported this key

    // Get current wallet config
    let mut wallet_config_signal = use_wallet_config();

    // Get the number of wallets used
    let num_wallets_used = wallet_config_signal.read().num_wallets_used;
    log::info!("num_wallets_used: {:?}", num_wallets_used);

    let private_key = match private_key_string {
        Some(private_key) => private_key,
        None => return Err(Error::InvalidPrivateKey),
    };

    // We can only add if we are less than the permissible number of wallets
    if num_wallets_used < MAX_WALLETS_ALLOWED {
        // Derive the keypair from the private key
        // let keypair_from_private_key = Keypair::from_base58_string(&private_key);

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
        let mut wallet_config = wallet_config_signal.write();

        // Get the pubkeys for the current wallets
        let mut wallet_pubkeys = wallet_config.wallet_pubkeys.clone();

        // Add the new pubkey to the config
        wallet_pubkeys.push(pubkey_string);
        wallet_config.wallet_pubkeys = wallet_pubkeys;

        // Get and update the current index
        let new_index = wallet_config.current_wallet_index + 1;
        wallet_config.current_wallet_index = new_index;
        wallet_config.num_wallets_used += 1;

        log::info!(
            "Wallet config updated to index: {}, Num wallets: {}, Pubkeys: {:?}",
            wallet_config.current_wallet_index,
            wallet_config.num_wallets_used,
            wallet_config.wallet_pubkeys
        );

        // Save the config first
        save_config(&wallet_config).map_err(|err| {
            println!("{:?}", err);
            Error::SaveWalletConfig
        })?;
        log::info!("Config saved");

        // Drop the lock before calling set
        drop(wallet_config);

        // Set the secret in the keyring with the new index
        set(bytes.as_slice(), new_index)?;
        log::info!("Set done");

        current_wallet.set(Wallet::Connected(multisig_authority.creator.pubkey()));

        // TODO: SET THE NEW WALLET AS THE CURRENT WALLLET (USE_WALLET HOOK)

        Ok(())
    } else {
        log::info!("Max number of wallets reached");
        Err(Error::UpdateWalletConfig)
    }
}

// / Returns the public key of the imported wallet on success
// pub fn import_private_key(private_key_string: &str) -> Result<Pubkey, Error> {
//     // Validate and parse the private key
//     let imported_keypair = match Keypair::from_base58_string(private_key_string) {
//         Ok(keypair) => keypair,
//         Err(_) => return Err(Error::InvalidPrivateKey),
//     };

//     log::info!("Parsed private key successfully");

//     // Get current wallet config
//     let mut wallet_config_signal = use_wallet_config();
//     let num_wallets_used = wallet_config_signal.read().num_wallets_used;

//     // // Check if we've reached the maximum number of wallets
//     if num_wallets_used >= MAX_WALLETS_ALLOWED {
//         log::info!("Maximum number of wallets reached");
//         return Err(Error::UpdateWalletConfig);
//     }

//     // Create a new MultisigAuthority with the imported keypair
//     let create_key = Keypair::new(); // Generate a new create_key
//     let multisig_authority = MultisigAuthority {
//         creator: imported_keypair,
//         create_key,
//     };

//     // Serialize the multisig authority
//     let bytes = bincode::serialize(&multisig_authority).map_err(|err| {
//         log::error!("Failed to serialize multisig authority: {:?}", err);
//         Error::BincodeSerialize
//     })?;

//     // Update the wallet config
//     let mut wallet_config = wallet_config_signal.write();
//     let new_index = wallet_config.current_wallet_index + 1;
//     wallet_config.current_wallet_index = new_index;
//     wallet_config.num_wallets_used += 1;

//     // Save the config to disk
//     save_config(&wallet_config).map_err(|err| {
//         log::error!("Failed to save wallet config: {:?}", err);
//         Error::SaveWalletConfig
//     })?;

//     // Release the lock before calling set
//     drop(wallet_config);

//     // Set the secret in the keyring with the new index
//     set(bytes.as_slice(), new_index)?;

//     // Return the public key so the UI can show it
//     Ok(imported_keypair.pubkey())
// }

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
