use crate::state::AppState;
use crate::commands::response::Response;
use serde::Serialize;
use tauri::State;
use serde_json::json;
use chrono::{Utc, TimeZone};

// 键基本信息结构体
#[derive(Debug, Serialize)]
pub struct KeyInfo {
    pub key: String,
    #[serde(rename = "type")]
    pub key_type: String,
}

// 键详细信息结构体
#[derive(Debug, Serialize)]
pub struct KeyDetail {
    pub key: String,
    #[serde(rename = "type")]
    pub key_type: String,
    pub ttl: i64,
    pub size: usize,
    pub create_time: String, // ISO 8601 格式的时间字符串
    pub value: serde_json::Value, // 根据类型存储不同的值
}

// 键列表响应数据
#[derive(Debug, Serialize)]
pub struct KeysListData {
    pub keys: Vec<KeyInfo>,
    pub total: usize,
}

// 设置键值命令
#[tauri::command]
pub async fn set_key(
    connection_id: String,
    key: String,
    key_type: String,
    value: serde_json::Value,
    ttl: i64,
    state: State<'_, AppState>,
) -> Result<Response<()>, String> {
    let connections = state.connections.lock().unwrap();

    if let Some(conn_state) = connections.get(&connection_id) {
        match conn_state.client.get_connection() {
            Ok(mut conn) => {
                let current_db = conn_state.current_db;
                if let Err(e) = redis::cmd("SELECT").arg(current_db).query::<()>(&mut conn) {
                    return Ok(Response::error(format!("切换到数据库 {} 失败: {}", current_db, e)));
                }

                let result = match key_type.as_str() {
                    "string" => {
                        let string_value = value.as_str().unwrap_or("");
                        if ttl > 0 {
                            redis::cmd("SETEX")
                                .arg(&key)
                                .arg(ttl)
                                .arg(string_value)
                                .query::<()>(&mut conn)
                        } else {
                            redis::cmd("SET")
                                .arg(&key)
                                .arg(string_value)
                                .query::<()>(&mut conn)
                        }
                    }
                    "hash" => {
                        let hash_map: std::collections::HashMap<String, String> = serde_json::from_value(value)
                            .map_err(|e| format!("解析哈希值失败: {}", e))?;
                        
                        // 先删除旧键（如果存在）
                        let _ = redis::cmd("DEL").arg(&key).query::<()>(&mut conn);
                        
                        // 使用 HSET 批量设置
                        let mut cmd = redis::cmd("HSET");
                        cmd.arg(&key);
                        for (field, val) in hash_map {
                            cmd.arg(field).arg(val);
                        }
                        let result = cmd.query::<()>(&mut conn);
                        
                        // 设置 TTL
                        if result.is_ok() && ttl > 0 {
                            let _ = redis::cmd("EXPIRE").arg(&key).arg(ttl).query::<()>(&mut conn);
                        }
                        result
                    }
                    "list" => {
                        let list_items: Vec<String> = serde_json::from_value(value)
                            .map_err(|e| format!("解析列表值失败: {}", e))?;
                        
                        // 先删除旧键（如果存在）
                        let _ = redis::cmd("DEL").arg(&key).query::<()>(&mut conn);
                        
                        // 使用 RPUSH 批量添加
                        let mut cmd = redis::cmd("RPUSH");
                        cmd.arg(&key);
                        for item in list_items {
                            cmd.arg(item);
                        }
                        let result = cmd.query::<()>(&mut conn);
                        
                        // 设置 TTL
                        if result.is_ok() && ttl > 0 {
                            let _ = redis::cmd("EXPIRE").arg(&key).arg(ttl).query::<()>(&mut conn);
                        }
                        result
                    }
                    "set" => {
                        let set_items: Vec<String> = serde_json::from_value(value)
                            .map_err(|e| format!("解析集合值失败: {}", e))?;
                        
                        // 先删除旧键（如果存在）
                        let _ = redis::cmd("DEL").arg(&key).query::<()>(&mut conn);
                        
                        // 使用 SADD 批量添加
                        let mut cmd = redis::cmd("SADD");
                        cmd.arg(&key);
                        for item in set_items {
                            cmd.arg(item);
                        }
                        let result = cmd.query::<()>(&mut conn);
                        
                        // 设置 TTL
                        if result.is_ok() && ttl > 0 {
                            let _ = redis::cmd("EXPIRE").arg(&key).arg(ttl).query::<()>(&mut conn);
                        }
                        result
                    }
                    "zset" => {
                        let zset_items: Vec<(String, f64)> = serde_json::from_value(value)
                            .map_err(|e| format!("解析有序集合值失败: {}", e))?;
                        
                        // 先删除旧键（如果存在）
                        let _ = redis::cmd("DEL").arg(&key).query::<()>(&mut conn);
                        
                        // 使用 ZADD 批量添加
                        let mut cmd = redis::cmd("ZADD");
                        cmd.arg(&key);
                        for (member, score) in zset_items {
                            cmd.arg(score).arg(member);
                        }
                        let result = cmd.query::<()>(&mut conn);
                        
                        // 设置 TTL
                        if result.is_ok() && ttl > 0 {
                            let _ = redis::cmd("EXPIRE").arg(&key).arg(ttl).query::<()>(&mut conn);
                        }
                        result
                    }
                    _ => return Ok(Response::error(format!("不支持的类型: {}", key_type))),
                };

                match result {
                    Ok(_) => Ok(Response::<()>::success_empty_with_message(format!("成功创建 {} 类型的键 {}", key_type, key))),
                    Err(e) => Ok(Response::error(format!("设置失败: {}", e))),
                }
            }
            Err(e) => Ok(Response::error(format!("获取连接失败: {}", e))),
        }
    } else {
        Ok(Response::error("Redis 未连接".to_string()))
    }
}

