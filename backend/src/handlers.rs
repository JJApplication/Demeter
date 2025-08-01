use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::Json,
};
use chrono::Utc;
use sqlx::Row;
use tracing::{error, info};
use sha2::{Sha256, Digest};
use base64::{Engine as _, engine::general_purpose};

use crate::{
    models::*,
    AppState,
};

// Token生成函数
fn generate_token(username: &str, password_hash: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(username.as_bytes());
    hasher.update(password_hash.as_bytes());
    hasher.update(b"demeter_secret_key"); // 添加密钥增强安全性
    let result = hasher.finalize();
    general_purpose::STANDARD.encode(result)
}

// Token验证函数
fn verify_token(token: &str, username: &str, password_hash: &str) -> bool {
    let expected_token = generate_token(username, password_hash);
    token == expected_token
}

// 从请求头中提取用户ID
async fn extract_user_id(headers: &HeaderMap, db: &sqlx::SqlitePool) -> Result<i64, StatusCode> {
    let auth_header = headers.get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // 查询所有用户，验证token
    let users_result = sqlx::query_as::<_, User>(
        "SELECT id, username, password_hash, public_access, readonly, created_at FROM users"
    )
    .fetch_all(db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    for user in users_result {
        if verify_token(auth_header, &user.username, &user.password_hash) {
            return Ok(user.id);
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}

// 检查用户是否为只读用户
async fn check_readonly_permission(headers: &HeaderMap, db: &sqlx::SqlitePool) -> Result<(), StatusCode> {
    let auth_header = headers.get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // 查询所有用户，验证token并检查readonly状态
    let users_result = sqlx::query_as::<_, User>(
        "SELECT id, username, password_hash, public_access, readonly, created_at FROM users"
    )
    .fetch_all(db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    for user in users_result {
        if verify_token(auth_header, &user.username, &user.password_hash) {
            if user.readonly {
                return Err(StatusCode::FORBIDDEN);
            }
            return Ok(());
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}

// 获取公开访问状态
pub async fn get_public_access_handler(
    State(state): State<AppState>,
) -> Result<Json<PublicAccessResponse>, StatusCode> {
    // 简化版本：获取第一个用户的公开访问设置
    let user_result = sqlx::query_as::<_, User>(
        "SELECT id, username, password_hash, public_access, readonly, created_at FROM users LIMIT 1"
    )
    .fetch_optional(&state.db)
    .await;

    match user_result {
        Ok(Some(user)) => {
            let response = PublicAccessResponse {
                public_access: user.public_access,
                username: user.username,
            };
            Ok(Json(response))
        }
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            error!("获取公开访问状态失败: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// 更新用户设置
pub async fn update_user_settings_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<UpdateUserSettingsRequest>,
) -> Result<Json<ApiResponse<UserResponse>>, StatusCode> {
    // 检查只读权限
    check_readonly_permission(&headers, &state.db).await?;
    
    let user_id = extract_user_id(&headers, &state.db).await?;
    
    let result = sqlx::query(
        "UPDATE users SET public_access = ? WHERE id = ?"
    )
    .bind(payload.public_access)
    .bind(user_id)
    .execute(&state.db)
    .await;

    match result {
        Ok(_) => {
            // 获取更新后的用户信息
            let user_result = sqlx::query_as::<_, User>(
                "SELECT id, username, password_hash, public_access, readonly, created_at FROM users WHERE id = ?"
            )
            .bind(user_id)
            .fetch_optional(&state.db)
            .await;

            match user_result {
                Ok(Some(user)) => {
                    info!("用户设置已更新: 公开访问 = {}", payload.public_access);
                    Ok(Json(ApiResponse::success(user.into())))
                }
                Ok(None) => Err(StatusCode::NOT_FOUND),
                Err(e) => {
                    error!("获取用户信息失败: {}", e);
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        }
        Err(e) => {
            error!("更新用户设置失败: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// 用户登录
pub async fn login_handler(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    let user_result = sqlx::query_as::<_, User>(
        "SELECT id, username, password_hash, public_access, readonly, created_at FROM users WHERE username = ?"
    )
    .bind(&payload.username)
    .fetch_optional(&state.db)
    .await;

    match user_result {
        Ok(Some(user)) => {
            if bcrypt::verify(&payload.password, &user.password_hash).unwrap_or(false) {
                let token = generate_token(&user.username, &user.password_hash);
                info!("用户 {} 登录成功", payload.username);
                Ok(Json(LoginResponse {
                    user: user.into(),
                    token,
                    message: "登录成功".to_string(),
                }))
            } else {
                error!("用户 {} 密码错误", payload.username);
                Err(StatusCode::UNAUTHORIZED)
            }
        }
        Ok(None) => {
            error!("用户 {} 不存在", payload.username);
            Err(StatusCode::UNAUTHORIZED)
        }
        Err(e) => {
            error!("数据库查询错误: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// 用户注册
pub async fn register_handler(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<ApiResponse<UserResponse>>, StatusCode> {
    // 检查用户名是否已存在
    let existing_user = sqlx::query("SELECT id FROM users WHERE username = ?")
        .bind(&payload.username)
        .fetch_optional(&state.db)
        .await;

    match existing_user {
        Ok(Some(_)) => {
            return Ok(Json(ApiResponse::error("用户名已存在")));
        }
        Ok(None) => {}
        Err(e) => {
            error!("数据库查询错误: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    // 创建新用户
    let password_hash = match bcrypt::hash(&payload.password, bcrypt::DEFAULT_COST) {
        Ok(hash) => hash,
        Err(e) => {
            error!("密码哈希错误: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let result = sqlx::query(
        "INSERT INTO users (username, password_hash, public_access, readonly) VALUES (?, ?, ?, ?) RETURNING id, username, created_at"
    )
    .bind(&payload.username)
    .bind(&password_hash)
    .bind(false) // 新注册用户默认不公开
    .bind(false) // 新注册用户默认不是只读
    .fetch_one(&state.db)
    .await;

    match result {
        Ok(row) => {
            let user = UserResponse {
                id: row.get("id"),
                username: row.get("username"),
                public_access: row.get("public_access"),
                readonly: row.get("readonly"),
            };
            info!("新用户注册: {}", payload.username);
            Ok(Json(ApiResponse::success(user)))
        }
        Err(e) => {
            error!("创建用户失败: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// 获取任务列表
pub async fn get_todos_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Vec<TodoResponse>>, StatusCode> {
    // 首先尝试从token获取用户ID
    let user_id_from_token = extract_user_id(&headers, &state.db).await.ok();
    
    let user_id = if let Some(uid) = user_id_from_token {
        // 用户已登录，返回其任务
        uid
    } else {
        // 用户未登录，检查是否有公开访问的用户
        let user_result = sqlx::query_as::<_, User>(
            "SELECT id, username, password_hash, public_access, readonly, created_at FROM users WHERE public_access = true LIMIT 1"
        )
        .fetch_optional(&state.db)
        .await;

        match user_result {
            Ok(Some(user)) => user.id,
            Ok(None) => {
                // 没有公开访问的用户，返回空列表
                return Ok(Json(Vec::new()));
            }
            Err(e) => {
                error!("查询公开访问用户失败: {}", e);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        }
    };

    // 获取该用户的任务
    let todos_result = sqlx::query_as::<_, Todo>(
        "SELECT id, user_id, title, description, emoji, completed, created_at, updated_at FROM todos WHERE user_id = ? ORDER BY created_at DESC"
    )
    .bind(user_id)
    .fetch_all(&state.db)
    .await;

    match todos_result {
        Ok(todos) => {
            let todo_responses: Vec<TodoResponse> = todos.into_iter().map(|todo| todo.into()).collect();
            Ok(Json(todo_responses))
        }
        Err(e) => {
            error!("获取任务列表失败: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// 创建新任务
pub async fn create_todo_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<CreateTodoRequest>,
) -> Result<Json<TodoResponse>, StatusCode> {
    // 检查只读权限
    check_readonly_permission(&headers, &state.db).await?;
    
    let user_id = extract_user_id(&headers, &state.db).await?;
    let now = Utc::now();
    
    let result = sqlx::query(
        "INSERT INTO todos (user_id, title, description, emoji, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?) RETURNING id"
    )
    .bind(user_id)
    .bind(&payload.title)
    .bind(&payload.description)
    .bind(&payload.emoji)
    .bind(now)
    .bind(now)
    .fetch_one(&state.db)
    .await;

    match result {
        Ok(row) => {
            let todo_id: i64 = row.get("id");
            let todo = TodoResponse {
                id: todo_id,
                title: payload.title,
                description: payload.description,
                emoji: payload.emoji,
                completed: false,
                created_at: now,
                updated_at: now,
            };
            info!("创建新任务: {}", todo.title);
            Ok(Json(todo))
        }
        Err(e) => {
            error!("创建任务失败: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// 更新任务
pub async fn update_todo_handler(
    Path(id): Path<i64>,
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<UpdateTodoRequest>,
) -> Result<Json<TodoResponse>, StatusCode> {
    // 检查只读权限
    check_readonly_permission(&headers, &state.db).await?;
    
    let user_id = extract_user_id(&headers, &state.db).await?;
    let now = Utc::now();
    
    // 获取当前任务
    let current_todo = sqlx::query_as::<_, Todo>(
        "SELECT id, user_id, title, description, emoji, completed, created_at, updated_at FROM todos WHERE id = ?"
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await;

    let mut todo = match current_todo {
        Ok(Some(todo)) => todo,
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Err(e) => {
            error!("查询任务失败: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // 更新字段
    if let Some(title) = payload.title {
        todo.title = title;
    }
    if let Some(description) = payload.description {
        todo.description = Some(description);
    }
    if let Some(emoji) = payload.emoji {
        todo.emoji = emoji;
    }
    if let Some(completed) = payload.completed {
        todo.completed = completed;
    }
    todo.updated_at = now;

    // 保存更新
    let result = sqlx::query(
        "UPDATE todos SET title = ?, description = ?, emoji = ?, completed = ?, updated_at = ? WHERE id = ?"
    )
    .bind(&todo.title)
    .bind(&todo.description)
    .bind(&todo.emoji)
    .bind(todo.completed)
    .bind(now)
    .bind(id)
    .execute(&state.db)
    .await;

    match result {
        Ok(_) => {
            info!("更新任务: {} (ID: {})", todo.title, id);
            Ok(Json(todo.into()))
        }
        Err(e) => {
            error!("更新任务失败: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// 删除任务
pub async fn delete_todo_handler(
    Path(id): Path<i64>,
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<StatusCode, StatusCode> {
    // 检查只读权限
    check_readonly_permission(&headers, &state.db).await?;
    
    let user_id = extract_user_id(&headers, &state.db).await?;
    
    let result = sqlx::query("DELETE FROM todos WHERE id = ? AND user_id = ?")
        .bind(id)
        .bind(user_id)
        .execute(&state.db)
        .await;

    match result {
        Ok(result) => {
            if result.rows_affected() > 0 {
                info!("删除任务 ID: {}", id);
                Ok(StatusCode::NO_CONTENT)
            } else {
                Err(StatusCode::NOT_FOUND)
            }
        }
        Err(e) => {
            error!("删除任务失败: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// 获取历史数据
pub async fn get_history_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Vec<HistoryDay>>, StatusCode> {
    // 首先尝试从token获取用户ID
    let user_id_from_token = extract_user_id(&headers, &state.db).await.ok();
    
    let user_id = if let Some(uid) = user_id_from_token {
        // 用户已登录，返回其历史任务
        uid
    } else {
        // 用户未登录，检查是否有公开访问的用户
        let user_result = sqlx::query_as::<_, User>(
            "SELECT id, username, password_hash, public_access, readonly, created_at FROM users WHERE public_access = true LIMIT 1"
        )
        .fetch_optional(&state.db)
        .await;

        match user_result {
            Ok(Some(user)) => user.id,
            Ok(None) => {
                // 没有公开访问的用户，返回空列表
                return Ok(Json(Vec::new()));
            }
            Err(e) => {
                error!("查询公开访问用户失败: {}", e);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        }
    };

    // 获取过去365天的数据，限制为指定用户
    let result = sqlx::query(
        r#"
        SELECT 
            DATE(created_at) as date,
            COUNT(*) as count,
            SUM(CASE WHEN completed = 1 THEN 1 ELSE 0 END) as completed_count
        FROM todos 
        WHERE user_id = ? AND created_at >= DATE('now', '-365 days')
        GROUP BY DATE(created_at)
        ORDER BY date
        "#
    )
    .bind(user_id)
    .fetch_all(&state.db)
    .await;

    match result {
        Ok(rows) => {
            let mut history_days = Vec::new();
            
            for row in rows {
                let date: String = row.get("date");
                let count: i64 = row.get("count");
                let completed_count: i64 = row.get("completed_count");
                
                // 获取该日期的所有任务
                let tasks_result = sqlx::query_as::<_, Todo>(
                    "SELECT id, user_id, title, description, emoji, completed, created_at, updated_at FROM todos WHERE user_id = ? AND DATE(created_at) = ? ORDER BY created_at"
                )
                .bind(user_id)
                .bind(&date)
                .fetch_all(&state.db)
                .await;
                
                let tasks = match tasks_result {
                    Ok(todos) => todos.into_iter().map(|todo| todo.into()).collect(),
                    Err(_) => Vec::new(),
                };
                
                history_days.push(HistoryDay {
                    date,
                    count,
                    completed_count,
                    tasks,
                });
            }
            
            Ok(Json(history_days))
        }
        Err(e) => {
            error!("获取历史数据失败: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}