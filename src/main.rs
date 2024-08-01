use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener};
use tokio::sync::mpsc::{Sender};
use crate::connection_handler::handle_connection;
use crate::models::server_signals::Signal;

pub mod models;
mod utils;
mod connection_handler;
mod stream_handler;

const PORT: i32 = 1935;
const ADDRESS: &str = "0.0.0.0";

#[tokio::main]
async fn main() {
    let server_listener = TcpListener::bind(format!("{}:{}", ADDRESS, PORT)).await.unwrap();
    let (tx, mut rx) = mpsc::channel::<Signal>(100);
    let tx_shared = Arc::new(Mutex::new(tx));

    tokio::spawn(async move {
        let mut streamers: HashMap<u128, Sender<Signal>> = HashMap::new();
        while let Some(message) = rx.recv().await {
            match message {
                Signal::IncomingClient {client, stream_id} => {
                    streamers.get_mut(&stream_id)
                        .unwrap().send(Signal::IncomingClient {client, stream_id}).await.unwrap();
                }
                Signal::IncomingStreamer {stream_id, tx} => {
                    streamers.insert(stream_id, tx);
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
