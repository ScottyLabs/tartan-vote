use std::env;

#[derive(Clone, Debug, Default)]
pub struct Config {
    pub database_url: String,
    pub bind_addr: String,
    pub sentry_dsn: Option<String>,
    pub oidc: Option<OidcSettings>,
}

#[derive(Clone, Debug)]
pub struct OidcSettings {
    pub keycloak_url: String,
    pub keycloak_realm: String,
    pub client_id: String,
    pub client_secret: String,
    pub app_url: String,
    pub oauth_relay_url: String,
    pub project_group: String,
    pub project_admin_group: String,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        Ok(Self {
            database_url: must_env("DATABASE_URL")?,
            bind_addr: bind_addr_from_env(),
            sentry_dsn: optional_env("SENTRY_DSN"),
            oidc: OidcSettings::from_env(),
        })
    }
}

impl OidcSettings {
    fn from_env() -> Option<Self> {
        Some(Self {
            keycloak_url: optional_env("KEYCLOAK_URL")?,
            keycloak_realm: optional_env("KEYCLOAK_REALM")?,
            client_id: optional_env("OIDC_CLIENT_ID")?,
            client_secret: optional_env("OIDC_CLIENT_SECRET")?,
            app_url: optional_env("APP_URL")?,
            oauth_relay_url: optional_env("OAUTH_RELAY_URL")?,
            project_group: optional_env("PROJECT_GROUP")?,
            project_admin_group: optional_env("PROJECT_ADMIN_GROUP")?,
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
