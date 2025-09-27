use crate::state::AppState;
use tauri::State;

// 获取内存使用情况
#[tauri::command]
pub async fn get_memory_usage(
    connection_id: String,
    state: State<'_, AppState>,
) -> Result<usize, String> {
    let connections = state.connections.lock().unwrap();

    if let Some(conn_state) = connections.get(&connection_id) {
        match conn_state.client.get_connection() {
            Ok(mut conn) => {
                match redis::cmd("INFO").arg("memory").query::<String>(&mut conn) {
                    Ok(info) => {
                        // 解析内存使用信息
                        for line in info.lines() {
                            if line.starts_with("used_memory:") {
                                if let Some(value) = line.split(':').nth(1) {
                                    return value.trim().parse::<usize>().map_err(|e| e.to_string());
                                }
                            }
                        }
                        Err("无法解析内存使用信息".to_string())
                    }
                    Err(e) => Err(format!("获取内存信息失败: {}", e)),
                }
            }
            Err(e) => Err(format!("获取连接失败: {}", e)),
        }
    } else {
        Err("Redis 未连接".to_string())
    }
}

// 获取连接数
#[tauri::command]
pub async fn get_connections(
    connection_id: String,
    state: State<'_, AppState>,
) -> Result<usize, String> {
    let connections = state.connections.lock().unwrap();

    if let Some(conn_state) = connections.get(&connection_id) {
        match conn_state.client.get_connection() {
            Ok(mut conn) => {
                match redis::cmd("INFO").arg("clients").query::<String>(&mut conn) {
                    Ok(info) => {
                        // 解析连接数信息
                        for line in info.lines() {
                            if line.starts_with("connected_clients:") {
                                if let Some(value) = line.split(':').nth(1) {
                                    return value.trim().parse::<usize>().map_err(|e| e.to_string());
                                }
                            }
                        }
                        Err("无法解析连接数信息".to_string())
                    }
                    Err(e) => Err(format!("获取连接信息失败: {}", e)),
                }
            }
            Err(e) => Err(format!("获取连接失败: {}", e)),
        }
    } else {
        Err("Redis 未连接".to_string())
    }
}

// 获取命中率
#[tauri::command]
pub async fn get_hit_rate(
    connection_id: String,
    state: State<'_, AppState>,
) -> Result<f64, String> {
    let connections = state.connections.lock().unwrap();

    if let Some(conn_state) = connections.get(&connection_id) {
        match conn_state.client.get_connection() {
            Ok(mut conn) => {
                match redis::cmd("INFO").arg("stats").query::<String>(&mut conn) {
                    Ok(info) => {
                        // 解析命中率信息
                        let mut keyspace_hits = 0;
                        let mut keyspace_misses = 0;

                        for line in info.lines() {
                            if line.starts_with("keyspace_hits:") {
                                if let Some(value) = line.split(':').nth(1) {
                                    keyspace_hits = value.trim().parse::<u64>().unwrap_or(0);
                                }
                            } else if line.starts_with("keyspace_misses:") {
                                if let Some(value) = line.split(':').nth(1) {
                                    keyspace_misses = value.trim().parse::<u64>().unwrap_or(0);
                                }
                            }
                        }

                        let total = keyspace_hits + keyspace_misses;
                        if total > 0 {
                            Ok((keyspace_hits as f64 / total as f64 * 100.0).round())
                        } else {
                            Ok(0.0)
                        }
                    }
                    Err(e) => Err(format!("获取统计信息失败: {}", e)),
                }
            }
            Err(e) => Err(format!("获取连接失败: {}", e)),
        }
    } else {
        Err("Redis 未连接".to_string())
    }
}

// 获取运行时间
#[tauri::command]
pub async fn get_uptime(
    connection_id: String,
    state: State<'_, AppState>,
) -> Result<u64, String> {
    let connections = state.connections.lock().unwrap();

    if let Some(conn_state) = connections.get(&connection_id) {
        match conn_state.client.get_connection() {
            Ok(mut conn) => {
                match redis::cmd("INFO").arg("server").query::<String>(&mut conn) {
                    Ok(info) => {
                        // 解析运行时间信息
                        for line in info.lines() {
                            if line.starts_with("uptime_in_seconds:") {
                                if let Some(value) = line.split(':').nth(1) {
                                    return value.trim().parse::<u64>().map_err(|e| e.to_string());
                                }
                            }
                        }
                        Err("无法解析运行时间信息".to_string())
                    }
                    Err(e) => Err(format!("获取服务器信息失败: {}", e)),
                }
            }
            Err(e) => Err(format!("获取连接失败: {}", e)),
        }
    } else {
        Err("Redis 未连接".to_string())
    }
}
