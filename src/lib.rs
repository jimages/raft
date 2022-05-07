pub mod config;
mod data;

use std::net::SocketAddrV4;
use std::time::Instant;

use anyhow::Error;
use anyhow::Result;
use config::Config;
use data::Node;
use data::Role;
use data::Status;
use log::error;
use log::info;
use once_cell::sync::OnceCell;

static STATUS: OnceCell<Status> = OnceCell::new();
static NODES: OnceCell<Vec<Node>> = OnceCell::new();

pub async fn enter(config: Config) -> Result<()> {
    init(&config)?;
    info!("initialize status: {:?}", STATUS.get().unwrap());
    // try to connect peers
    for peer in config.peers {}
    Ok(())
}

pub fn init(config: &Config) -> Result<()> {
    if let Err(value) = STATUS.set(Status {
        bind_address: SocketAddrV4::new(config.bind, config.port),
        role: Role::Follower,
        last_time: Instant::now(),
        term: 0,
    }) {
        error!("error whtn init status: {:?}", value);
        return Err(Error::msg(format!("_init_ status error {:?}", value)));
    }
    Ok(())
}