// 获取键值命令 - 修复 nil 响应问题
#[tauri::command]
pub async fn get_key(
    connection_id: String,
    key: String,
    state: State<'_, AppState>,
) -> Result<Response<String>, String> {
    let connections = state.connections.lock().unwrap();

    if let Some(conn_state) = connections.get(&connection_id) {
        match conn_state.client.get_connection() {
            Ok(mut conn) => {
                let current_db = conn_state.current_db;
                if let Err(e) = redis::cmd("SELECT").arg(current_db).query::<()>(&mut conn) {
                    return Ok(Response::error(format!("切换到数据库 {} 失败: {}", current_db, e)));
                }

                // 使用 Option<String> 处理 nil 响应
                match redis::cmd("GET").arg(&key).query::<Option<String>>(&mut conn) {
                    Ok(Some(value)) => Ok(Response::success_with_message(value, format!("成功获取 {}", key))),
                    Ok(None) => Ok(Response::success_with_message("".to_string(), format!("键 {} 存在但值为空", key))),
                    Err(e) => Ok(Response::error(format!("获取失败: {}", e))),
                }
            }
            Err(e) => Ok(Response::error(format!("获取连接失败: {}", e))),
        }
    } else {
        Ok(Response::error("Redis 未连接".to_string()))
    }
}


