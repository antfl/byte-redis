use crate::state::{AppState};
use serde::Serialize;
use tauri::State;

// 键详细信息结构体
#[derive(Debug, Serialize)]
pub struct KeyDetail {
    pub key: String,
    #[serde(rename = "type")]
    pub key_type: String,
    pub ttl: i64,
    pub size: usize,
}

// 键列表响应结构体
#[derive(Debug, Serialize)]
pub struct KeysResponse {
    pub success: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys: Option<Vec<String>>,
    pub total: Option<usize>,
    pub cursor: Option<u64>,
}

// 键详细信息响应结构体
#[derive(Debug, Serialize)]
pub struct KeysWithDetailsResponse {
    pub success: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys: Option<Vec<KeyDetail>>,
    pub total: Option<usize>,
}

// 通用响应结构体
#[derive(Debug, Serialize)]
pub struct RedisResponse {
    pub success: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

// 设置键值命令
#[tauri::command]
pub async fn set_key(
    connection_id: String,
    key: String,
    value: String,
    ttl: i64,
    state: State<'_, AppState>,
) -> Result<RedisResponse, String> {
    let connections = state.connections.lock().unwrap();

    if let Some(conn_state) = connections.get(&connection_id) {
        match conn_state.client.get_connection() {
            Ok(mut conn) => {
                let result = if ttl > 0 {
                    redis::cmd("SETEX")
                        .arg(&key)
                        .arg(ttl)
                        .arg(&value)
                        .query::<()>(&mut conn)
                } else {
                    redis::cmd("SET")
                        .arg(&key)
                        .arg(&value)
                        .query::<()>(&mut conn)
                };

                match result {
                    Ok(_) => Ok(RedisResponse {
                        success: true,
                        message: format!("成功设置 {} = {}", key, value),
                        value: None,
                    }),
                    Err(e) => Ok(RedisResponse {
                        success: false,
                        message: format!("设置失败: {}", e),
                        value: None,
                    }),
                }
            }
            Err(e) => Ok(RedisResponse {
                success: false,
                message: format!("获取连接失败: {}", e),
                value: None,
            }),
        }
    } else {
        Ok(RedisResponse {
            success: false,
            message: "Redis 未连接".to_string(),
            value: None,
        })
    }
}

// 获取键值命令
#[tauri::command]
pub async fn get_key(
    connection_id: String,
    key: String,
    state: State<'_, AppState>,
) -> Result<RedisResponse, String> {
    let connections = state.connections.lock().unwrap();

    if let Some(conn_state) = connections.get(&connection_id) {
        match conn_state.client.get_connection() {
            Ok(mut conn) => {
                match redis::cmd("GET").arg(&key).query::<String>(&mut conn) {
                    Ok(value) => Ok(RedisResponse {
                        success: true,
                        message: format!("成功获取 {}", key),
                        value: Some(value),
                    }),
                    Err(e) => Ok(RedisResponse {
                        success: false,
                        message: format!("获取失败: {}", e),
                        value: None,
                    }),
                }
            }
            Err(e) => Ok(RedisResponse {
                success: false,
                message: format!("获取连接失败: {}", e),
                value: None,
            }),
        }
    } else {
        Ok(RedisResponse {
            success: false,
            message: "Redis 未连接".to_string(),
            value: None,
        })
    }
}

// 分页获取所有键命令
#[tauri::command]
pub async fn get_keys(
    connection_id: String,
    cursor: u64,
    pattern: String,
    count: usize,
    state: State<'_, AppState>,
) -> Result<KeysResponse, String> {
    let connections = state.connections.lock().unwrap();

    if let Some(conn_state) = connections.get(&connection_id) {
        match conn_state.client.get_connection() {
            Ok(mut conn) => {
                let mut cmd = redis::cmd("SCAN");
                cmd.arg(cursor)
                    .arg("MATCH")
                    .arg(&pattern)
                    .arg("COUNT")
                    .arg(count);

                let res: (u64, Vec<String>) = match cmd.query(&mut conn) {
                    Ok(r) => r,
                    Err(e) => {
                        return Ok(KeysResponse {
                            success: false,
                            message: format!("SCAN命令失败: {}", e),
                            keys: None,
                            total: None,
                            cursor: None,
                        })
                    }
                };

                // 获取总数
                let total: usize = match redis::cmd("DBSIZE").query(&mut conn) {
                    Ok(t) => t,
                    Err(_) => 0,
                };

                Ok(KeysResponse {
                    success: true,
                    message: "成功获取键列表".to_string(),
                    keys: Some(res.1),
                    total: Some(total),
                    cursor: Some(res.0),
                })
            }
            Err(e) => Ok(KeysResponse {
                success: false,
                message: format!("获取连接失败: {}", e),
                keys: None,
                total: None,
                cursor: None,
            }),
        }
    } else {
        Ok(KeysResponse {
            success: false,
            message: "Redis 未连接".to_string(),
            keys: None,
            total: None,
            cursor: None,
        })
    }
}

#[tauri::command]
pub async fn get_keys_with_details(
    connection_id: String,
    pattern: String,
    offset: usize,
    limit: usize,
    state: State<'_, AppState>,
) -> Result<KeysWithDetailsResponse, String> {
    let connections = state.connections.lock().unwrap();

    if let Some(conn_state) = connections.get(&connection_id) {
        match conn_state.client.get_connection() {
            Ok(mut conn) => {
                // 使用 SCAN 命令获取键名
                let mut cmd = redis::cmd("SCAN");
                cmd.cursor_arg(0)
                    .arg("MATCH")
                    .arg(&pattern)
                    .arg("COUNT")
                    .arg(10000);

                let res: (u64, Vec<String>) = match cmd.query(&mut conn) {
                    Ok(r) => r,
                    Err(e) => {
                        return Ok(KeysWithDetailsResponse {
                            success: false,
                            message: format!("SCAN命令失败: {}", e),
                            keys: None,
                            total: None,
                        })
                    }
                };

                let all_keys = res.1;
                let total = all_keys.len();

                // 分页处理
                let start = offset;
                let end = std::cmp::min(start + limit, total);
                let page_keys = &all_keys[start..end];

                // 使用管道获取每个键的详细信息
                let mut pipe = redis::pipe();
                for key in page_keys {
                    pipe.cmd("TYPE").arg(key)
                        .cmd("TTL").arg(key);
                }

                let results: Vec<redis::Value> = match pipe.query(&mut conn) {
                    Ok(r) => r,
                    Err(e) => {
                        return Ok(KeysWithDetailsResponse {
                            success: false,
                            message: format!("管道查询失败: {}", e),
                            keys: None,
                            total: None,
                        })
                    }
                };

                // 解析结果
                let mut keys_with_details = Vec::new();
                let mut i = 0;
                for key in page_keys {
                    if i + 1 >= results.len() {
                        break;
                    }

                    // 解析类型
                    let key_type = match &results[i] {
                        redis::Value::Data(data) => String::from_utf8_lossy(data).to_string(),
                        redis::Value::Status(s) => s.clone(),
                        _ => "unknown".to_string(),
                    };

                    // 解析 TTL
                    let ttl = match &results[i+1] {
                        redis::Value::Int(n) => *n as i64,
                        _ => -1,
                    };

                    // 获取键大小
                    let size = match get_key_size_internal(&mut conn, key) {
                        Ok(size) => size,
                        Err(_) => 0, // 如果获取大小失败，使用 0 作为默认值
                    };

                    keys_with_details.push(KeyDetail {
                        key: key.clone(),
                        key_type,
                        ttl,
                        size,
                    });

                    i += 2;
                }

                Ok(KeysWithDetailsResponse {
                    success: true,
                    message: "成功获取键列表".to_string(),
                    keys: Some(keys_with_details),
                    total: Some(total),
                })
            }
            Err(e) => Ok(KeysWithDetailsResponse {
                success: false,
                message: format!("获取连接失败: {}", e),
                keys: None,
                total: None,
            }),
        }
    } else {
        Ok(KeysWithDetailsResponse {
            success: false,
            message: "Redis 未连接".to_string(),
            keys: None,
            total: None,
        })
    }
}

// 获取键大小
fn get_key_size_internal(conn: &mut redis::Connection, key: &str) -> Result<usize, String> {
    // 尝试使用 MEMORY USAGE 命令
    if let Ok(size) = redis::cmd("MEMORY").arg("USAGE").arg(key).query::<usize>(conn) {
        return Ok(size);
    }

    // 如果 MEMORY 命令失败，使用替代方法估算大小
    match redis::cmd("DEBUG").arg("OBJECT").arg(key).query::<String>(conn) {
        Ok(debug_info) => {
            // 解析 DEBUG OBJECT 的输出以获取序列化长度
            for part in debug_info.split_whitespace() {
                if part.starts_with("serializedlength:") {
                    if let Some(len_str) = part.split(':').nth(1) {
                        if let Ok(len) = len_str.parse::<usize>() {
                            return Ok(len);
                        }
                    }
                }
            }
            Err("无法解析键大小".to_string())
        }
        Err(e) => Err(format!("获取键大小失败: {}", e)),
    }
}

// 获取键类型
#[tauri::command]
pub async fn get_key_type(
    connection_id: String,
    key: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let connections = state.connections.lock().unwrap();

    if let Some(conn_state) = connections.get(&connection_id) {
        match conn_state.client.get_connection() {
            Ok(mut conn) => {
                match redis::cmd("TYPE").arg(&key).query::<String>(&mut conn) {
                    Ok(key_type) => Ok(key_type),
                    Err(e) => Err(format!("获取键类型失败: {}", e)),
                }
            }
            Err(e) => Err(format!("获取连接失败: {}", e)),
        }
    } else {
        Err("Redis 未连接".to_string())
    }
}

// 获取键 TTL
#[tauri::command]
pub async fn get_key_ttl(
    connection_id: String,
    key: String,
    state: State<'_, AppState>,
) -> Result<i64, String> {
    let connections = state.connections.lock().unwrap();

    if let Some(conn_state) = connections.get(&connection_id) {
        match conn_state.client.get_connection() {
            Ok(mut conn) => {
                match redis::cmd("TTL").arg(&key).query::<i64>(&mut conn) {
                    Ok(ttl) => Ok(ttl),
                    Err(e) => Err(format!("获取键 TTL 失败: {}", e)),
                }
            }
            Err(e) => Err(format!("获取连接失败: {}", e)),
        }
    } else {
        Err("Redis 未连接".to_string())
    }
}

// 设置键 TTL
#[tauri::command]
pub async fn set_key_ttl(
    connection_id: String,
    key: String,
    ttl: i64,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let connections = state.connections.lock().unwrap();

    if let Some(conn_state) = connections.get(&connection_id) {
        match conn_state.client.get_connection() {
            Ok(mut conn) => {
                if ttl > 0 {
                    match redis::cmd("EXPIRE").arg(&key).arg(ttl).query::<i64>(&mut conn) {
                        Ok(result) => {
                            if result == 1 {
                                Ok(())
                            } else {
                                Err("设置TTL失败".to_string())
                            }
                        }
                        Err(e) => Err(format!("设置 TTL 失败: {}", e)),
                    }
                } else {
                    match redis::cmd("PERSIST").arg(&key).query::<i64>(&mut conn) {
                        Ok(result) => {
                            if result == 1 {
                                Ok(())
                            } else {
                                Err("移除 TTL 失败".to_string())
                            }
                        }
                        Err(e) => Err(format!("移除 TTL 失败: {}", e)),
                    }
                }
            }
            Err(e) => Err(format!("获取连接失败: {}", e)),
        }
    } else {
        Err("Redis 未连接".to_string())
    }
}

// 获取键大小
#[tauri::command]
pub async fn get_key_size(
    connection_id: String,
    key: String,
    state: State<'_, AppState>,
) -> Result<usize, String> {
    let connections = state.connections.lock().unwrap();

    if let Some(conn_state) = connections.get(&connection_id) {
        match conn_state.client.get_connection() {
            Ok(mut conn) => {
                get_key_size_internal(&mut conn, &key)
            }
            Err(e) => Err(format!("获取连接失败: {}", e)),
        }
    } else {
        Err("Redis 未连接".to_string())
    }
}

// 删除键
#[tauri::command]
pub async fn delete_key(
    connection_id: String,
    key: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let connections = state.connections.lock().unwrap();

    if let Some(conn_state) = connections.get(&connection_id) {
        match conn_state.client.get_connection() {
            Ok(mut conn) => {
                match redis::cmd("DEL").arg(&key).query::<usize>(&mut conn) {
                    Ok(count) => {
                        if count > 0 {
                            Ok(())
                        } else {
                            Err("键不存在".to_string())
                        }
                    }
                    Err(e) => Err(format!("删除键失败: {}", e)),
                }
            }
            Err(e) => Err(format!("获取连接失败: {}", e)),
        }
    } else {
        Err("Redis 未连接".to_string())
    }
}
