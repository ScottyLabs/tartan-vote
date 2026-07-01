mod config;
mod core;
mod domain;
mod server;

use dotenvy::dotenv;
use sea_orm::DatabaseConnection;
use tracing_subscriber::prelude::*;
use voting_app_store::Store;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub store: Store,
    pub config: config::Config,
}

fn main() {
    dotenv().ok();

    let config = config::Config::from_env().expect("failed to load configuration");

    let _sentry_guard = config.sentry_dsn.as_deref().map(|dsn| {
        sentry::init((
            dsn,
            sentry::ClientOptions {
                release: sentry::release_name!(),
                send_default_pii: false,
                ..Default::default()
            },
        ))
    });

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .with(sentry::integrations::tracing::layer())
        .init();

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("failed to build tokio runtime")
        .block_on(server::setup(config));
}
