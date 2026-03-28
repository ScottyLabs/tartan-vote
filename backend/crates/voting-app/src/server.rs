use axum::{Router, middleware, response::IntoResponse, routing::get};
use axum_oidc::{EmptyAdditionalClaims, OidcAuthLayer, OidcLoginLayer, error::MiddlewareError};
use http::{HeaderValue, Method, Uri, header};
use migration::{Migrator, MigratorTrait};
use sea_orm::Database;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_sessions::{
    Expiry, SessionManagerLayer,
    cookie::{SameSite, time::Duration},
};
use voting_app_store::Store;

use crate::{AppState, config::Config};

pub async fn setup() {
    let config = Config::from_env().expect("failed to load configuration");

    let db = Database::connect(&config.database_url)
        .await
        .expect("failed to connect to database");

    Migrator::up(&db, None)
        .await
        .expect("failed to run database migrations");
    println!("Migration complete!");

    let store = Store::new(db.clone());
    let app_state = AppState { db, store, config };

    let session_layer = SessionManagerLayer::new(tower_sessions::MemoryStore::default())
        .with_secure(false)
        .with_same_site(SameSite::Lax)
        .with_expiry(Expiry::OnInactivity(Duration::hours(24)));

    let oidc_auth_layer = OidcAuthLayer::<EmptyAdditionalClaims>::discover_client(
        Uri::try_from(app_state.config.app_base_url.clone()).expect("valid APP_BASE_URL"),
        app_state.config.oidc_issuer.clone(),
        app_state.config.oidc_client_id.clone(),
        Some(app_state.config.oidc_client_secret.clone()),
        vec![
            "openid".to_string(),
            "email".to_string(),
            "profile".to_string(),
        ],
    )
    .await
    .expect("failed to discover OIDC provider");

    let oidc_login_service = ServiceBuilder::new()
        .layer(axum::error_handling::HandleErrorLayer::new(
            |err: MiddlewareError| async move {
                tracing::error!("OIDC login error: {:?}", err);
                err.into_response()
            },
        ))
        .layer(OidcLoginLayer::<EmptyAdditionalClaims>::new());

    let oidc_auth_service = ServiceBuilder::new()
        .layer(axum::error_handling::HandleErrorLayer::new(
            |err: MiddlewareError| async move {
                tracing::error!("OIDC auth error: {:?}", err);
                err.into_response()
            },
        ))
        .layer(oidc_auth_layer);

    let bind_addr = app_state.config.bind_addr.clone();

    let cors_layer = CorsLayer::new()
        .allow_origin(
            app_state
                .config
                .frontend_base_url
                .parse::<HeaderValue>()
                .expect("valid frontend URL"),
        )
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION, header::ACCEPT])
        .allow_credentials(true);

    let protected_auth_router = Router::new()
        .route(
            "/auth/callback",
            get(crate::domain::auth::handlers::callback),
        )
        .route("/auth/logout", get(crate::domain::auth::handlers::logout))
        .layer(oidc_login_service.clone());

    let api_router = Router::new()
        .route("/", get(crate::domain::auth::handlers::demo_home)) // demo only
        .route("/auth/login", get(crate::domain::auth::handlers::login))
        .merge(protected_auth_router)
        .route(
            "/auth/status",
            get(crate::domain::auth::handlers::auth_status),
        )
        .route(
            "/events/{id}/vote",
            axum::routing::post(crate::domain::votes::handlers::cast_vote),
        )
        .route(
            "/events/{id}/results",
            get(crate::domain::votes::handlers::get_motion_results),
        )
        .route(
            "/events/{id}/end",
            axum::routing::get(crate::domain::event::handlers::end_event),
        )
        .route(
            "/events/create/{session_code}",
            axum::routing::post(crate::domain::event::handlers::create_event),
        )
        .route("/health", get(|| async { "OK" }))
        .route(
            "/session/{session_code}/attendance",
            get(crate::domain::attendance::handlers::attendance),
        )
        .route(
            "/session/create",
            get(crate::domain::session::handlers::create_session),
        )
        .route(
            "/session/join/{session_code}",
            get(crate::domain::session::handlers::join_session),
        )
        .route(
            "/session/{session_code}/end",
            axum::routing::get(crate::domain::session::handlers::end_session),
        )
        .route(
            "/session/{session_code}/status",
            get(crate::domain::session::handlers::status_session),
        )
        .route(
            "/events/{session_code}/check",
            get(crate::domain::event::handlers::check_event),
        )
        .fallback(get(crate::domain::auth::handlers::demo_not_found)) // demo only
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            crate::core::auth::middleware::sync_user_middleware,
        ))
        .layer(oidc_auth_service)
        .layer(session_layer)
        .layer(cors_layer)
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(&bind_addr)
        .await
        .expect("failed to bind to server address");
    println!("Listening on {}", &bind_addr);

    axum::serve(listener, api_router.into_make_service())
        .await
        .expect("failed to start server");
}
