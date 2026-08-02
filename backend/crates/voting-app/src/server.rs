use axum::{
    Router, error_handling::HandleErrorLayer, http::StatusCode, middleware, response::IntoResponse,
    routing::get,
};
use axum_oidc::{OidcAuthLayer, OidcLoginLayer, error::MiddlewareError, handle_oidc_redirect};
use fred::prelude::{ClientLike, Config as RedisConfig, Pool};
use migration::{Migrator, MigratorTrait};
use sea_orm::Database;
use tower::ServiceBuilder;
use tower_http::services::{ServeDir, ServeFile};
use tower_sessions::{
    Expiry, SessionManagerLayer,
    cookie::{SameSite, time::Duration},
};
use tower_sessions_redis_store::RedisStore;
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_scalar::{Scalar, Servable};
use voting_app_store::Store;

use crate::core::auth::oidc::{self, GroupClaims, SessionWrapper};
use crate::core::openapi::ApiDoc;
use crate::{AppState, config::Config};

#[utoipa::path(get, path = "/health", tag = "health", responses((status = OK, body = str)))]
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
    let static_dir = app_state.config.static_dir.clone();

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest(
            "/api",
            OpenApiRouter::new()
                .routes(routes!(crate::domain::auth::handlers::auth_status))
                .routes(routes!(health))
                .routes(routes!(crate::domain::motion::handlers::create_motion))
                .routes(routes!(crate::domain::motion::handlers::check_motion))
                .routes(routes!(crate::domain::motion::handlers::end_motion))
                .routes(routes!(crate::domain::attendance::handlers::attendance))
                .routes(routes!(crate::domain::votes::handlers::cast_vote))
                .routes(routes!(crate::domain::votes::handlers::get_motion_results))
                .routes(routes!(crate::domain::votes::handlers::get_vote_instances))
                .routes(routes!(
                    crate::domain::votes::handlers::list_proxy_assignments
                ))
                .routes(routes!(crate::domain::votes::handlers::assign_proxy))
                .routes(routes!(
                    crate::domain::votes::handlers::export_motion_results
                ))
                .routes(routes!(crate::domain::session::handlers::create_session))
                .routes(routes!(crate::domain::session::handlers::join_session))
                .routes(routes!(crate::domain::session::handlers::set_session_proxy))
                .routes(routes!(crate::domain::session::handlers::end_session))
                .routes(routes!(crate::domain::session::handlers::lock_session))
                .routes(routes!(crate::domain::session::handlers::open_session))
                .routes(routes!(crate::domain::session::handlers::status_session))
                .routes(routes!(crate::domain::session::export::export_session_data))
                .routes(routes!(
                    crate::domain::session::export::export_session_motions_json
                )),
        )
        .split_for_parts();

    let login_layer = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|e: MiddlewareError| async move {
            tracing::error!("OIDC login error: {:?}", e);
            e.into_response()
        }))
        .layer(OidcLoginLayer::<GroupClaims, SessionWrapper>::new());

    let auth_routes: Router<AppState> = Router::new()
        .route(
            "/auth/login",
            get(crate::domain::auth::handlers::login).layer(login_layer),
        )
        .route(
            "/auth/callback",
            get(handle_oidc_redirect::<GroupClaims, SessionWrapper>),
        )
        .route("/auth/logout", get(crate::domain::auth::handlers::logout));

    let api_routes: Router<AppState> = Router::new()
        .merge(Scalar::with_url("/scalar", api.clone()))
        .route(
            "/openapi.json",
            get(move || {
                let api = api.clone();
                async move { axum::Json(api) }
            }),
        )
        .merge(auth_routes);

    let pool = Pool::new(
        RedisConfig::from_url(&app_state.config.valkey_url).expect("valid VALKEY_URL"),
        None,
        None,
        None,
        6,
    )
    .expect("failed to build valkey pool");
    pool.connect();
    pool.wait_for_connect()
        .await
        .expect("failed to connect to valkey");
    let session_store = RedisStore::new(pool);
    let secure_cookies = app_state.config.oidc.app_url.starts_with("https://");
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(secure_cookies)
        .with_same_site(SameSite::Lax)
        .with_expiry(Expiry::OnInactivity(Duration::hours(1)));

    let client = oidc::build_client(&app_state.config.oidc)
        .await
        .expect("failed to build OIDC client (Keycloak discovery)");
    tracing::info!("OIDC discovery completed");

    let oidc_auth_layer = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|e: MiddlewareError| async move {
            tracing::error!("OIDC auth error: {:?}", e);
            e.into_response()
        }))
        .layer(OidcAuthLayer::<GroupClaims, SessionWrapper>::new(client));

    let api_router = router
        .nest("/api", api_routes)
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            crate::core::auth::middleware::sync_user_middleware,
        ))
        .layer(oidc_auth_layer)
        .layer(session_layer)
        .layer(crate::core::cors::layer())
        .with_state(app_state);

    let app = match static_dir {
        Some(dir) => {
            let index = ServeFile::new(dir.join("index.html"));
            let serve = ServeDir::new(dir).not_found_service(index);
            Router::new().merge(api_router).fallback_service(serve)
        }
        None => Router::new()
            .merge(api_router)
            .fallback(|| async { StatusCode::NOT_FOUND }),
    };

    let listener = tokio::net::TcpListener::bind(&bind_addr)
        .await
        .expect("failed to bind to server address");
    println!("Listening on {}", bind_addr);

    axum::serve(listener, app.into_make_service())
        .await
        .expect("failed to start server");
}
