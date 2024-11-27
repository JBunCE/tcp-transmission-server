use tokio::sync::mpsc::Sender;
use crate::models::server_data::Client;

pub enum Signal{
    IncomingClient {
        client: Client,
        stream_uuid: String,
    },
    IncomingStreamer {
        stream_uuid: String,
        user_uuid: String,
        tx: Sender<Signal>
    },
}