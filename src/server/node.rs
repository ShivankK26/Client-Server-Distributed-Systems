use tokio::net::{TcpListener, TcpStream};
use std::error::Error;
use log::{info, error};
use crate::client::Message;


pub struct Node {
    addr: String,
    is_leader: bool,
    cluster_addrs: Vec<String>,
}

impl {
    pub fn new(addr: String, is_leader: bool, cluster_addrs: Vec<String>) -> Self {
        Self {
            addr,
            is_leader,
            cluster_addrs
        }
    }

    
}