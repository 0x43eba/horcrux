use crate::algorithm::encrypt::{encrypt_shards, package, str_to_shards};
use std::error::Error;
use crate::transmit::client::send_shards_to_ipfs;
use base64::{prelude::BASE64_STANDARD, Engine};
use log::info;

pub async fn send(input: &str, key: &str, reps: usize, shards: usize) -> Result<(), Box<dyn Error>> {
    println!("{}", input);
    let mut shards = str_to_shards(input, shards, reps).unwrap();
    let encrypted_shards = encrypt_shards(&mut shards, key);
    let packaged_shards = package(encrypted_shards);
    let ipfs_hashes = send_shards_to_ipfs(packaged_shards).await.unwrap();
    let hashes = ipfs_hashes.join(",");
    let hashes = BASE64_STANDARD.encode(hashes.as_bytes());
    info!("Hashes: {}", hashes);
    Ok(())
}