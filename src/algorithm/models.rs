use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Shard {
    pub id: usize,
    pub data: String,
    pub replica_number: usize,
    pub total_shards: usize,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct EncryptedData {
    pub encrypted_data: String,
    pub nonce: String,
}
