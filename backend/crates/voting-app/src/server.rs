use axum::{
    Router, error_handling::HandleErrorLayer, middleware, response::IntoResponse, routing::get,
};
use axum_oidc::{OidcAuthLayer, OidcLoginLayer, error::MiddlewareError, handle_oidc_redirect};
use migration::{Migrator, MigratorTrait};
use sea_orm::Database;
use tower::ServiceBuilder;
use tower_sessions::{
    Expiry, SessionManagerLayer,
    cookie::{SameSite, time::Duration},
};
use tower_sessions_sqlx_store::PostgresStore;
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_scalar::{Scalar, Servable};
use voting_app_store::Store;

use crate::core::auth::oidc::{self, GroupClaims, SessionWrapper};
use crate::core::openapi::ApiDoc;
use crate::{AppState, config::Config};

#[utoipa::path(get, path = "/api/health", tag = "health", responses((status = OK, body = str)))]
async fn health() -> &'static str {
    "OK"
}

pub async fn setup(config: Config) {
    let db = Database::connect(&config.database_url)
        .await
        .expect("failed to connect to database");

    Migrator::up(&db, None)
        .await
        .expect("failed to run database migrations");
    println!("Migration complete!");

    let store = Store::new(db.clone());
    let app_state = AppState { db, store, config };

    let bind_addr = app_state.config.bind_addr.clone();

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(crate::domain::auth::handlers::auth_status))
        .routes(routes!(crate::domain::auth::bypass::bypass_login))
        .routes(routes!(crate::domain::auth::bypass::bypass_status))
        .routes(routes!(crate::domain::auth::bypass::bypass_logout))
        .routes(routes!(health))
        .routes(routes!(crate::domain::event::handlers::create_event))
        .routes(routes!(crate::domain::event::handlers::check_event))
        .routes(routes!(crate::domain::event::handlers::end_event))
        .routes(routes!(crate::domain::attendance::handlers::attendance))
        .split_for_parts();

    let base_router = router
        .merge(Scalar::with_url("/api/scalar", api.clone()))
        .route(
            "/api/openapi.json",
            get(move || {
                let api = api.clone();
                async move { axum::Json(api) }
            }),
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
            "/events/{id}/vote-instances",
            get(crate::domain::votes::handlers::get_vote_instances),
        )
        .route(
            "/events/{id}/proxies",
            get(crate::domain::votes::handlers::list_proxy_assignments)
                .post(crate::domain::votes::handlers::assign_proxy),
        )
        .route(
            "/events/{id}/export",
            get(crate::domain::votes::handlers::export_event_results),
        )
        .route("/health", get(|| async { "OK" }))
        .route(
            "/session/create",
            get(crate::domain::session::handlers::create_session),
        )
        .route(
            "/session/join/{session_code}",
            get(crate::domain::session::handlers::join_session),
        )
        .route(
            "/session/{session_code}/proxy",
            axum::routing::post(crate::domain::session::handlers::set_session_proxy),
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
            "/session/{session_code}/export/{kind}/{format}",
            get(crate::domain::session::export::export_session_data),
        )
        .route(
            "/session/{session_code}/events/export",
            get(crate::domain::session::export::export_session_events_json),
        )
        .route("/", get(crate::domain::auth::handlers::demo_home))
        .fallback(get(crate::domain::auth::handlers::demo_not_found));

    // A server-side session store holds the OIDC token set (and is what `/auth/logout`
    // flushes). Backed by the existing Postgres connection so sessions survive backend
    // restarts/deploys and stay consistent across multiple instances. The session table
    // is created on startup via `migrate()`.
    let session_store = PostgresStore::new(app_state.db.get_postgres_connection_pool().clone());
    session_store
        .migrate()
        .await
        .expect("failed to run session store migrations");
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_same_site(SameSite::Lax)
        .with_expiry(Expiry::OnInactivity(Duration::hours(1)));

    let auth_routes: Router<AppState> = if app_state.config.oidc.is_some() {
        let login_layer = ServiceBuilder::new()
            .layer(HandleErrorLayer::new(|e: MiddlewareError| async move {
                tracing::error!("OIDC login error: {:?}", e);
                e.into_response()
            }))
            .layer(OidcLoginLayer::<GroupClaims, SessionWrapper>::new());

        Router::new()
            .route(
                "/auth/login",
                get(crate::domain::auth::handlers::login).layer(login_layer),
            )
            .route(
                "/auth/callback",
                get(handle_oidc_redirect::<GroupClaims, SessionWrapper>),
            )
            .route("/auth/logout", get(crate::domain::auth::handlers::logout))
    } else {
        Router::new()
            .route("/auth/login", get(crate::domain::auth::handlers::login))
            .route(
                "/auth/callback",
                get(crate::domain::auth::handlers::callback),
            )
            .route("/auth/logout", get(crate::domain::auth::handlers::logout))
    };

    let mut api_router = base_router.merge(auth_routes);

    if let Some(oidc_settings) = app_state.config.oidc.clone() {
        let client = oidc::build_client(&oidc_settings)
            .await
            .expect("failed to build OIDC client (Keycloak discovery)");
        tracing::info!("OIDC discovery completed");

        let oidc_auth_layer = ServiceBuilder::new()
            .layer(HandleErrorLayer::new(|e: MiddlewareError| async move {
                tracing::error!("OIDC auth error: {:?}", e);
                e.into_response()
            }))
            .layer(OidcAuthLayer::<GroupClaims, SessionWrapper>::new(client));

        api_router = api_router
            .layer(middleware::from_fn_with_state(
                app_state.clone(),
                crate::core::auth::middleware::sync_user_middleware,
            ))
            .layer(oidc_auth_layer);
    }

    let api_router = api_router
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            crate::domain::auth::bypass::bypass_auth_middleware,
        ))
        .layer(session_layer)
        .layer(crate::core::cors::layer())
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(&bind_addr)
        .await
        .expect("failed to bind to server address");
    println!("Listening on {}", bind_addr);

    axum::serve(listener, api_router.into_make_service())
        .await
        .expect("failed to start server");
}
