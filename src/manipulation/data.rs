use serde::{Deserialize, Serialize};

use rdkafka::message::FromBytes;

#[derive(Debug, Clone)]
pub struct Hist {
    pub id: u8,
    pub value: u32,
}

impl Hist {
    pub fn default() -> Hist {
        Hist { id: 0, value: 0 }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct URedisPayload {
    pub key: String,
    pub ttl: u32,
    pub payload: String,
}

#[derive(Debug, Deserialize)]
pub struct URedisResponse {
    pub code: u32,
    pub msg: String
}

impl URedisPayload {
    pub fn new(key: &str, value: String) -> URedisPayload {
        URedisPayload{key: key.to_string(), ttl: 100u32, payload: value}
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KafkaConsumerPayload {
    pub key: String,
    pub img: String,
}
