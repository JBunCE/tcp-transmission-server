use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "data")]
pub enum ConnectionMessage{
    Connect {
        is_streamer: u8,
        stream_id: u128,
    },
    Disconnect,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum StreamerMessage{
    Start,
    Stop,
    Frame {
        bytes: String,
    },
}