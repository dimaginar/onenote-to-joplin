mod commands;
mod checks;
mod types;

use commands::{checks as check_cmds, report};
use tauri::Manager;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let _webview = app.get_webview_window("main")
                .expect("no main window");
            #[cfg(target_os = "windows")]
            _webview.with_webview(|wv: tauri::webview::PlatformWebview| unsafe {
                wv.controller()
                    .CoreWebView2().unwrap()
                    .Settings().unwrap()
                    .SetAreDefaultContextMenusEnabled(false).unwrap();
            }).unwrap();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            check_cmds::run_readiness_scan,
            report::generate_report,
            report::save_report,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
