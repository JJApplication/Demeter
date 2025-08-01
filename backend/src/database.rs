use sqlx::{sqlite::{SqlitePool, SqliteConnectOptions}, Row};
use std::str::FromStr;
use tracing::info;
use crate::config::DatabaseConfig;

pub async fn init_database(config: &DatabaseConfig) -> anyhow::Result<SqlitePool> {
    // 创建数据库连接选项，根据配置决定是否自动创建数据库文件
    let options = SqliteConnectOptions::from_str(&config.url)?
        .create_if_missing(config.create_if_missing);
    
    // 创建数据库连接池
    let pool = SqlitePool::connect_with(options).await?;

    // 运行数据库迁移
    run_migrations(&pool).await?;

    info!("数据库初始化完成");
    Ok(pool)
}

pub async fn run_migrations(pool: &SqlitePool) -> anyhow::Result<()> {
    // 创建用户表
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL,
            public_access BOOLEAN NOT NULL DEFAULT FALSE,
            readonly BOOLEAN NOT NULL DEFAULT FALSE,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await?;

    // 为现有用户表添加readonly字段（如果不存在）
    sqlx::query(
        "ALTER TABLE users ADD COLUMN readonly BOOLEAN NOT NULL DEFAULT FALSE"
    )
    .execute(pool)
    .await
    .ok(); // 忽略错误，因为字段可能已存在

    // 创建任务表
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            title TEXT NOT NULL,
            description TEXT,
            emoji TEXT NOT NULL DEFAULT '📝',
            completed BOOLEAN NOT NULL DEFAULT FALSE,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
        )
        "#,
    )
    .execute(pool)
    .await?;

    // 创建索引
    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_todos_user_id ON todos(user_id);
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_todos_created_at ON todos(created_at);
        "#,
    )
    .execute(pool)
    .await?;

    // 插入默认用户（如果不存在）
    let user_exists = sqlx::query("SELECT COUNT(*) as count FROM users WHERE username = ?")
        .bind("guest")
        .fetch_one(pool)
        .await?
        .get::<i64, _>("count") > 0;

    if !user_exists {
        let password_hash = bcrypt::hash("password", bcrypt::DEFAULT_COST)?;
        sqlx::query(
            "INSERT INTO users (username, password_hash, public_access, readonly) VALUES (?, ?, ?, ?)"
        )
        .bind("guest")
        .bind(password_hash)
        .bind(false)
        .bind(false) // 默认用户不是只读
        .execute(pool)
        .await?;
        
        info!("创建默认用户: guest/password");
    }

    Ok(())
}