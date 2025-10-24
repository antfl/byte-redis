use crate::state::AppState;
use serde::Serialize;
use tauri::State;

#[derive(Debug, Serialize)]
pub struct RedisServerInfo {
    // 基础指标
    memory_usage: usize,
    maxmemory: usize,
    connections: usize,
    hit_rate: f64,
    uptime: u64,
    total_keys: usize,
    ops_per_sec: u64,
    used_cpu: f64,
    role: String,
    version: String,
    persistence: String,

    // 扩展指标
    clients_blocked: usize,
    keys_evicted: usize,
    keys_expired: usize,
    replication_status: String,
    mem_fragmentation_ratio: f64,
    aof_size: usize,
    rdb_last_save: u64,
    connected_slaves: usize,
}

#[tauri::command]
pub async fn get_redis_server_info(
    connection_id: String,
    state: State<'_, AppState>,
) -> Result<RedisServerInfo, String> {
    let connections = state.connections.lock().unwrap();
    let conn_state = connections
        .get(&connection_id)
        .ok_or("Redis 未连接".to_string())?;

    let mut conn = conn_state
        .client
        .get_connection()
        .map_err(|e| format!("获取连接失败: {}", e))?;

    // 一次性获取所有INFO信息
    let info: String = redis::cmd("INFO")
        .query(&mut conn)
        .map_err(|e| format!("获取Redis信息失败: {}", e))?;

    // 解析所有需要的信息
    Ok(RedisServerInfo {
        // 原有指标
        memory_usage: parse_info_value(&info, "used_memory").unwrap_or(0),
        maxmemory: parse_info_value(&info, "maxmemory").unwrap_or(0),
        connections: parse_info_value(&info, "connected_clients").unwrap_or(0),
        hit_rate: calculate_hit_rate(&info),
        uptime: parse_info_value(&info, "uptime_in_seconds").unwrap_or(0),
        total_keys: calculate_total_keys(&info),
        ops_per_sec: parse_info_value(&info, "instantaneous_ops_per_sec").unwrap_or(0),
        used_cpu: parse_info_value(&info, "used_cpu_sys").unwrap_or(0.0),
        role: parse_info_string(&info, "role").unwrap_or_else(|| "unknown".into()),
        version: parse_info_string(&info, "redis_version").unwrap_or_else(|| "unknown".into()),
        persistence: get_persistence_status(&info),

        // 新增指标
        clients_blocked: parse_info_value(&info, "blocked_clients").unwrap_or(0),
        keys_evicted: parse_info_value(&info, "evicted_keys").unwrap_or(0),
        keys_expired: parse_info_value(&info, "expired_keys").unwrap_or(0),
        replication_status: get_replication_status(&info),
        mem_fragmentation_ratio: parse_info_value(&info, "mem_fragmentation_ratio").unwrap_or(0.0),
        aof_size: parse_info_value(&info, "aof_current_size").unwrap_or(0),
        rdb_last_save: parse_info_value(&info, "rdb_last_save_time").unwrap_or(0),
        connected_slaves: parse_info_value(&info, "connected_slaves").unwrap_or(0),
    })
}

// 辅助函数：解析数值类型的信息
fn parse_info_value<T: std::str::FromStr>(info: &str, key: &str) -> Option<T> {
    info.lines()
        .find(|line| line.starts_with(key))
        .and_then(|line| line.split(':').nth(1))
        .and_then(|val| val.trim().parse().ok())
}

// 辅助函数：解析字符串类型的信息
fn parse_info_string(info: &str, key: &str) -> Option<String> {
    info.lines()
        .find(|line| line.starts_with(key))
        .and_then(|line| line.split(':').nth(1))
        .map(|val| val.trim().to_string())
}

// 计算缓存命中率
fn calculate_hit_rate(info: &str) -> f64 {
    let hits: u64 = parse_info_value(info, "keyspace_hits").unwrap_or(0);
    let misses: u64 = parse_info_value(info, "keyspace_misses").unwrap_or(0);
    let total = hits + misses;

    if total > 0 {
        (hits as f64 / total as f64 * 100.0).round()
    } else {
        0.0
    }
}

// 计算数据库键总数
fn calculate_total_keys(info: &str) -> usize {
    info.lines()
        .filter(|line| line.starts_with("db"))
        .filter_map(|line| {
            line.split(',')
                .find(|part| part.starts_with("keys="))
                .and_then(|part| part.split('=').nth(1))
                .and_then(|num| num.parse::<usize>().ok())
        })
        .sum()
}

// 获取持久化状态
fn get_persistence_status(info: &str) -> String {
    let rdb_enabled = parse_info_value::<u8>(info, "rdb_bgsave_in_progress").unwrap_or(0) == 1;
    let aof_enabled = parse_info_value::<u8>(info, "aof_enabled").unwrap_or(0) == 1;

    match (rdb_enabled, aof_enabled) {
        (true, true) => "RDB+AOF".into(),
        (true, false) => "RDB".into(),
        (false, true) => "AOF".into(),
        _ => "None".into(),
    }
}

// 获取复制状态
fn get_replication_status(info: &str) -> String {
    let role = parse_info_string(info, "role").unwrap_or_default();
    let master_link_status = parse_info_string(info, "master_link_status").unwrap_or_default();

    match role.as_str() {
        "master" => "主节点".to_string(),
        "slave" => {
            if master_link_status == "up" {
                "从节点 (已连接)".to_string()
            } else {
                "从节点 (未连接)".to_string()
            }
        }
        _ => "未知".to_string(),
    }
}
