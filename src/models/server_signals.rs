use tokio::sync::mpsc::Sender;
use crate::models::server_data::Client;

pub enum Signal{
    IncomingClient {
        client: Client,
        stream_id: u128,
    },
    IncomingStreamer {
        stream_id: u128,
        tx: Sender<Signal>
    },
}