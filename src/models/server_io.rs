use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "data")]
pub enum ConnectionMessage{
    Connect {
        is_streamer: u8,
        stream_uuid: String,
        user_uuid: String,
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