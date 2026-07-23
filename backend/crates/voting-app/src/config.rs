use anyhow::{Context, Result};
use std::env;

secretspec_derive::declare_secrets!("../../../secretspec.toml");

#[derive(Clone, Debug, Default)]
pub struct Config {
    pub database_url: String,
    pub bind_addr: String,
    pub sentry_dsn: Option<String>,
    pub oidc: OidcSettings,
}

#[derive(Clone, Debug, Default)]
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
    pub fn from_env() -> Result<Self> {
        let secrets = SecretSpec::builder()
            .with_provider("env")
            .with_reason("tartan-vote startup")
            .load()?
            .secrets;

        Ok(Self {
            database_url: env::var("DATABASE_URL").context("DATABASE_URL must be set")?,
            bind_addr: bind_addr_from_env(),
            sentry_dsn: secrets.sentry_dsn.or_else(|| optional_env("SENTRY_DSN")),
            oidc: OidcSettings {
                keycloak_url: secrets.keycloak_url.context("KEYCLOAK_URL must be set")?,
                keycloak_realm: secrets
                    .keycloak_realm
                    .context("KEYCLOAK_REALM must be set")?,
                client_id: secrets
                    .oidc_client_id
                    .context("OIDC_CLIENT_ID must be set")?,
                client_secret: secrets
                    .oidc_client_secret
                    .context("OIDC_CLIENT_SECRET must be set")?,
                app_url: env::var("APP_URL").context("APP_URL must be set")?,
                oauth_relay_url: secrets
                    .oauth_relay_url
                    .context("OAUTH_RELAY_URL must be set")?,
                project_group: secrets.project_group.context("PROJECT_GROUP must be set")?,
                project_admin_group: secrets
                    .project_admin_group
                    .context("PROJECT_ADMIN_GROUP must be set")?,
            },
        })
    }
}

fn bind_addr_from_env() -> String {
    if let Ok(port) = env::var("PORT") {
        return format!("0.0.0.0:{port}");
    }
    env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".to_string())
}

fn optional_env(name: &str) -> Option<String> {
    env::var(name).ok().filter(|value| !value.trim().is_empty())
}