// 获取键列表（只返回键名和类型）
#[tauri::command]
pub async fn get_keys(
    connection_id: String,
    pattern: String,
    state: State<'_, AppState>,
) -> Result<Response<KeysListData>, String> {
    let connections = state.connections.lock().unwrap();

    if let Some(conn_state) = connections.get(&connection_id) {
        match conn_state.client.get_connection() {
            Ok(mut conn) => {
                let current_db = conn_state.current_db;
                if let Err(e) = redis::cmd("SELECT").arg(current_db).query::<()>(&mut conn) {
                    return Ok(Response::error(format!("切换到数据库 {} 失败: {}", current_db, e)));
                }

                // 使用 SCAN 命令分批获取键名
                let mut keys_with_info = Vec::new();
                let mut cursor: u64 = 0;
                let batch_size = 500; // 每批处理的数量
                let mut total = 0;

                loop {
                    // 执行 SCAN 命令
                    let mut cmd = redis::cmd("SCAN");
                    cmd.arg(cursor)
                        .arg("MATCH")
                        .arg(&pattern)
                        .arg("COUNT")
                        .arg(batch_size);

                    let (new_cursor, keys): (u64, Vec<String>) = match cmd.query(&mut conn) {
                        Ok(r) => r,
                        Err(e) => {
                            return Ok(Response::error(format!("SCAN命令失败: {}", e)))
                        }
                    };

                    total += keys.len();
                    cursor = new_cursor;

                    // 如果本批次没有键，继续下一批
                    if keys.is_empty() {
                        if cursor == 0 {
                            break;
                        }
                        continue;
                    }

                    // 使用管道批量获取键类型
                    let mut pipe = redis::pipe();
                    for key in &keys {
                        pipe.cmd("TYPE").arg(key);
                    }

                    let key_types: Vec<String> = match pipe.query(&mut conn) {
                        Ok(types) => types,
                        Err(e) => {
                            return Ok(Response::error(format!("批量获取键类型失败: {}", e)))
                        }
                    };

                    // 组合键名和类型
                    for (i, key) in keys.into_iter().enumerate() {
                        if let Some(key_type) = key_types.get(i) {
                            keys_with_info.push(KeyInfo {
                                key,
                                key_type: key_type.clone(),
                            });
                        }
                    }

                    // 检查是否完成
                    if cursor == 0 {
                        break;
                    }
                }

                Ok(Response::success(KeysListData {
                    keys: keys_with_info,
                    total,
                }))
            }
            Err(e) => Ok(Response::error(format!("获取连接失败: {}", e))),
        }
    } else {
        Ok(Response::error("Redis 未连接".to_string()))
    }
}

#[tauri::command]
pub async fn get_key_detail(
    connection_id: String,
    key: String,
    state: State<'_, AppState>,
) -> Result<Response<KeyDetail>, String> {
    let connections = state.connections.lock().unwrap();

    if let Some(conn_state) = connections.get(&connection_id) {
        match conn_state.client.get_connection() {
            Ok(mut conn) => {
                let current_db = conn_state.current_db;
                if let Err(e) = redis::cmd("SELECT").arg(current_db).query::<()>(&mut conn) {
                    return Ok(Response::error(format!("切换到数据库 {} 失败: {}", current_db, e)));
                }

                // 检查键是否存在
                let exists: bool = match redis::cmd("EXISTS").arg(&key).query(&mut conn) {
                    Ok(exists) => exists,
                    Err(e) => {
                        return Ok(Response::error(format!("检查键存在失败: {}", e)))
                    }
                };

                if !exists {
                    return Ok(Response::error(format!("键 {} 不存在", key)));
                }

                // 获取键类型
                let key_type = match redis::cmd("TYPE").arg(&key).query::<String>(&mut conn) {
                    Ok(t) => t,
                    Err(e) => {
                        return Ok(Response::error(format!("获取键类型失败: {}", e)))
                    }
                };

                // 获取TTL
                let ttl = match redis::cmd("TTL").arg(&key).query::<i64>(&mut conn) {
                    Ok(t) => t,
                    Err(e) => {
                        return Ok(Response::error(format!("获取TTL失败: {}", e)))
                    }
                };

                // 获取键大小
                let size = match get_key_size_internal(&mut conn, &key) {
                    Ok(s) => s,
                    Err(e) => {
                        return Ok(Response::error(format!("获取键大小失败: {}", e)))
                    }
                };

                // 获取创建时间（Redis不直接支持，这里使用最后修改时间作为近似值）
                let last_modified: i64 = match redis::cmd("LASTSAVE").query(&mut conn) {
                    Ok(t) => t,
                    Err(_) => Utc::now().timestamp(),
                };

                let create_time = match Utc.timestamp_opt(last_modified, 0) {
                    chrono::LocalResult::Single(dt) => dt,
                    _ => Utc::now(),
                };

                // 根据类型获取值
                let value = match key_type.as_str() {
                    "string" => {
                        let val: String = match redis::cmd("GET").arg(&key).query(&mut conn) {
                            Ok(v) => v,
                            Err(e) => {
                                return Ok(Response::error(format!("获取字符串值失败: {}", e)))
                            }
                        };
                        json!(val)
                    }
                    "hash" => {
                        let val: Vec<(String, String)> = match redis::cmd("HGETALL").arg(&key).query(&mut conn) {
                            Ok(v) => v,
                            Err(e) => {
                                return Ok(Response::error(format!("获取哈希值失败: {}", e)))
                            }
                        };
                        let hash_items: Vec<serde_json::Value> = val.into_iter().map(|(field, value)| {
                            json!({
                                "field": field,
                                "value": value
                            })
                        }).collect();
                        json!(hash_items)
                    }
                    "list" => {
                        let val: Vec<String> = match redis::cmd("LRANGE").arg(&key).arg(0).arg(-1).query(&mut conn) {
                            Ok(v) => v,
                            Err(e) => {
                                return Ok(Response::error(format!("获取列表值失败: {}", e)))
                            }
                        };
                        json!(val)
                    }
                    "set" => {
                        let val: Vec<String> = match redis::cmd("SMEMBERS").arg(&key).query(&mut conn) {
                            Ok(v) => v,
                            Err(e) => {
                                return Ok(Response::error(format!("获取集合值失败: {}", e)))
                            }
                        };
                        json!(val)
                    }
                    "zset" => {
                        let val: Vec<(String, f64)> = match redis::cmd("ZRANGE").arg(&key).arg(0).arg(-1).arg("WITHSCORES").query(&mut conn) {
                            Ok(v) => v,
                            Err(e) => {
                                return Ok(Response::error(format!("获取有序集合值失败: {}", e)))
                            }
                        };
                        let zset_items: Vec<serde_json::Value> = val.into_iter().map(|(value, score)| {
                            json!({
                                "value": value,
                                "score": score
                            })
                        }).collect();
                        json!(zset_items)
                    }
                    _ => json!(null),
                };

                Ok(Response::success(KeyDetail {
                    key: key.clone(),
                    key_type,
                    ttl,
                    size,
                    create_time: create_time.to_rfc3339(),
                    value,
                }))
            }
            Err(e) => Ok(Response::error(format!("获取连接失败: {}", e))),
        }
    } else {
        Ok(Response::error("Redis 未连接".to_string()))
    }
}

