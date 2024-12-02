use std::env;

use lapin::{options::BasicPublishOptions, BasicProperties, Connection, ConnectionProperties};
use serde_json::to_vec;

use crate::models::server_events::StreamEvent;


pub async fn message_producer(payload: StreamEvent) ->  Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = env::var("RABBITMQ_ADDR").unwrap_or_else(|_| "amqp://guest:guest@50.19.40.173:5672".into());
    let conn = Connection::connect(&addr, ConnectionProperties::default()).await?;

    let channel = conn.create_channel().await?;
    
    let payload_bytes = to_vec(&payload)?;

    channel.basic_publish(
        "", 
        "StreamEvent", 
        BasicPublishOptions::default(), 
        &payload_bytes,
        BasicProperties::default()
    ).await?;

    println!("Sent message: {:?}", payload);

    Ok(())
}