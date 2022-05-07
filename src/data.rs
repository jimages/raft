use anyhow::Result;
use std::net::SocketAddrV4;
use std::time::Instant;
use tokio::net::{TcpListener, TcpStream};

#[derive(Debug)]
pub struct Node {
    pub ip: SocketAddrV4,
    pub role: Role,         // leader, follower, candidate
    pub last_time: Instant, // 最后一次联络的时间
    pub steam: TcpStream,
    pub listener: TcpListener,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Status {
    pub bind_address: SocketAddrV4,
    pub role: Role,         // leader, follower, candidate
    pub last_time: Instant, // 最后一次联络的时间
    pub term: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Role {
    Master,
    Follower,
    Candidate,
}
