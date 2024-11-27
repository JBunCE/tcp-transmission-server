use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use uuid::Uuid;
use tokio::sync::mpsc::Sender;
use tokio::sync::{mpsc, Mutex};
use crate::models::server_data::Client;
use crate::models::server_io::ConnectionMessage;
use crate::models::server_signals::Signal;
use crate::stream_handler::handle_stream_client;
use crate::utils::message_decoder::{decode_message, from_utf8};

pub async fn handle_connection(mut socket: TcpStream, tx: Arc<Mutex<Sender<Signal>>>) {
    let buffer = &mut [0; 2048];
    let message = socket.read(buffer).await.unwrap();

    println!("Message: {}", from_utf8(&buffer[..message]));

    match decode_message::<ConnectionMessage>(from_utf8(&buffer[..message])) {
        ConnectionMessage::Connect { is_streamer, stream_uuid, user_uuid } => {
            if is_streamer == 1 {
                let (s_tx, s_rx) = mpsc::channel::<Signal>(100);
                let tx = tx.lock().await;
                tx.send(Signal::IncomingStreamer {
                    stream_uuid,
                    user_uuid,
                    tx: s_tx.clone(),
                }).await.unwrap();
                tokio::spawn(async move {
                    handle_stream_client(socket, s_rx).await;
                });
            } else {
                let tx = tx.lock().await;
                tx.send(Signal::IncomingClient {
                    client: Client {
                        address: socket.peer_addr().unwrap().to_string(),
                        socket,
                    },
                    stream_uuid,
                }).await.unwrap();
            }
        },
        _  => {
            println!("Error: Invalid message.");
        }
    }
}