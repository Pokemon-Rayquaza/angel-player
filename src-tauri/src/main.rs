#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::pick_folder,
            commands::scan_flac_folder,
            commands::scan_default_music,
            commands::pick_program,
            commands::launch_program,
            commands::open_in_explorer,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Angel Player");
}