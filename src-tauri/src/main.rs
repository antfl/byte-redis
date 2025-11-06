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
            commands::keys::rename_key,
            commands::keys::set_key,
            commands::keys::add_set_item,
            commands::keys::get_key_detail,
            commands::keys::delete_set_item,
            commands::keys::delete_key,
            commands::keys::add_zset_item,
            commands::keys::append_list_item,
            commands::keys::update_hash_field,
            commands::keys::set_key_ttl,
            commands::keys::update_list_item,
            commands::keys::delete_list_item,
            commands::import_export::export_key,
            commands::import_export::export_keys,
            commands::import_export::import_key,
            commands::import_export::import_keys,
            commands::database::get_db_count,
            commands::database::get_db_key_count,
            commands::database::get_all_db_key_counts,
            commands::database::select_db,
            commands::server::get_redis_server_info,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
