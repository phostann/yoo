use axum::{
    extract::DefaultBodyLimit,
    routing::{delete, get, post},
    Router, Server,
};
use sea_orm::Database;
use std::{env, net::SocketAddr, str::FromStr};
use tower_http::cors::CorsLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::handler;
use crate::handler::AppState;

#[tokio::main]
async fn start() -> anyhow::Result<()> {
    env::set_var("RUST_LOG", "debug");
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("error")),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenvy::dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let host = env::var("HOST").expect("HOST must be set");
    let port = env::var("PORT").expect("PORT must be set");
    let server_url = format!("{host}:{port}");

    let conn = Database::connect(db_url)
        .await
        .expect("Failed to connect to database");

    let state = AppState { conn };

    
    let app = Router::new()
        .route("/register", post(handler::register))
        .route("/login", post(handler::login))
        .route("/profile", get(handler::profile))
        .route("/refresh", post(handler::refresh))
        .route("/group", post(handler::create_group))
        .route(
            "/group/:id",
            get(handler::get_group_by_id)
                .patch(handler::update_group_by_id)
                .delete(handler::delete_group_by_id),
        )
        .route("/groups", get(handler::list_groups))
        .route("/config", post(handler::create_config))
        .route(
            "/config/:id",
            get(handler::get_config_by_id)
                .patch(handler::update_config_by_id)
                .delete(handler::delete_config_by_id),
        )
        .route("/configs", get(handler::list_configs))
        .route("/template", post(handler::create_template))
        .route(
            "/template/:id",
            get(handler::get_template_by_id)
                .patch(handler::update_template_by_id)
                .delete(handler::delete_template_by_id),
        )
        .route("/templates", get(handler::list_templates))
        .route("/template/tags", get(handler::get_template_tags))
        .route("/project", post(handler::create_project))
        .route("/projects", get(handler::list_projects))
        .route(
            "/project/:id",
            get(handler::get_project_by_id)
                .patch(handler::update_project_by_id)
                .delete(handler::delete_project_by_id),
        )
        .route(
            "/project/name/:name",
            delete(handler::delete_project_by_name),
        )
        .route("/upload", post(handler::upload))
        .layer(CorsLayer::permissive())
        // max body size is 5MB
        .layer(DefaultBodyLimit::max(1024 * 1024 * 5))
        .with_state(state);

    let addr = SocketAddr::from_str(&server_url).expect("Invalid server address");
    tracing::debug!("Listening on {}", addr);

    Server::bind(&addr).serve(app.into_make_service()).await?;

    Ok(())
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        eprintln!("Error: {err}");
    }
}
