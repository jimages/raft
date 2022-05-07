use anyhow::Result;
use clap::Parser;
use std::net::{Ipv4Addr, SocketAddrV4};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub bind: Ipv4Addr,

    pub port: u16,

    pub peers: Vec<SocketAddrV4>,
}

/// wangrenquan's raft demo
#[derive(Deserialize, Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct CliConfig {
    /// bind address
    #[clap(short, long)]
    pub bind: Option<Ipv4Addr>,

    /// port
    #[clap(short, long)]
    pub port: Option<u16>,

    /// peers
    #[clap(long, use_value_delimiter(true))]
    pub peers: Option<Vec<SocketAddrV4>>,
}
