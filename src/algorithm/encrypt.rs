use std::error::Error;
use crate::algorithm::models::{Shard, EncryptedData};
use base64::{prelude::BASE64_STANDARD, Engine};
use sodiumoxide::crypto::secretbox::{self};
use log::{debug};
use sha256::Sha256Digest;

pub fn str_to_shards(inp: &str, total_shards: usize, replicas: usize) -> Result<Vec<Shard>, Box<dyn Error>> {
    debug!("Input: {}, Total shards: {}, Replicas: {}", inp, total_shards, replicas);
    debug!("Converting string to shards...");
    let byte_array = inp.as_bytes();
    
    debug!("Byte array: {:?}", byte_array);

    if byte_array.len() < total_shards {
        return Err("The number of shards must be less than the length of the input string.".into());
    }
    
    let shard_size = (byte_array.len() as f32 / total_shards as f32).ceil() as usize;
    debug!("Shard size: {}", shard_size);
    let mut shard_id = 1;

    let shards = byte_array.chunks(shard_size).map(|chunk| {
        let mut replica_array = Vec::<Shard>::new();
        let mut replica_number = 1;
        for _ in 0..replicas {
            let mut shard = Shard {
                id: shard_id,
                data: String::new(),
                replica_number: replica_number,
                total_shards,
            };
            replica_number += 1;
            shard.data = BASE64_STANDARD.encode(chunk);
            debug!("Shard Replica {}: {:?}", shard.id, shard);
            replica_array.push(shard);
        }
        shard_id += 1;
        replica_array
    }).collect::<Vec<Vec<Shard>>>().into_iter().flatten().collect::<Vec<Shard>>();
    debug!("Success");
    debug!("Shards: {:?}", shards);
    Ok(shards)
}

pub fn encrypt_shards(shards: &mut Vec<Shard>, key: &str) -> Vec<EncryptedData>{
    debug!("Encrypting shards...");
    let mut encrypted_shards = Vec::<EncryptedData>::new();
    let sha_256 = Sha256Digest::digest(key.as_bytes()).as_bytes()[..32].to_vec();
    let secret_key = secretbox::Key::from_slice(&sha_256).unwrap();
    for shard in shards {
        let shard_json = serde_json::to_string(shard);
        let nonce = secretbox::gen_nonce();
        let encrypted_text = secretbox::seal(&shard_json.unwrap().as_bytes(), &nonce, &secret_key);
        let encrypted_shard = EncryptedData {
            encrypted_data: BASE64_STANDARD.encode(encrypted_text),
            nonce: BASE64_STANDARD.encode(nonce),
        };
        debug!("Shard {}, Replica: {}: {:?}", shard.id, shard.replica_number, encrypted_shard);
        encrypted_shards.push(encrypted_shard);
    }
    encrypted_shards
}

pub fn package(shards: Vec<EncryptedData>) -> Vec<String> {
    debug!("Packaging shards...");
    let mut packaged_shards = Vec::<String>::new();
    for shard in shards {
        let serialised = serde_json::to_string(&shard).unwrap();
        debug!("Packaged shard: {}", serialised);
        let b64 = BASE64_STANDARD.encode(&serialised.as_bytes());
        debug!("Packaged shard (base64): {}", b64);
        packaged_shards.push(b64);
    }
    packaged_shards
}