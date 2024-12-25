use std::error::Error;
use tokio;

mod client;
mod server;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    // start 4 server nodes
    let server_addresses = vec![
        "127.0.0.1:8001",
        "127.0.0.1:8002", 
        "127.0.0.1:8003",
        "127.0.0.1:8004"
    ];

    // initialize the servers
    let mut handles = vec![];

    // start leader node
    let leader = 
    println!("Hello, world!");
}