// 获取键大小（内部函数）
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
) -> Result<Response<String>, String> {
    let connections = state.connections.lock().unwrap();

    if let Some(conn_state) = connections.get(&connection_id) {
        match conn_state.client.get_connection() {
            Ok(mut conn) => {
                match redis::cmd("TYPE").arg(&key).query::<String>(&mut conn) {
                    Ok(key_type) => Ok(Response::success(key_type)),
                    Err(e) => Ok(Response::error(format!("获取键类型失败: {}", e))),
                }
            }
            Err(e) => Ok(Response::error(format!("获取连接失败: {}", e))),
        }
    } else {
        Ok(Response::error("Redis 未连接".to_string()))
    }
}

// 获取键 TTL
#[tauri::command]
pub async fn get_key_ttl(
    connection_id: String,
    key: String,
    state: State<'_, AppState>,
) -> Result<Response<i64>, String> {
    let connections = state.connections.lock().unwrap();

    if let Some(conn_state) = connections.get(&connection_id) {
        match conn_state.client.get_connection() {
            Ok(mut conn) => {
                match redis::cmd("TTL").arg(&key).query::<i64>(&mut conn) {
                    Ok(ttl) => Ok(Response::success(ttl)),
                    Err(e) => Ok(Response::error(format!("获取键 TTL 失败: {}", e))),
                }
            }
            Err(e) => Ok(Response::error(format!("获取连接失败: {}", e))),
        }
    } else {
        Ok(Response::error("Redis 未连接".to_string()))
    }
}

