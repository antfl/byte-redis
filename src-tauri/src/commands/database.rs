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
                // 切换到指定数据库
                if conn_state.current_db != db_index {
                    let _: () = redis::cmd("SELECT")
                        .arg(db_index)
                        .query(&mut conn)
                        .map_err(|e| format!("切换数据库失败: {}", e))?;
                }

                // 获取键数量
                match redis::cmd("DBSIZE").query::<usize>(&mut conn) {
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
