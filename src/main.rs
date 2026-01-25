use quest_tracker::config::config_loader;
use quest_tracker::config::config_model::DotEnvyConfig;
use tracing::{error, info};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    let dotenvy_env: DotEnvyConfig = match config_loader::load() {
        Ok(env) => env,
        Err(e) => {
            error!("Failed to load ENV : {}", e);
            std::process::exit(1);
        }
    };

    info!("ENV Loaded! {:?}", dotenvy_env);
    info!("Starting Quest Tracker...");
}