// 设置键 TTL
#[tauri::command]
pub async fn set_key_ttl(
    connection_id: String,
    key: String,
    ttl: i64,
    state: State<'_, AppState>,
) -> Result<Response<()>, String> {
    let connections = state.connections.lock().unwrap();

    if let Some(conn_state) = connections.get(&connection_id) {
        match conn_state.client.get_connection() {
            Ok(mut conn) => {
                if ttl > 0 {
                    match redis::cmd("EXPIRE").arg(&key).arg(ttl).query::<i64>(&mut conn) {
                        Ok(result) => {
                            if result == 1 {
                                Ok(Response::<()>::success_empty())
                            } else {
                                Ok(Response::error("设置TTL失败".to_string()))
                            }
                        }
                        Err(e) => Ok(Response::error(format!("设置 TTL 失败: {}", e))),
                    }
                } else {
                    match redis::cmd("PERSIST").arg(&key).query::<i64>(&mut conn) {
                        Ok(result) => {
                            if result == 1 {
                                Ok(Response::<()>::success_empty())
                            } else {
                                Ok(Response::error("移除 TTL 失败".to_string()))
                            }
                        }
                        Err(e) => Ok(Response::error(format!("移除 TTL 失败: {}", e))),
                    }
                }
            }
            Err(e) => Ok(Response::error(format!("获取连接失败: {}", e))),
        }
    } else {
        Ok(Response::error("Redis 未连接".to_string()))
    }
}

// 获取键大小
#[tauri::command]
pub async fn get_key_size(
    connection_id: String,
    key: String,
    state: State<'_, AppState>,
) -> Result<Response<usize>, String> {
    let connections = state.connections.lock().unwrap();

    if let Some(conn_state) = connections.get(&connection_id) {
        match conn_state.client.get_connection() {
            Ok(mut conn) => {
                match get_key_size_internal(&mut conn, &key) {
                    Ok(size) => Ok(Response::success(size)),
                    Err(e) => Ok(Response::error(e)),
                }
            }
            Err(e) => Ok(Response::error(format!("获取连接失败: {}", e))),
        }
    } else {
        Ok(Response::error("Redis 未连接".to_string()))
    }
}

// 删除键
#[tauri::command]
pub async fn delete_key(
    connection_id: String,
    key: String,
    state: State<'_, AppState>,
) -> Result<Response<()>, String> {
    let connections = state.connections.lock().unwrap();

    if let Some(conn_state) = connections.get(&connection_id) {
        match conn_state.client.get_connection() {
            Ok(mut conn) => {
                match redis::cmd("DEL").arg(&key).query::<usize>(&mut conn) {
                    Ok(count) => {
                        if count > 0 {
                            Ok(Response::<()>::success_empty())
                        } else {
                            Ok(Response::error("键不存在".to_string()))
                        }
                    }
                    Err(e) => Ok(Response::error(format!("删除键失败: {}", e))),
                }
            }
            Err(e) => Ok(Response::error(format!("获取连接失败: {}", e))),
        }
    } else {
        Ok(Response::error("Redis 未连接".to_string()))
    }
}


// 新增命令：重命名键
#[tauri::command]
pub async fn rename_key(
    connection_id: String,
    old_key: String,
    new_key: String,
    state: State<'_, AppState>,
) -> Result<Response<()>, String> {
    let connections = state.connections.lock().unwrap();

    if let Some(conn_state) = connections.get(&connection_id) {
        match conn_state.client.get_connection() {
            Ok(mut conn) => {
                match redis::cmd("RENAME")
                    .arg(&old_key)
                    .arg(&new_key)
                    .query::<()>(&mut conn)
                {
                    Ok(_) => Ok(Response::<()>::success_empty_with_message(format!("成功重命名 {} 为 {}", old_key, new_key))),
                    Err(e) => Ok(Response::error(format!("重命名失败: {}", e))),
                }
            }
            Err(e) => Ok(Response::error(format!("获取连接失败: {}", e))),
        }
    } else {
        Ok(Response::error("Redis 未连接".to_string()))
    }
}

