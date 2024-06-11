use envconfig::Envconfig as _;
use state::AppState;
use tracing::info;

mod config;
mod errors;
mod router;
mod state;

#[tokio::main(flavor = "current_thread")]
async fn main() -> errors::Result<()> {
    tracing_subscriber::fmt::init();

    let config = config::AppConfig::init_from_env()?;

    let router = router::create(AppState::new());

    let bind_addr = format!("{}:{}", config.host, config.port);
    let listener = tokio::net::TcpListener::bind(&bind_addr).await?;

    info!("Listening on {}...", bind_addr);

    axum::serve(listener, router).await?;

    Ok(())
}
