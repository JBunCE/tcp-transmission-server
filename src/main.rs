use std::collections::HashMap;
use std::sync::Arc;
use models::server_events::StreamEvent;
use tokio::sync::{mpsc, Mutex};
use tokio::net::TcpListener;
use tokio::sync::mpsc::Sender;
use crate::connection_handler::handle_connection;
use crate::models::server_signals::Signal;

pub mod models;
mod utils;
mod connection_handler;
mod stream_handler;
mod broker;

const PORT: i32 = 1998;
const ADDRESS: &str = "0.0.0.0";

#[tokio::main]
async fn main() {
    let server_listener = TcpListener::bind(format!("{}:{}", ADDRESS, PORT)).await.unwrap();
    let (tx, mut rx) = mpsc::channel::<Signal>(100);
    let tx_shared = Arc::new(Mutex::new(tx));

    println!("Server started at {}:{}", ADDRESS, PORT);

    tokio::spawn(async move {
        let mut streamers: HashMap<String, Sender<Signal>> = HashMap::new();
        while let Some(message) = rx.recv().await {
            match message {
                Signal::IncomingClient {client, stream_uuid} => {
                    streamers.get_mut(&stream_uuid)
                        .unwrap().send(Signal::IncomingClient {client, stream_uuid}).await.unwrap();
                }
                Signal::IncomingStreamer {stream_uuid, tx, user_uuid} => {
                    streamers.insert(stream_uuid.clone(), tx);

                    let event = StreamEvent {
                        uuid: stream_uuid,
                        user_id: user_uuid,
                        access_token: String::from("NA")
                    };

                    let _ = broker::message_producer(event).await;
                }
            }
        }
    });

    loop {
        let (socket, _) = server_listener.accept().await.unwrap();
        let tx_shared = tx_shared.clone();
        tokio::spawn(async move {
            handle_connection(socket, tx_shared).await;
        });
    }
}
