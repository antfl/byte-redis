use crate::state::{AppState, ConnectionState, RedisConnectionConfig};
use crate::commands::response::Response;
use redis::Client;
use tauri::State;

// 连接 Redis 命令
#[tauri::command]
pub async fn connect_redis(
    config: RedisConnectionConfig,
    state: State<'_, AppState>,
) -> Result<Response<()>, String> {
    // 构建连接 URL
    let url = if let (Some(username), Some(password)) = (config.username.clone(), config.password.clone()) {
        format!("redis://{}:{}@{}:{}/{}",
            username, password, config.host, config.port, config.db.unwrap_or(0))
    } else if let Some(password) = config.password {
        format!("redis://:{}@{}:{}/{}",
            password, config.host, config.port, config.db.unwrap_or(0))
    } else {
        format!("redis://{}:{}/{}",
            config.host, config.port, config.db.unwrap_or(0))
    };

    let client = match Client::open(url) {
        Ok(c) => c,
        Err(e) => {
            return Ok(Response::error(format!("客户端创建失败: {}", e)))
        }
    };

    // 测试连接
    match client.get_connection() {
        Ok(_) => {
            // 保存客户端到状态
            let mut connections = state.connections.lock().unwrap();
            let db_index = config.db.unwrap_or(0);

            // 切换到指定数据库
            let mut conn = client.get_connection().map_err(|e| format!("获取连接失败: {}", e))?;
            redis::cmd("SELECT").arg(db_index).query::<()>(&mut conn)
                .map_err(|e| format!("切换数据库失败: {}", e))?;

            connections.insert(
                config.id.clone(),
                ConnectionState {
                    client: client,
                    current_db: db_index,
                },
            );

            Ok(Response::<()>::success_empty_with_message(format!("成功连接到 {}", config.name)))
        }
        Err(e) => Ok(Response::error(format!("连接失败: {}", e))),
    }
}

// 断开 Redis 连接
#[tauri::command]
pub async fn disconnect_redis(
    connection_id: String,
    state: State<'_, AppState>,
) -> Result<Response<()>, String> {
    let mut connections = state.connections.lock().unwrap();

    if connections.remove(&connection_id).is_some() {
        Ok(Response::<()>::success_empty_with_message("连接已断开".to_string()))
    } else {
        Ok(Response::error("连接不存在".to_string()))
    }
}
