#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod state;

use state::AppState;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            commands::connection::connect_redis,
            commands::connection::disconnect_redis,
            commands::keys::set_key,
            commands::keys::get_key,
            commands::keys::get_keys,
            commands::keys::get_keys_with_details,
            commands::keys::get_key_type,
            commands::keys::get_key_ttl,
            commands::keys::set_key_ttl,
            commands::keys::get_key_size,
            commands::keys::delete_key,
            commands::database::get_db_count,
            commands::database::get_db_key_count,
            commands::database::select_db,
            commands::server::get_memory_usage,
            commands::server::get_connections,
            commands::server::get_hit_rate,
            commands::server::get_uptime
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
