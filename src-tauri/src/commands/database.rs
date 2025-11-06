use crate::state::AppState;
use tauri::State;

// 获取数据库数量
#[tauri::command]
pub async fn get_db_count(
    connection_id: String,
    state: State<'_, AppState>,
) -> Result<usize, String> {
    let connections = state.connections.lock().unwrap();

    if let Some(conn_state) = connections.get(&connection_id) {
        match conn_state.client.get_connection() {
            Ok(mut conn) => {
                match redis::cmd("CONFIG").arg("GET").arg("databases").query::<Vec<String>>(&mut conn) {
                    Ok(config) => {
                        if config.len() >= 2 {
                            config[1].parse::<usize>().map_err(|e| e.to_string())
                        } else {
                            Ok(16) // 默认16个数据库
                        }
                    }
                    Err(e) => Err(format!("获取数据库数量失败: {}", e)),
                }
            }
            Err(e) => Err(format!("获取连接失败: {}", e)),
        }
    } else {
        Err("Redis 未连接".to_string())
    }
}

// 获取数据库键数量
#[tauri::command]
pub async fn get_db_key_count(
    connection_id: String,
    db_index: u8,
    state: State<'_, AppState>,
) -> Result<usize, String> {
    let connections = state.connections.lock().unwrap();

    if let Some(conn_state) = connections.get(&connection_id) {
        match conn_state.client.get_connection() {
            Ok(mut conn) => {
                let original_db = conn_state.current_db;

                let _: () = redis::cmd("SELECT")
                    .arg(db_index)
                    .query(&mut conn)
                    .map_err(|e| format!("切换数据库失败: {}", e))?;

                // 获取键数量
                let result = redis::cmd("DBSIZE").query::<usize>(&mut conn);

                if original_db != db_index {
                    let _ = redis::cmd("SELECT")
                        .arg(original_db)
                        .query::<()>(&mut conn)
                        .ok();
                }

                match result {
                    Ok(count) => Ok(count),
                    Err(e) => Err(format!("获取键数量失败: {}", e)),
                }
            }
            Err(e) => Err(format!("获取连接失败: {}", e)),
        }
    } else {
        Err("Redis 未连接".to_string())
    }
}

// 批量获取所有数据库的键数量
#[tauri::command]
pub async fn get_all_db_key_counts(
    connection_id: String,
    db_count: usize,
    state: State<'_, AppState>,
) -> Result<Vec<(u8, usize)>, String> {
    let connections = state.connections.lock().unwrap();

    if let Some(conn_state) = connections.get(&connection_id) {
        match conn_state.client.get_connection() {
            Ok(mut conn) => {
                let original_db = conn_state.current_db;
                let mut results = Vec::new();

                // 遍历所有数据库
                for db_index in 0..db_count {
                    // 切换到指定数据库
                    if let Err(_e) = redis::cmd("SELECT")
                        .arg(db_index)
                        .query::<()>(&mut conn)
                    {
                        // 如果切换失败，记录为0
                        results.push((db_index as u8, 0));
                        continue;
                    }

                    // 获取键数量
                    match redis::cmd("DBSIZE").query::<usize>(&mut conn) {
                        Ok(count) => results.push((db_index as u8, count)),
                        Err(_) => results.push((db_index as u8, 0)),
                    }
                }

                // 切换回原来的数据库
                if original_db < db_count as u8 {
                    let _ = redis::cmd("SELECT")
                        .arg(original_db)
                        .query::<()>(&mut conn)
                        .ok();
                }

                Ok(results)
            }
            Err(e) => Err(format!("获取连接失败: {}", e)),
        }
    } else {
        Err("Redis 未连接".to_string())
    }
}

// 切换数据库
#[tauri::command]
pub async fn select_db(
    connection_id: String,
    db_index: u8,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut connections = state.connections.lock().unwrap();

    if let Some(conn_state) = connections.get_mut(&connection_id) {
        match conn_state.client.get_connection() {
            Ok(mut conn) => {
                match redis::cmd("SELECT").arg(db_index).query::<()>(&mut conn) {
                    Ok(_) => {
                        conn_state.current_db = db_index;
                        Ok(())
                    }
                    Err(e) => Err(format!("切换数据库失败: {}", e)),
                }
            }
            Err(e) => Err(format!("获取连接失败: {}", e)),
        }
    } else {
        Err("Redis 未连接".to_string())
    }
}