// 新增命令：更新哈希字段
#[tauri::command]
pub async fn update_hash_field(
    connection_id: String,
    key: String,
    field: String,
    value: String,
    state: State<'_, AppState>,
) -> Result<Response<()>, String> {
    let connections = state.connections.lock().unwrap();

    if let Some(conn_state) = connections.get(&connection_id) {
        match conn_state.client.get_connection() {
            Ok(mut conn) => {
                let current_db = conn_state.current_db;
                if let Err(e) = redis::cmd("SELECT").arg(current_db).query::<()>(&mut conn) {
                    return Ok(Response::error(format!("切换到数据库 {} 失败: {}", current_db, e)));
                }

                match redis::cmd("HSET")
                    .arg(&key)
                    .arg(&field)
                    .arg(&value)
                    .query::<i64>(&mut conn)
                {
                    Ok(_) => Ok(Response::<()>::success_empty_with_message(format!("成功更新 {} 的字段 {}", key, field))),
                    Err(e) => Ok(Response::error(format!("更新失败: {}", e))),
                }
            }
            Err(e) => Ok(Response::error(format!("获取连接失败: {}", e))),
        }
    } else {
        Ok(Response::error("Redis 未连接".to_string()))
    }
}

// 新增命令：删除哈希字段
#[tauri::command]
pub async fn delete_hash_field(
    connection_id: String,
    key: String,
    field: String,
    state: State<'_, AppState>,
) -> Result<Response<()>, String> {
    let connections = state.connections.lock().unwrap();

    if let Some(conn_state) = connections.get(&connection_id) {
        match conn_state.client.get_connection() {
            Ok(mut conn) => {
                let current_db = conn_state.current_db;
                if let Err(e) = redis::cmd("SELECT").arg(current_db).query::<()>(&mut conn) {
                    return Ok(Response::error(format!("切换到数据库 {} 失败: {}", current_db, e)));
                }

                match redis::cmd("HDEL")
                    .arg(&key)
                    .arg(&field)
                    .query::<i64>(&mut conn)
                {
                    Ok(count) => {
                        if count > 0 {
                            Ok(Response::<()>::success_empty_with_message(format!("成功删除 {} 的字段 {}", key, field)))
                        } else {
                            Ok(Response::error(format!("字段 {} 不存在", field)))
                        }
                    }
                    Err(e) => Ok(Response::error(format!("删除失败: {}", e))),
                }
            }
            Err(e) => Ok(Response::error(format!("获取连接失败: {}", e))),
        }
    } else {
        Ok(Response::error("Redis 未连接".to_string()))
    }
}

// 新增命令：更新列表项
#[tauri::command]
pub async fn update_list_item(
    connection_id: String,
    key: String,
    index: i64,
    value: String,
    state: State<'_, AppState>,
) -> Result<Response<()>, String> {
    let connections = state.connections.lock().unwrap();

    if let Some(conn_state) = connections.get(&connection_id) {
        match conn_state.client.get_connection() {
            Ok(mut conn) => {
                let current_db = conn_state.current_db;
                if let Err(e) = redis::cmd("SELECT").arg(current_db).query::<()>(&mut conn) {
                    return Ok(Response::error(format!("切换到数据库 {} 失败: {}", current_db, e)));
                }

                // 获取列表长度
                let len: i64 = match redis::cmd("LLEN").arg(&key).query(&mut conn) {
                    Ok(len) => len,
                    Err(e) => {
                        return Ok(Response::error(format!("获取列表长度失败: {}", e)))
                    }
                };

                // 验证索引是否在有效范围内
                if index >= len || index < -len {
                    return Ok(Response::error(format!("索引 {} 超出范围，列表长度为 {}", index, len)));
                }

                // 处理负索引
                let effective_index = if index < 0 {
                    // 将负索引转换为正索引
                    len + index
                } else {
                    index
                };

                match redis::cmd("LSET")
                    .arg(&key)
                    .arg(effective_index)
                    .arg(&value)
                    .query::<()>(&mut conn)
                {
                    Ok(_) => Ok(Response::<()>::success_empty_with_message(format!("成功更新 {} 的索引 {}", key, index))),
                    Err(e) => Ok(Response::error(format!("更新失败: {}", e))),
                }
            }
            Err(e) => Ok(Response::error(format!("获取连接失败: {}", e))),
        }
    } else {
        Ok(Response::error("Redis 未连接".to_string()))
    }
}

