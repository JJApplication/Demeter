use std::env;
use bcrypt;
use sqlx::{sqlite::SqlitePool, Row};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 || args.len() > 5 {
        eprintln!("使用方法: {} <用户名> <密码> [--public] [--readonly]", args[0]);
        eprintln!("示例: {} john mypassword", args[0]);
        eprintln!("示例: {} john mypassword --public", args[0]);
        eprintln!("示例: {} john mypassword --readonly", args[0]);
        eprintln!("示例: {} john mypassword --public --readonly", args[0]);
        std::process::exit(1);
    }
    
    let username = &args[1];
    let password = &args[2];
    let mut public_access = false;
    let mut readonly = false;
    
    // 解析可选标志
    for arg in &args[3..] {
        match arg.as_str() {
            "--public" => public_access = true,
            "--readonly" => readonly = true,
            _ => {
                eprintln!("错误: 未知标志 '{}'", arg);
                std::process::exit(1);
            }
        }
    }
    
    // 验证输入
    if username.is_empty() || password.is_empty() {
        eprintln!("错误: 用户名和密码不能为空");
        std::process::exit(1);
    }
    
    if username.len() < 3 {
        eprintln!("错误: 用户名至少需要3个字符");
        std::process::exit(1);
    }
    
    if password.len() < 6 {
        eprintln!("错误: 密码至少需要6个字符");
        std::process::exit(1);
    }
    
    // 连接数据库
    let database_url = "sqlite:./todolist.db";
    let pool = SqlitePool::connect(database_url).await?;
    
    // 检查用户是否已存在
    let existing_user = sqlx::query(
        "SELECT COUNT(*) as count FROM users WHERE username = ?"
    )
    .bind(username)
    .fetch_one(&pool)
    .await?;
    
    let count: i64 = existing_user.get("count");
    if count > 0 {
        eprintln!("错误: 用户名 '{}' 已存在", username);
        std::process::exit(1);
    }
    
    // 哈希密码
    let password_hash = bcrypt::hash(password, bcrypt::DEFAULT_COST)
        .map_err(|e| anyhow::anyhow!("密码哈希失败: {}", e))?;
    
    // 创建用户
    sqlx::query(
        "INSERT INTO users (username, password_hash, public_access, readonly) VALUES (?, ?, ?, ?)"
    )
    .bind(username)
    .bind(&password_hash)
    .bind(public_access)
    .bind(readonly)
    .execute(&pool)
    .await?;
    
    println!("✅ 用户 '{}' 创建成功！", username);
    if public_access {
        println!("🌐 设置: 任务列表公开访问");
    } else {
        println!("🔒 设置: 任务列表不公开访问");
        println!("🔧 可以在前端历史任务页面修改公开访问设置");
    }
    if readonly {
        println!("📖 设置: 只读用户 (无法创建、修改或删除任务)");
    } else {
        println!("✏️ 设置: 普通用户 (可以创建、修改和删除任务)");
    }
    
    Ok(())
}