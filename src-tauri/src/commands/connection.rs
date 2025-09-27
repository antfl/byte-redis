use crate::state::{AppState, ConnectionState, RedisConnectionConfig};
use redis::Client;
use tauri::State;

// 通用响应结构体
#[derive(Debug, serde::Serialize)]
pub struct RedisResponse {
    pub success: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

// 连接 Redis 命令
#[tauri::command]
pub async fn connect_redis(
    config: RedisConnectionConfig,
    state: State<'_, AppState>,
) -> Result<RedisResponse, String> {
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
            return Ok(RedisResponse {
                success: false,
                message: format!("客户端创建失败: {}", e),
                value: None,
            })
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

            Ok(RedisResponse {
                success: true,
                message: format!("成功连接到 {}", config.name),
                value: None,
            })
        }
        Err(e) => Ok(RedisResponse {
            success: false,
            message: format!("连接失败: {}", e),
            value: None,
        }),
    }
}

// 断开 Redis 连接
#[tauri::command]
pub async fn disconnect_redis(
    connection_id: String,
    state: State<'_, AppState>,
) -> Result<RedisResponse, String> {
    let mut connections = state.connections.lock().unwrap();

    if connections.remove(&connection_id).is_some() {
        Ok(RedisResponse {
            success: true,
            message: "连接已断开".to_string(),
            value: None,
        })
    } else {
        Ok(RedisResponse {
            success: false,
            message: "连接不存在".to_string(),
            value: None,
        })
    }
}
