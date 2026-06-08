use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub app_base_url: String,
    pub frontend_base_url: String,
    pub cors_allowed_origins: Vec<String>,
    pub better_auth_base_url: String,
    pub better_auth_provider_id: String,
    pub database_url: String,
    pub bind_addr: String,
    pub dev_auth_bypass: bool,
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
            better_auth_base_url: must_env("BETTER_AUTH_BASE_URL")?,
            better_auth_provider_id: env::var("BETTER_AUTH_PROVIDER_ID")
                .unwrap_or_else(|_| "cmu-sso".to_string()),
            database_url: must_env("DATABASE_URL")?,
            bind_addr: env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".to_string()),
            dev_auth_bypass: env::var("DEV_AUTH_BYPASS")
                .map(|value| matches!(value.to_ascii_lowercase().as_str(), "1" | "true" | "yes"))
                .unwrap_or(false),
        })
    }
}

fn must_env(name: &str) -> Result<String, String> {
    env::var(name).map_err(|_| format!("{name} must be set"))
}
