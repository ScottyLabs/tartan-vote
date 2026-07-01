use std::{env, path::PathBuf};

use crate::core::frontend;

#[derive(Clone, Debug)]
pub struct Config {
    pub app_base_url: String,
    pub frontend_base_url: String,
    pub cors_allowed_origins: Vec<String>,
    pub database_url: String,
    pub bind_addr: String,
    pub sentry_dsn: Option<String>,
    pub frontend_dist: Option<PathBuf>,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        let frontend_base_url = must_env("FRONTEND_BASE_URL")?;
        let cors_allowed_origins = env::var("CORS_ALLOWED_ORIGINS")
            .ok()
            .map(|raw| {
                raw.split(',')
                    .map(str::trim)
                    .filter(|value| !value.is_empty())
                    .map(ToOwned::to_owned)
                    .collect::<Vec<_>>()
            })
            .filter(|origins| !origins.is_empty())
            .unwrap_or_else(|| vec![frontend_base_url.clone()]);

        Ok(Self {
            app_base_url: must_env("APP_BASE_URL")?,
            frontend_base_url,
            cors_allowed_origins,
            database_url: must_env("DATABASE_URL")?,
            bind_addr: bind_addr_from_env(),
            sentry_dsn: optional_env("SENTRY_DSN"),
            frontend_dist: frontend::resolve_frontend_dist(),
        })
    }
}

fn bind_addr_from_env() -> String {
    if let Ok(port) = env::var("PORT") {
        return format!("0.0.0.0:{port}");
    }
    env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".to_string())
}

fn must_env(name: &str) -> Result<String, String> {
    env::var(name).map_err(|_| format!("{name} must be set"))
}

fn optional_env(name: &str) -> Option<String> {
    env::var(name).ok().filter(|value| !value.trim().is_empty())
}
