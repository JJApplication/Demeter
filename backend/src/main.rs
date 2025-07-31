use axum::{
    http::{HeaderValue, Method, header::{CONTENT_TYPE, AUTHORIZATION}},
    routing::{get, post, put},
    Router,
};
use sqlx::sqlite::SqlitePool;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

mod models;
mod handlers;
mod database;
mod config;

use database::*;
use config::Config;

#[derive(Clone)]
pub struct AppState {
    db: SqlitePool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 加载配置文件
    let config = Config::load()?;
    
    // 初始化日志
    tracing_subscriber::fmt::init();

    // 初始化数据库
    let db = init_database(&config.database).await?;
    let state = AppState { db };

    // 配置CORS
    let cors = CorsLayer::new()
        .allow_origin(config.server.cors_origin.parse::<HeaderValue>()?)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([CONTENT_TYPE, AUTHORIZATION]);

    // 构建路由
    let app = Router::new()
        .route("/api/login", post(handlers::login_handler))
        .route("/api/register", post(handlers::register_handler))
        .route("/api/todos", get(handlers::get_todos_handler).post(handlers::create_todo_handler))
        .route("/api/todos/:id", put(handlers::update_todo_handler).delete(handlers::delete_todo_handler))
        .route("/api/history", get(handlers::get_history_handler))
        .route("/api/public-access", get(handlers::get_public_access_handler))
        .route("/api/user/settings", put(handlers::update_user_settings_handler))
        .layer(cors)
        .with_state(state);

    info!("服务器启动在 {}", config.server_url());

    // 启动服务器
    let listener = tokio::net::TcpListener::bind(&config.server_address()).await?;
    axum::serve(listener, app).await?;

    Ok(())
}