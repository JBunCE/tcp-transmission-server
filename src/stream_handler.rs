use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::mpsc::Receiver;
use tokio::sync::Mutex;
use crate::models::server_data::Client;
use crate::models::server_io::StreamerMessage;
use crate::models::server_signals::Signal;
use crate::utils::message_decoder::{decode_message, from_utf8};

pub async fn handle_stream_client(mut socket: TcpStream, mut rx: Receiver<Signal>) {
    let clients: Arc<Mutex<Vec<Client>>> = Arc::new(Mutex::new(Vec::new()));
    let buffer = &mut [0; 100000];

    let clients_cloned = clients.clone();
    tokio::spawn(async move {
        loop {
            if let Some(message) = rx.recv().await {
                match message {
                    Signal::IncomingClient { client, stream_uuid: _ } => {
                        let mut clients = clients_cloned.lock().await;
                        clients.push(client);
                    }
                    _ => { println!("Error: Invalid message.") }
                }
            }
        }
    });

    loop {
        let message = socket.read(buffer).await.unwrap();
        if message > 0 {
            let (
                message_c,
                buffer_c,
            ) = (message.clone(), buffer.clone());
            let clients_cloned = clients.clone();
            tokio::spawn(async move {
               match decode_message::<StreamerMessage>(from_utf8(&buffer_c[..message_c])) {
                    StreamerMessage::Start => {
                        println!("Start");
                    },
                    StreamerMessage::Stop => {
                        println!("Stop");
                    },
                    StreamerMessage::Frame { bytes } => {
                        let mut clients = clients_cloned.lock().await;
                        let mut black_list = Vec::new();
                        for client in clients.iter_mut() {
                            if let Err(_) = client.socket.write_all((&bytes).as_ref()).await {
                                println!("Error: Couldn't send frame to client.");
                                black_list.push(client.address.clone());
                            }
                        }
                        clients.retain(|client| !black_list.contains(&client.address));
                    }
               }
            });
            for i in 0..message {
                buffer[i] = 0;
            }
        }
    }
}