use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;
use std::error::Error;
use log::{info, error};


pub enum Message {
    Request { id: String, data: String },
    Response { id: String, data: String },
}

pub struct Client {
    leader_addr: String,
}

impl Client {
    pub fn new(leader_addr: String) -> Self {
        Self{ leader_addr }
    }

    pub async fn start(&self) -> Result<(), Box<dyn Error>> {
        info!("Starting the client, connecting to leader at {}", self.leader_addr);

        let stream = TcpStream::connect(&self.leader_addr).await?;

        loop {
            let request = Message::Request {
                id: uuid:Uuid::new_v4().to_string(), // Example output: 550e8400-e29b-41d4-a716-446655440000
                data: "sample request".to_string(),
            };

            // send request 
            let request_str = serde_json::to_string(&request)?; // serializes the data to JSON string
            // handle sending request...

            // receive response
            // handle receiving response...

            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    }
}