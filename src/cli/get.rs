use std::error::Error;
use crate::transmit::client::{get_shards_from_ipfs, pin_to_instance};
use crate::algorithm::decrypt::{unpackage, decrypt_shards, shards_to_str};
use base64::{prelude::BASE64_STANDARD, Engine};
use log::info;
use crate::steg::decoder::decode_from_image;

pub async fn get(location: String, key: &str) -> Result<String, Box<dyn Error>> {
    let encoded_hash_array = decode_from_image(location.as_str());
    let hashes = BASE64_STANDARD.decode(encoded_hash_array.as_bytes())?;
    let string_hashes = String::from_utf8(hashes)?;
    let hashes = string_hashes
        .split(",")
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|hash| hash.to_string())
        .collect::<Vec<String>>();
    let retrieved_shards = get_shards_from_ipfs(hashes).await?;
    let unpacked_shards = unpackage(retrieved_shards)?;
    let decrypted_shards = decrypt_shards(unpacked_shards, key)?;
    let output = shards_to_str(decrypted_shards)?;
    Ok(output)
}

pub async fn get_to_pin(location: String) -> Result<(), Box<dyn Error>> {
    let encoded_hash_array = decode_from_image(location.as_str());
    let hashes = BASE64_STANDARD.decode(encoded_hash_array.as_bytes())?;
    let string_hashes = String::from_utf8(hashes)?;
    let hashes = string_hashes
        .split(",")
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|hash| hash.to_string())
        .collect::<Vec<String>>();

    match pin_to_instance(hashes).await {
        Ok(_) => {
            info!("Shards pinned to IPFS.");
            Ok(())
        },
        Err(e) => return Err(e.into()),
    }
}