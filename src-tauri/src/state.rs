use redis::Client;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// 定义 Redis 连接配置
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct RedisConnectionConfig {
    pub id: String,
    pub name: String,
    pub host: String,
    pub port: u16,
    pub username: Option<String>,
    pub password: Option<String>,
    pub db: Option<u8>,
}

// 定义连接状态
pub struct ConnectionState {
    pub client: Client,
    pub current_db: u8,
}

// 定义应用状态
pub struct AppState {
    pub connections: Arc<Mutex<HashMap<String, ConnectionState>>>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            connections: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}
