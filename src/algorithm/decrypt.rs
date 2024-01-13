use crate::algorithm::models::EncryptedData;
use base64::{prelude::BASE64_STANDARD, Engine};
use log::{debug, warn};
use sodiumoxide::crypto::secretbox;
use std::error::Error;
use sha256::Sha256Digest;

use super::models::Shard;

pub fn unpackage(data: Vec<String>) -> Result<Vec<EncryptedData>, Box<dyn Error>> {
    let mut decoded_shards = Vec::<EncryptedData>::new();

    for datum in data {
        let decoded_data = BASE64_STANDARD.decode(&datum.as_bytes());
        debug!("Decoded data: {:?}", decoded_data);
        match decoded_data {
            Ok(decoded_data) => match serde_json::from_slice::<EncryptedData>(&decoded_data) {
                Ok(encrypted_data) => {
                    debug!("Resulting Encrypted Shard: {:?}", encrypted_data);
                    decoded_shards.push(encrypted_data);
                }
                Err(e) => return Err(e.into()),
            },
            Err(e) => return Err(e.into()),
        }
    }
    Ok(decoded_shards)
}

pub fn decrypt_shards(shards: Vec<EncryptedData>, key: &str) -> Result<Vec<Shard>, Box<dyn Error>> {
    let mut decrypted_shards = Vec::<Shard>::new();
    for shard in shards {
        let nonce_string = BASE64_STANDARD.decode(&shard.nonce.as_bytes());
        let encrypted_data = BASE64_STANDARD.decode(&shard.encrypted_data.as_bytes());
        let sha_256 = Sha256Digest::digest(key.as_bytes()).as_bytes()[..32].to_vec();
        let secret_key = secretbox::Key::from_slice(&sha_256).unwrap();
        match (nonce_string, encrypted_data) {
            (Ok(nonce), Ok(data)) => {
                let decrypted_data = secretbox::open(
                    &data,
                    &secretbox::Nonce::from_slice(&nonce).unwrap(),
                    &secret_key,
                );
                match decrypted_data {
                    Ok(decrypted_data) => {
                        debug!("Decrypted data: {:?}", decrypted_data);
                        let shard = serde_json::from_slice::<Shard>(&decrypted_data);
                        match shard {
                            Ok(shard) => {
                                debug!("Decrypted shard: {:?}", shard);
                                decrypted_shards.push(shard);
                            }
                            Err(e) => return Err(Box::<dyn Error>::from(e)),
                        }
                    }
                    _ => {
                        warn!("Decryption failed. Empty payload");
                        return Err("Decryption failed.".into());
                    }
                }
            }
            (Ok(_), Err(e)) => return Err(e.into()),
            (Err(e), _) => return Err(e.into()),
        }
    }
    Ok(decrypted_shards)
}

pub fn shards_to_str(shards: Vec<Shard>) -> Result<String, Box<dyn Error>> {
    if shards.len() == 0 {
        warn!("No shards to convert.");
        return Err("No shards to convert.".into());
    }
    let mut data = String::new();
    let mut found_shards = 1;
    let mut shard_collection = Vec::<Shard>::new();
    debug!("Searching for a complete shard set...");
    for shard in shards {
        if shard.id == found_shards {
            debug!("Found shard for ID : {:?}", shard.id);
            shard_collection.push(shard);
            found_shards += 1;
        }
    }
    debug!("Found a complete shard set.");
    for shard in shard_collection {
        let decoded_shard = BASE64_STANDARD.decode(&shard.data.as_bytes());
        match decoded_shard {
            Ok(decoded_shard) => match String::from_utf8(decoded_shard) {
                Ok(decoded_shard) => {
                    debug!("Decoded shard: {}", decoded_shard);
                    data.push_str(&decoded_shard);
                }
                Err(e) => return Err(e.into()),
            },
            Err(e) => return Err(e.into()),
        }
    }
    Ok(data)
}
