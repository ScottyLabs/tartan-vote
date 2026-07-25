use axum::{extract::FromRequestParts, http::request::Parts};
use axum_oidc::{
    AdditionalClaims, OidcClient, OidcSession,
    openidconnect::{
        self, ClientId, ClientSecret, CsrfToken, IssuerUrl, Scope, core::CoreGenderClaim,
    },
};
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use serde::{Deserialize, Serialize};
use tower_sessions::Session;

use crate::config::OidcSettings;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupClaims {
    #[serde(default)]
    pub groups: Vec<String>,
}

impl openidconnect::AdditionalClaims for GroupClaims {}
impl AdditionalClaims for GroupClaims {}

pub struct SessionWrapper(Session);

impl<S: Send + Sync> FromRequestParts<S> for SessionWrapper {
    type Rejection = <Session as FromRequestParts<S>>::Rejection;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        Ok(Self(Session::from_request_parts(parts, state).await?))
    }
}

impl<AC: AdditionalClaims> axum_oidc::Session<AC> for SessionWrapper {
    type Error = tower_sessions::session::Error;

    async fn get(&self) -> Result<OidcSession<AC, CoreGenderClaim>, Self::Error> {
        Ok(self.0.get("axum-oidc").await?.unwrap_or_default())
    }

    async fn set(&mut self, value: OidcSession<AC, CoreGenderClaim>) -> Result<(), Self::Error> {
        self.0.insert("axum-oidc", value).await?;
        Ok(())
    }
}

#[derive(Serialize)]
struct RelayState<'a> {
    return_to: &'a str,
    csrf: uuid::Uuid,
}

fn relay_state(return_to: &str) -> String {
    let state = RelayState {
        return_to,
        csrf: uuid::Uuid::new_v4(),
    };
    URL_SAFE_NO_PAD.encode(serde_json::to_vec(&state).expect("serialize relay state"))
}

pub async fn build_client(settings: &OidcSettings) -> anyhow::Result<OidcClient<GroupClaims>> {
    let issuer_url = IssuerUrl::new(format!(
        "{}/realms/{}",
        settings.keycloak_url, settings.keycloak_realm
    ))
    .map_err(|e| anyhow::anyhow!("invalid Keycloak issuer URL: {e}"))?;

    let callback = format!("{}/auth/callback", settings.app_url);

    let client = OidcClient::<GroupClaims>::builder()
        .with_default_http_client()
        .with_redirect_url(
            axum::http::Uri::try_from(settings.oauth_relay_url.clone())
                .map_err(|e| anyhow::anyhow!("invalid OAUTH_RELAY_URL: {e}"))?,
        )
        .with_client_id(ClientId::new(settings.client_id.clone()))
        .with_client_secret(ClientSecret::new(settings.client_secret.clone()))
        .with_scopes(vec![
            Scope::new("openid".into()),
            Scope::new("email".into()),
            Scope::new("profile".into()),
        ])
        .with_state_generator(move || CsrfToken::new(relay_state(&callback)))
        .discover(issuer_url)
        .await
        .map_err(|e| anyhow::anyhow!("OIDC discovery failed: {e}"))?
        .build();

    Ok(client)
}
