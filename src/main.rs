use anyhow::Result;
use chrono::Local;
use clap::Parser;
use log::info;
use raft::config::CliConfig;
use raft::config::Config;
use raft::enter;
use std::fs::File;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    init_logger();
    let mut config = load_config()?;
    let args = CliConfig::parse();
    dbg!(&args);
    config = update_config(args, config);
    enter(config).await?;
    Ok(())
}
fn init_logger() {
    env_logger::builder()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}:{}] [{}] - {}",
                Local::now().format("[%Y-%m-%d %H:%M:%S]"),
                record.file().unwrap_or("non location"),
                record.line().unwrap_or(0),
                record.level(),
                record.args()
            )
        })
        .init();
    info!("init env logger");
}

fn load_config() -> Result<Config> {
    let config_file = File::open("config.yaml")?;
    Ok(serde_yaml::from_reader(config_file)?)
}

fn update_config(cli_config: CliConfig, mut config: Config) -> Config {
    if let Some(bind) = cli_config.bind {
        config.bind = bind;
    }

    if let Some(port) = cli_config.port {
        config.port = port;
    }

    if let Some(peers) = cli_config.peers {
        config.peers = peers;
    }
    config
}