// 新增命令：删除列表项
#[tauri::command]
pub async fn delete_list_item(
    connection_id: String,
    key: String,
    value: String,
    count: i64,
    state: State<'_, AppState>,
) -> Result<Response<()>, String> {
    let connections = state.connections.lock().unwrap();

    if let Some(conn_state) = connections.get(&connection_id) {
        match conn_state.client.get_connection() {
            Ok(mut conn) => {
                let current_db = conn_state.current_db;
                if let Err(e) = redis::cmd("SELECT").arg(current_db).query::<()>(&mut conn) {
                    return Ok(Response::error(format!("切换到数据库 {} 失败: {}", current_db, e)));
                }

                match redis::cmd("LREM")
                    .arg(&key)
                    .arg(count)
                    .arg(&value)
                    .query::<i64>(&mut conn)
                {
                    Ok(removed_count) => {
                        if removed_count > 0 {
                            Ok(Response::<()>::success_empty_with_message(format!("成功删除 {} 个元素", removed_count)))
                        } else {
                            Ok(Response::error("未找到匹配的元素".to_string()))
                        }
                    }
                    Err(e) => Ok(Response::error(format!("删除失败: {}", e))),
                }
            }
            Err(e) => Ok(Response::error(format!("获取连接失败: {}", e))),
        }
    } else {
        Ok(Response::error("Redis 未连接".to_string()))
    }
}

// 新增命令：在列表末尾添加元素
#[tauri::command]
pub async fn append_list_item(
    connection_id: String,
    key: String,
    value: String,
    state: State<'_, AppState>,
) -> Result<Response<()>, String> {
    let connections = state.connections.lock().unwrap();

    if let Some(conn_state) = connections.get(&connection_id) {
        match conn_state.client.get_connection() {
            Ok(mut conn) => {
                let current_db = conn_state.current_db;
                if let Err(e) = redis::cmd("SELECT").arg(current_db).query::<()>(&mut conn) {
                    return Ok(Response::error(format!("切换到数据库 {} 失败: {}", current_db, e)));
                }

                match redis::cmd("RPUSH")
                    .arg(&key)
                    .arg(&value)
                    .query::<i64>(&mut conn)
                {
                    Ok(new_len) => Ok(Response::<()>::success_empty_with_message(format!("成功在列表 {} 末尾添加元素，新长度: {}", key, new_len))),
                    Err(e) => Ok(Response::error(format!("添加失败: {}", e))),
                }
            }
            Err(e) => Ok(Response::error(format!("获取连接失败: {}", e))),
        }
    } else {
        Ok(Response::error("Redis 未连接".to_string()))
    }
}

// 新增命令：添加Set元素
#[tauri::command]
pub async fn add_set_item(
    connection_id: String,
    key: String,
    value: String,
    state: State<'_, AppState>,
) -> Result<Response<()>, String> {
    let connections = state.connections.lock().unwrap();

    if let Some(conn_state) = connections.get(&connection_id) {
        match conn_state.client.get_connection() {
            Ok(mut conn) => {
                let current_db = conn_state.current_db;
                if let Err(e) = redis::cmd("SELECT").arg(current_db).query::<()>(&mut conn) {
                    return Ok(Response::error(format!("切换到数据库 {} 失败: {}", current_db, e)));
                }

                match redis::cmd("SADD")
                    .arg(&key)
                    .arg(&value)
                    .query::<i64>(&mut conn)
                {
                    Ok(count) => {
                        if count > 0 {
                            Ok(Response::<()>::success_empty_with_message(format!("成功添加元素到集合 {}", key)))
                        } else {
                            Ok(Response::error("元素已存在".to_string()))
                        }
                    }
                    Err(e) => Ok(Response::error(format!("添加失败: {}", e))),
                }
            }
            Err(e) => Ok(Response::error(format!("获取连接失败: {}", e))),
        }
    } else {
        Ok(Response::error("Redis 未连接".to_string()))
    }
}

