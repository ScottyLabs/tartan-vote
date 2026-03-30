use axum::{Router, middleware, routing::get};
use http::{HeaderValue, Method, header};
use migration::{Migrator, MigratorTrait};
use sea_orm::Database;
use tower_http::cors::CorsLayer;
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

    let bind_addr = app_state.config.bind_addr.clone();

    let allowed_origins = app_state
        .config
        .cors_allowed_origins
        .iter()
        .map(|origin| {
            origin
                .parse::<HeaderValue>()
                .expect("valid CORS_ALLOWED_ORIGINS entry")
        })
        .collect::<Vec<_>>();

    let cors_layer = CorsLayer::new()
        .allow_origin(allowed_origins)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION, header::ACCEPT])
        .allow_credentials(true);

    let api_router = Router::new()
        .route("/", get(crate::domain::auth::handlers::demo_home)) // demo only
        .route("/auth/login", get(crate::domain::auth::handlers::login))
        .route(
            "/auth/callback",
            get(crate::domain::auth::handlers::callback),
        )
        .route("/auth/logout", get(crate::domain::auth::handlers::logout))
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
        .route(
            "/events/{session_code}/check",
            get(crate::domain::event::handlers::check_event),
        )
        .fallback(get(crate::domain::auth::handlers::demo_not_found)) // demo only
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            crate::core::auth::middleware::sync_user_middleware,
        ))
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
