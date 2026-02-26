// Server binary: only compiled when feature "server" is enabled.
// When building for wasm32 with feature "client", we still need a binary target;
// this stub satisfies the linker (the real app runs from the lib).
#![cfg(any(feature = "server", all(feature = "client", target_arch = "wasm32")))]

#[cfg(all(feature = "client", target_arch = "wasm32"))]
fn main() {}

#[cfg(feature = "server")]
mod config;
#[cfg(feature = "server")]
mod crypto;
#[cfg(feature = "server")]
mod middleware;
#[cfg(feature = "server")]
mod models;
#[cfg(feature = "server")]
mod modules;
#[cfg(feature = "server")]
mod state;
#[cfg(feature = "server")]
mod ui;

#[cfg(feature = "server")]
use std::net::SocketAddr;

#[cfg(feature = "server")]
use anyhow::Context;
#[cfg(feature = "server")]
use axum::{
    extract::{Path, Query},
    Json, Router,
    middleware as axum_middleware,
    response::Redirect,
    routing::{get, patch, post},
};
#[cfg(feature = "server")]
use models::{ApiResponse, HealthResponse};
#[cfg(feature = "server")]
use sqlx::PgPool;
#[cfg(feature = "server")]
use state::AppState;
#[cfg(feature = "server")]
use tower_http::{cors::CorsLayer, services::ServeDir};
#[cfg(feature = "server")]
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

#[cfg(feature = "server")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cfg = config::AppConfig::from_env();
    let db = PgPool::connect(&cfg.database_url)
        .await
        .context("failed to connect to postgres")?;

    sqlx::migrate!("./migrations")
        .run(&db)
        .await
        .context("failed to run migrations")?;

    let redis = redis::Client::open(cfg.redis_url.clone()).context("invalid REDIS_URL")?;
    let state = AppState {
        db,
        redis,
        secret_key: cfg.secret_key,
    };

    // Routes that require a valid session (x-session-id header).
    let protected = Router::new()
        .route("/api/users/me/sessions", get(modules::users::list_my_sessions))
        // Credentials
        .route(
            "/api/credentials",
            get(modules::credentials::list_credentials)
                .post(modules::credentials::create_credential),
        )
        .route(
            "/api/credentials/{id}",
            patch(modules::credentials::update_credential),
        )
        .route(
            "/api/credentials/{id}/disable",
            post(modules::credentials::disable_credential),
        )
        .route(
            "/api/credentials/{id}/rotate",
            post(modules::credentials::rotate_credential),
        )
        // Orchestrator
        .route("/api/orchestrator/runs", get(modules::orchestrator::list_runs))
        // Ingestion / Sources
        .route(
            "/api/sources",
            get(modules::ingestion::list_sources).post(modules::ingestion::create_source),
        )
        .route(
            "/api/admin/ingest/jobs",
            get(modules::ingestion::list_ingest_jobs),
        )
        .route(
            "/internal/ingest/run/{source_id}",
            post(modules::ingestion::trigger_ingest_job),
        )
        .layer(axum_middleware::from_fn_with_state(
            state.clone(),
            middleware::require_auth,
        ));

    let app = Router::new()
        // Public web + auth
        .route("/", get(|| async { Redirect::permanent("/app/") }))
        .route("/dashboard", get(ui::pages::dashboard))
        .route("/app", get(|| async { Redirect::permanent("/app/") }))
        .route("/app/topic/{id}", get(app_topic_redirect))
        .nest_service("/app/", ServeDir::new("dist"))
        .route("/health", get(health))
        .route("/api/auth/register", post(modules::auth::register))
        .route("/api/auth/login", post(modules::auth::login))
        .route("/api/auth/logout", post(modules::auth::logout))
        .route("/api/auth/refresh", post(modules::auth::refresh))
        .route("/api/localities", get(modules::locality::list_localities))
        .route("/api/topics", get(modules::topics::list_topics).post(modules::topics::create_topic))
        .route(
            "/api/topics/{id}",
            get(modules::topics::get_topic)
                .patch(modules::topics::update_topic)
                .delete(modules::topics::delete_topic),
        )
        .route("/api/locations", get(modules::locations::list_locations))
        .route("/api/locations/{id}", get(modules::locations::get_location))
        .route("/api/map/alignment/{topic_id}", get(modules::maps::get_alignment))
        // Protected routes
        .merge(protected)
        // Static assets
        .nest_service("/public", ServeDir::new("public"))
        // Global IP rate limiting (fails open on Redis errors)
        .layer(axum_middleware::from_fn_with_state(
            state.clone(),
            middleware::rate_limit_ip,
        ))
        .layer(CorsLayer::permissive())
        .layer(axum_middleware::from_fn(middleware::request_log))
        .with_state(state);

    let addr: SocketAddr = format!("{}:{}", cfg.host, cfg.port).parse()?;
    tracing::info!("politech listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

#[cfg(feature = "server")]
async fn health() -> Json<ApiResponse<HealthResponse>> {
    Json(ApiResponse {
        ok: true,
        data: HealthResponse {
            status: "ok",
            service: "politech",
        },
    })
}

#[cfg(feature = "server")]
async fn app_topic_redirect(
    Path(id): Path<String>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Redirect {
    let edit_suffix = if params.get("edit").is_some_and(|v| v == "1") {
        "&edit=1"
    } else {
        ""
    };
    Redirect::temporary(&format!("/app/?topic={id}{edit_suffix}"))
}
