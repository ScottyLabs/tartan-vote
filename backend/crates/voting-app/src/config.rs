use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub bind_addr: String,
    pub sentry_dsn: Option<String>,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        Ok(Self {
            database_url: must_env("DATABASE_URL")?,
            bind_addr: bind_addr_from_env(),
            sentry_dsn: optional_env("SENTRY_DSN"),
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
