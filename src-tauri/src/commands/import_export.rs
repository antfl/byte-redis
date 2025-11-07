use crate::state::AppState;
use crate::commands::response::Response;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tauri::State;
use chrono::{Utc, TimeZone};
use redis::Connection;

// 键详情结构体
#[derive(Debug, Serialize, Deserialize)]
pub struct KeyDetail {
    pub key: String,
    #[serde(rename = "type")]
    pub key_type: String,
    pub ttl: i64,
    pub size: usize,
    pub create_time: String,
    pub value: serde_json::Value,
}

// 导出单个键
#[tauri::command]
pub async fn export_key(
    connection_id: String,
    key: String,
    state: State<'_, AppState>,
) -> Result<Response<KeyDetail>, String> {
    let connections = state.connections.lock().unwrap();

    if let Some(conn_state) = connections.get(&connection_id) {
        match conn_state.client.get_connection() {
            Ok(mut conn) => {
                match get_key_detail_internal(&mut conn, &key) {
                    Ok(detail) => Ok(Response::success(detail)),
                    Err(e) => Ok(Response::error(e)),
                }
            }
            Err(e) => Ok(Response::error(format!("获取连接失败: {}", e))),
        }
    } else {
        Ok(Response::error("Redis 未连接".to_string()))
    }
}

// 导出多个键
#[tauri::command]
pub async fn export_keys(
    connection_id: String,
    pattern: String,
    state: State<'_, AppState>,
) -> Result<Response<Vec<KeyDetail>>, String> {
    let connections = state.connections.lock().unwrap();

    if let Some(conn_state) = connections.get(&connection_id) {
        match conn_state.client.get_connection() {
            Ok(mut conn) => {
                // 获取匹配的键列表
                let keys: Vec<String> = redis::cmd("KEYS")
                    .arg(&pattern)
                    .query(&mut conn)
                    .map_err(|e| format!("获取键列表失败: {}", e))?;

                // 导出每个键
                let mut exported_keys = Vec::new();
                for key in keys {
                    match get_key_detail_internal(&mut conn, &key) {
                        Ok(detail) => exported_keys.push(detail),
                        Err(e) => eprintln!("导出键 {} 失败: {}", key, e),
                    }
                }
                Ok(Response::success(exported_keys))
            }
            Err(e) => Ok(Response::error(format!("获取连接失败: {}", e))),
        }
    } else {
        Ok(Response::error("Redis 未连接".to_string()))
    }
}

// 内部函数：获取键详情
fn get_key_detail_internal(
    conn: &mut Connection,
    key: &str,
) -> Result<KeyDetail, String> {
    // 检查键是否存在
    let exists: bool = redis::cmd("EXISTS")
        .arg(key)
        .query(conn)
        .map_err(|e| format!("检查键存在失败: {}", e))?;

    if !exists {
        return Err(format!("键 {} 不存在", key));
    }

    // 获取键类型
    let key_type = redis::cmd("TYPE")
        .arg(key)
        .query::<String>(conn)
        .map_err(|e| format!("获取键类型失败: {}", e))?;

    // 获取TTL
    let ttl = redis::cmd("TTL")
        .arg(key)
        .query::<i64>(conn)
        .map_err(|e| format!("获取TTL失败: {}", e))?;

    // 获取键大小
    let size = get_key_size_internal(conn, key)?;

    // 获取创建时间（使用最后修改时间作为近似值）
    let last_modified: i64 = redis::cmd("LASTSAVE")
        .query(conn)
        .unwrap_or_else(|_| Utc::now().timestamp());

    let create_time = Utc.timestamp_opt(last_modified, 0)
        .single()
        .unwrap_or_else(Utc::now)
        .to_rfc3339();

    // 根据类型获取值
    let value = match key_type.as_str() {
        "string" => {
            let val: String = redis::cmd("GET")
                .arg(key)
                .query(conn)
                .map_err(|e| format!("获取字符串值失败: {}", e))?;
            json!(val)
        }
        "hash" => {
            let val: Vec<(String, String)> = redis::cmd("HGETALL")
                .arg(key)
                .query(conn)
                .map_err(|e| format!("获取哈希值失败: {}", e))?;
            json!(val)
        }
        "list" => {
            let val: Vec<String> = redis::cmd("LRANGE")
                .arg(key)
                .arg(0)
                .arg(-1)
                .query(conn)
                .map_err(|e| format!("获取列表值失败: {}", e))?;
            json!(val)
        }
        "set" => {
            let val: Vec<String> = redis::cmd("SMEMBERS")
                .arg(key)
                .query(conn)
                .map_err(|e| format!("获取集合值失败: {}", e))?;
            json!(val)
        }
        "zset" => {
            let val: Vec<(String, f64)> = redis::cmd("ZRANGE")
                .arg(key)
                .arg(0)
                .arg(-1)
                .arg("WITHSCORES")
                .query(conn)
                .map_err(|e| format!("获取有序集合值失败: {}", e))?;
            json!(val)
        }
        _ => json!(null),
    };

    Ok(KeyDetail {
        key: key.to_string(),
        key_type,
        ttl,
        size,
        create_time,
        value,
    })
}

