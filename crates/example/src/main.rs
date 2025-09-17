use anyhow::Result;
use log::info;
use tokio::time::{Duration, interval};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_default_env()
        .filter(None, log::LevelFilter::Info)
        .init();

    let mut tick = interval(Duration::from_secs(5));

    loop {
        tick.tick().await;
        info!("tick event every 5 seconds !!");
    }
}
