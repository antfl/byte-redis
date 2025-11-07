use crate::state::AppState;
use crate::commands::response::Response;
use tauri::State;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct DbKeyCount {
    pub db_index: u8,
    pub key_count: usize,
}

// 获取数据库数量
#[tauri::command]
pub async fn get_db_count(
    connection_id: String,
    state: State<'_, AppState>,
) -> Result<Response<usize>, String> {
    let connections = state.connections.lock().unwrap();

    if let Some(conn_state) = connections.get(&connection_id) {
        match conn_state.client.get_connection() {
            Ok(mut conn) => {
                match redis::cmd("CONFIG").arg("GET").arg("databases").query::<Vec<String>>(&mut conn) {
                    Ok(config) => {
                        let count = if config.len() >= 2 {
                            config[1].parse::<usize>().unwrap_or(16)
                        } else {
                            16 // 默认16个数据库
                        };
                        Ok(Response::success(count))
                    }
                    Err(e) => Ok(Response::error(format!("获取数据库数量失败: {}", e))),
                }
            }
            Err(e) => Ok(Response::error(format!("获取连接失败: {}", e))),
        }
    } else {
        Ok(Response::error("Redis 未连接".to_string()))
    }
}

// 获取数据库键数量
#[tauri::command]
pub async fn get_db_key_count(
    connection_id: String,
    db_index: u8,
    state: State<'_, AppState>,
) -> Result<Response<usize>, String> {
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
                    Ok(count) => Ok(Response::success(count)),
                    Err(e) => Ok(Response::error(format!("获取键数量失败: {}", e))),
                }
            }
            Err(e) => Ok(Response::error(format!("获取连接失败: {}", e))),
        }
    } else {
        Ok(Response::error("Redis 未连接".to_string()))
    }
}

// 批量获取所有数据库的键数量
#[tauri::command]
pub async fn get_all_db_key_counts(
    connection_id: String,
    db_count: usize,
    state: State<'_, AppState>,
) -> Result<Response<Vec<DbKeyCount>>, String> {
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
                        results.push(DbKeyCount {
                            db_index: db_index as u8,
                            key_count: 0,
                        });
                        continue;
                    }

                    // 获取键数量
                    let count = match redis::cmd("DBSIZE").query::<usize>(&mut conn) {
                        Ok(count) => count,
                        Err(_) => 0,
                    };
                    results.push(DbKeyCount {
                        db_index: db_index as u8,
                        key_count: count,
                    });
                }

                // 切换回原来的数据库
                if original_db < db_count as u8 {
                    let _ = redis::cmd("SELECT")
                        .arg(original_db)
                        .query::<()>(&mut conn)
                        .ok();
                }

                Ok(Response::success(results))
            }
            Err(e) => Ok(Response::error(format!("获取连接失败: {}", e))),
        }
    } else {
        Ok(Response::error("Redis 未连接".to_string()))
    }
}

// 切换数据库
#[tauri::command]
pub async fn select_db(
    connection_id: String,
    db_index: u8,
    state: State<'_, AppState>,
) -> Result<Response<()>, String> {
    let mut connections = state.connections.lock().unwrap();

    if let Some(conn_state) = connections.get_mut(&connection_id) {
        match conn_state.client.get_connection() {
            Ok(mut conn) => {
                match redis::cmd("SELECT").arg(db_index).query::<()>(&mut conn) {
                    Ok(_) => {
                        conn_state.current_db = db_index;
                        Ok(Response::<()>::success_empty_with_message(format!("成功切换到数据库 {}", db_index)))
                    }
                    Err(e) => Ok(Response::error(format!("切换数据库失败: {}", e))),
                }
            }
            Err(e) => Ok(Response::error(format!("获取连接失败: {}", e))),
        }
    } else {
        Ok(Response::error("Redis 未连接".to_string()))
    }
}
