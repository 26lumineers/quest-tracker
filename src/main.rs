use quest_tracker::config::config_loader;
use tracing::{info, error};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).init();
    let dotenvy_env = match config_loader::load() {
        Ok(env: DotEnvyConfig) => env,
        Err(e: Error) => {
            error!("Failed to load ENV : {}",e);
            std::process::exit(1);
        }
    };

    info!("ENV Loaded! {:?}",dotenvy_env);
    info!("Starting Quest Tracker...");
}