// 新增命令：删除Set元素
#[tauri::command]
pub async fn delete_set_item(
    connection_id: String,
    key: String,
    value: String,
    state: State<'_, AppState>,
) -> Result<Response<()>, String> {
    let connections = state.connections.lock().unwrap();

    if let Some(conn_state) = connections.get(&connection_id) {
        match conn_state.client.get_connection() {
            Ok(mut conn) => {
                let current_db = conn_state.current_db;
                if let Err(e) = redis::cmd("SELECT").arg(current_db).query::<()>(&mut conn) {
                    return Ok(Response::error(format!("切换到数据库 {} 失败: {}", current_db, e)));
                }

                match redis::cmd("SREM")
                    .arg(&key)
                    .arg(&value)
                    .query::<i64>(&mut conn)
                {
                    Ok(count) => {
                        if count > 0 {
                            Ok(Response::<()>::success_empty_with_message(format!("成功从集合 {} 删除元素", key)))
                        } else {
                            Ok(Response::error("元素不存在".to_string()))
                        }
                    }
                    Err(e) => Ok(Response::error(format!("删除失败: {}", e))),
                }
            }
            Err(e) => Ok(Response::error(format!("获取连接失败: {}", e))),
        }
    } else {
        Ok(Response::error("Redis 未连接".to_string()))
    }
}

// 新增命令：添加ZSet元素
#[tauri::command]
pub async fn add_zset_item(
    connection_id: String,
    key: String,
    score: f64,
    value: String,
    state: State<'_, AppState>,
) -> Result<Response<()>, String> {
    let connections = state.connections.lock().unwrap();

    if let Some(conn_state) = connections.get(&connection_id) {
        match conn_state.client.get_connection() {
            Ok(mut conn) => {
                let current_db = conn_state.current_db;
                if let Err(e) = redis::cmd("SELECT").arg(current_db).query::<()>(&mut conn) {
                    return Ok(Response::error(format!("切换到数据库 {} 失败: {}", current_db, e)));
                }

                match redis::cmd("ZADD")
                    .arg(&key)
                    .arg(score)
                    .arg(&value)
                    .query::<i64>(&mut conn)
                {
                    Ok(count) => {
                        if count > 0 {
                            Ok(Response::<()>::success_empty_with_message(format!("成功添加元素到有序集合 {}", key)))
                        } else {
                            Ok(Response::error("元素已存在".to_string()))
                        }
                    }
                    Err(e) => Ok(Response::error(format!("添加失败: {}", e))),
                }
            }
            Err(e) => Ok(Response::error(format!("获取连接失败: {}", e))),
        }
    } else {
        Ok(Response::error("Redis 未连接".to_string()))
    }
}

// 新增命令：删除ZSet元素
#[tauri::command]
pub async fn delete_zset_item(
    connection_id: String,
    key: String,
    value: String,
    state: State<'_, AppState>,
) -> Result<Response<()>, String> {
    let connections = state.connections.lock().unwrap();

    if let Some(conn_state) = connections.get(&connection_id) {
        match conn_state.client.get_connection() {
            Ok(mut conn) => {
                let current_db = conn_state.current_db;
                if let Err(e) = redis::cmd("SELECT").arg(current_db).query::<()>(&mut conn) {
                    return Ok(Response::error(format!("切换到数据库 {} 失败: {}", current_db, e)));
                }

                match redis::cmd("ZREM")
                    .arg(&key)
                    .arg(&value)
                    .query::<i64>(&mut conn)
                {
                    Ok(count) => {
                        if count > 0 {
                            Ok(Response::<()>::success_empty_with_message(format!("成功从有序集合 {} 删除元素", key)))
                        } else {
                            Ok(Response::error("元素不存在".to_string()))
                        }
                    }
                    Err(e) => Ok(Response::error(format!("删除失败: {}", e))),
                }
            }
            Err(e) => Ok(Response::error(format!("获取连接失败: {}", e))),
        }
    } else {
        Ok(Response::error("Redis 未连接".to_string()))
    }
}