// 获取键大小（内部函数）
fn get_key_size_internal(conn: &mut Connection, key: &str) -> Result<usize, String> {
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

// 导入单个键
#[tauri::command]
pub async fn import_key(
    connection_id: String,
    key_detail: KeyDetail,
    overwrite: bool,
    state: State<'_, AppState>,
) -> Result<Response<()>, String> {
    let connections = state.connections.lock().unwrap();

    if let Some(conn_state) = connections.get(&connection_id) {
        match conn_state.client.get_connection() {
            Ok(mut conn) => {
                // 检查键是否存在
                let exists: bool = redis::cmd("EXISTS")
                    .arg(&key_detail.key)
                    .query(&mut conn)
                    .map_err(|e| format!("检查键存在失败: {}", e))?;

                if exists && !overwrite {
                    return Ok(Response::error(format!("键 {} 已存在，跳过导入", key_detail.key)));
                }

                // 如果存在且需要覆盖，先删除旧键
                if exists {
                    redis::cmd("DEL")
                        .arg(&key_detail.key)
                        .query::<()>(&mut conn)
                        .map_err(|e| format!("删除旧键失败: {}", e))?;
                }

                // 根据类型创建键
                match key_detail.key_type.as_str() {
                    "string" => {
                        let value = key_detail.value.as_str().ok_or("无效的字符串值")?;
                        if key_detail.ttl > 0 {
                            redis::cmd("SETEX")
                                .arg(&key_detail.key)
                                .arg(key_detail.ttl)
                                .arg(value)
                                .query::<()>(&mut conn)
                                .map_err(|e| format!("导入字符串键失败: {}", e))?;
                        } else {
                            redis::cmd("SET")
                                .arg(&key_detail.key)
                                .arg(value)
                                .query::<()>(&mut conn)
                                .map_err(|e| format!("导入字符串键失败: {}", e))?;
                        }
                    }
                    "hash" => {
                        let items: Vec<(String, String)> = serde_json::from_value(key_detail.value)
                            .map_err(|e| format!("解析哈希值失败: {}", e))?;

                        let mut pipe = redis::pipe();
                        for (field, value) in items {
                            pipe.cmd("HSET")
                                .arg(&key_detail.key)
                                .arg(field)
                                .arg(value);
                        }
                        pipe.query::<()>(&mut conn)
                            .map_err(|e| format!("导入哈希键失败: {}", e))?;

                        // 设置TTL
                        if key_detail.ttl > 0 {
                            redis::cmd("EXPIRE")
                                .arg(&key_detail.key)
                                .arg(key_detail.ttl)
                                .query::<()>(&mut conn)
                                .map_err(|e| format!("设置TTL失败: {}", e))?;
                        }
                    }
                    "list" => {
                        let items: Vec<String> = serde_json::from_value(key_detail.value)
                            .map_err(|e| format!("解析列表值失败: {}", e))?;

                        let mut pipe = redis::pipe();
                        for item in items {
                            pipe.cmd("RPUSH")
                                .arg(&key_detail.key)
                                .arg(item);
                        }
                        pipe.query::<()>(&mut conn)
                            .map_err(|e| format!("导入列表键失败: {}", e))?;

                        // 设置TTL
                        if key_detail.ttl > 0 {
                            redis::cmd("EXPIRE")
                                .arg(&key_detail.key)
                                .arg(key_detail.ttl)
                                .query::<()>(&mut conn)
                                .map_err(|e| format!("设置TTL失败: {}", e))?;
                        }
                    }
                    "set" => {
                        let items: Vec<String> = serde_json::from_value(key_detail.value)
                            .map_err(|e| format!("解析集合值失败: {}", e))?;

                        let mut pipe = redis::pipe();
                        for item in items {
                            pipe.cmd("SADD")
                                .arg(&key_detail.key)
                                .arg(item);
                        }
                        pipe.query::<()>(&mut conn)
                            .map_err(|e| format!("导入集合键失败: {}", e))?;

                        // 设置TTL
                        if key_detail.ttl > 0 {
                            redis::cmd("EXPIRE")
                                .arg(&key_detail.key)
                                .arg(key_detail.ttl)
                                .query::<()>(&mut conn)
                                .map_err(|e| format!("设置TTL失败: {}", e))?;
                        }
                    }
                    "zset" => {
                        let items: Vec<(String, f64)> = serde_json::from_value(key_detail.value)
                            .map_err(|e| format!("解析有序集合值失败: {}", e))?;

                        let mut pipe = redis::pipe();
                        for (value, score) in items {
                            pipe.cmd("ZADD")
                                .arg(&key_detail.key)
                                .arg(score)
                                .arg(value);
                        }
                        pipe.query::<()>(&mut conn)
                            .map_err(|e| format!("导入有序集合键失败: {}", e))?;

                        // 设置TTL
                        if key_detail.ttl > 0 {
                            redis::cmd("EXPIRE")
                                .arg(&key_detail.key)
                                .arg(key_detail.ttl)
                                .query::<()>(&mut conn)
                                .map_err(|e| format!("设置TTL失败: {}", e))?;
                        }
                    }
                    _ => {
                        return Err(format!("不支持的类型: {}", key_detail.key_type));
                    }
                }

                Ok(Response::<()>::success_empty_with_message(format!("成功导入键 {}", key_detail.key)))
            }
            Err(e) => Ok(Response::error(format!("获取连接失败: {}", e))),
        }
    } else {
        Ok(Response::error("Redis 未连接".to_string()))
    }
}

// 导入多个键
#[tauri::command]
pub async fn import_keys(
    connection_id: String,
    keys: Vec<KeyDetail>,
    overwrite: bool,
    state: State<'_, AppState>,
) -> Result<Response<()>, String> {
    let mut success_count = 0;
    let mut error_count = 0;
    let mut errors = Vec::new();

    // 获取连接状态
    let connections = state.connections.lock().unwrap();

    for key_detail in keys {
        if let Some(conn_state) = connections.get(&connection_id) {
            match conn_state.client.get_connection() {
                Ok(mut conn) => {
                    // 检查键是否存在
                    let exists: bool = match redis::cmd("EXISTS")
                        .arg(&key_detail.key)
                        .query(&mut conn)
                    {
                        Ok(exists) => exists,
                        Err(e) => {
                            errors.push(format!("检查键存在失败: {}", e));
                            error_count += 1;
                            continue;
                        }
                    };

                    if exists && !overwrite {
                        errors.push(format!("键 {} 已存在，跳过导入", key_detail.key));
                        error_count += 1;
                        continue;
                    }

                    // 如果存在且需要覆盖，先删除旧键
                    if exists {
                        if let Err(e) = redis::cmd("DEL")
                            .arg(&key_detail.key)
                            .query::<()>(&mut conn)
                        {
                            errors.push(format!("删除旧键失败: {}", e));
                            error_count += 1;
                            continue;
                        }
                    }

                    // 根据类型创建键
                    match key_detail.key_type.as_str() {
                        "string" => {
                            let value = match key_detail.value.as_str() {
                                Some(v) => v,
                                None => {
                                    errors.push("无效的字符串值".to_string());
                                    error_count += 1;
                                    continue;
                                }
                            };

                            let result = if key_detail.ttl > 0 {
                                redis::cmd("SETEX")
                                    .arg(&key_detail.key)
                                    .arg(key_detail.ttl)
                                    .arg(value)
                                    .query::<()>(&mut conn)
                            } else {
                                redis::cmd("SET")
                                    .arg(&key_detail.key)
                                    .arg(value)
                                    .query::<()>(&mut conn)
                            };

                            if let Err(e) = result {
                                errors.push(format!("导入字符串键失败: {}", e));
                                error_count += 1;
                                continue;
                            }
                        }
                        "hash" => {
                            let items: Vec<(String, String)> = match serde_json::from_value(key_detail.value.clone()) {
                                Ok(items) => items,
                                Err(e) => {
                                    errors.push(format!("解析哈希值失败: {}", e));
                                    error_count += 1;
                                    continue;
                                }
                            };

                            let mut pipe = redis::pipe();
                            for (field, value) in items {
                                pipe.cmd("HSET")
                                    .arg(&key_detail.key)
                                    .arg(field)
                                    .arg(value);
                            }

                            if let Err(e) = pipe.query::<()>(&mut conn) {
                                errors.push(format!("导入哈希键失败: {}", e));
                                error_count += 1;
                                continue;
                            }

                            // 设置TTL
                            if key_detail.ttl > 0 {
                                if let Err(e) = redis::cmd("EXPIRE")
                                    .arg(&key_detail.key)
                                    .arg(key_detail.ttl)
                                    .query::<()>(&mut conn)
                                {
                                    errors.push(format!("设置TTL失败: {}", e));
                                    error_count += 1;
                                    continue;
                                }
                            }
                        }
                        "list" => {
                            let items: Vec<String> = match serde_json::from_value(key_detail.value.clone()) {
                                Ok(items) => items,
                                Err(e) => {
                                    errors.push(format!("解析列表值失败: {}", e));
                                    error_count += 1;
                                    continue;
                                }
                            };

                            let mut pipe = redis::pipe();
                            for item in items {
                                pipe.cmd("RPUSH")
                                    .arg(&key_detail.key)
                                    .arg(item);
                            }

                            if let Err(e) = pipe.query::<()>(&mut conn) {
                                errors.push(format!("导入列表键失败: {}", e));
                                error_count += 1;
                                continue;
                            }

                            // 设置TTL
                            if key_detail.ttl > 0 {
                                if let Err(e) = redis::cmd("EXPIRE")
                                    .arg(&key_detail.key)
                                    .arg(key_detail.ttl)
                                    .query::<()>(&mut conn)
                                {
                                    errors.push(format!("设置TTL失败: {}", e));
                                    error_count += 1;
                                    continue;
                                }
                            }
                        }
                        "set" => {
                            let items: Vec<String> = match serde_json::from_value(key_detail.value.clone()) {
                                Ok(items) => items,
                                Err(e) => {
                                    errors.push(format!("解析集合值失败: {}", e));
                                    error_count += 1;
                                    continue;
                                }
                            };

                            let mut pipe = redis::pipe();
                            for item in items {
                                pipe.cmd("SADD")
                                    .arg(&key_detail.key)
                                    .arg(item);
                            }

                            if let Err(e) = pipe.query::<()>(&mut conn) {
                                errors.push(format!("导入集合键失败: {}", e));
                                error_count += 1;
                                continue;
                            }

                            // 设置TTL
                            if key_detail.ttl > 0 {
                                if let Err(e) = redis::cmd("EXPIRE")
                                    .arg(&key_detail.key)
                                    .arg(key_detail.ttl)
                                    .query::<()>(&mut conn)
                                {
                                    errors.push(format!("设置TTL失败: {}", e));
                                    error_count += 1;
                                    continue;
                                }
                            }
                        }
                        "zset" => {
                            let items: Vec<(String, f64)> = match serde_json::from_value(key_detail.value.clone()) {
                                Ok(items) => items,
                                Err(e) => {
                                    errors.push(format!("解析有序集合值失败: {}", e));
                                    error_count += 1;
                                    continue;
                                }
                            };

                            let mut pipe = redis::pipe();
                            for (value, score) in items {
                                pipe.cmd("ZADD")
                                    .arg(&key_detail.key)
                                    .arg(score)
                                    .arg(value);
                            }

                            if let Err(e) = pipe.query::<()>(&mut conn) {
                                errors.push(format!("导入有序集合键失败: {}", e));
                                error_count += 1;
                                continue;
                            }

                            // 设置TTL
                            if key_detail.ttl > 0 {
                                if let Err(e) = redis::cmd("EXPIRE")
                                    .arg(&key_detail.key)
                                    .arg(key_detail.ttl)
                                    .query::<()>(&mut conn)
                                {
                                    errors.push(format!("设置TTL失败: {}", e));
                                    error_count += 1;
                                    continue;
                                }
                            }
                        }
                        _ => {
                            errors.push(format!("不支持的类型: {}", key_detail.key_type));
                            error_count += 1;
                            continue;
                        }
                    }

                    success_count += 1;
                }
                Err(e) => {
                    errors.push(format!("获取连接失败: {}", e));
                    error_count += 1;
                }
            }
        } else {
            errors.push("Redis 未连接".to_string());
            error_count += 1;
        }
    }

    let message = if error_count == 0 {
        format!("成功导入所有 {} 个键", success_count)
    } else {
        format!(
            "导入完成: 成功 {} 个, 失败 {} 个\n错误列表:\n{}",
            success_count,
            error_count,
            errors.join("\n")
        )
    };

    if error_count == 0 {
        Ok(Response::<()>::success_empty_with_message(message))
    } else {
        Ok(Response::error(message))
    }
}
