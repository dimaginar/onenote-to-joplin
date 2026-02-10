mod commands;
mod checks;
mod types;

use commands::{checks as check_cmds, report};

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            check_cmds::run_readiness_scan,
            report::generate_report,
            report::save_report,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
