use std::io::Cursor;
use log::debug;
use ipfs_api::{IpfsClient, IpfsApi};
use std::error::Error;
use futures::TryStreamExt;

pub async fn send_shards_to_ipfs(shards: Vec<String>) -> Result<Vec<String>, Box<dyn Error>>{
    debug!("Sending shards to IPFS..");
    let ipfs = IpfsClient::default();
    let cursors = shards.into_iter().map(|shard| Cursor::new(shard.into_bytes()));
    let mut ipfs_hashes = Vec::<String>::new();
    for cursor in cursors {
        debug!("Transmitting cursor to IPFS..");
        match ipfs.add(cursor).await {
            Ok(res) => {
                debug!("IPFS Hash: {}", res.hash);
                ipfs_hashes.push(res.hash);
            }
            Err(e) => return Err(e.into()),
        }
    }
    debug!("Shards sent to IPFS.");
    Ok(ipfs_hashes)
}

pub async fn get_shards_from_ipfs(hashes: Vec<String>) -> Result<Vec<String>, Box<dyn Error>>{
    debug!("Getting shards from IPFS..");
    let ipfs = IpfsClient::default();
    let mut shards = Vec::<String>::new();
    for hash in hashes {
        debug!("Retrieving shard from IPFS..");
        ipfs.cat(&hash).map_ok(|chunk| {
            let buf = chunk.to_vec();
            String::from_utf8(buf).unwrap()
        }).try_for_each(|shard| {
            shards.push(shard);
            futures::future::ready(Ok(()))
        }).await?;
    }
    debug!("Shards retrieved from IPFS.");
    Ok(shards)
}