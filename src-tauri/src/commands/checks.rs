use crate::types::ScanResult;

#[tauri::command]
pub async fn run_readiness_scan() -> Result<ScanResult, String> {
    // COM requires STA, so run on a dedicated blocking thread
    tokio::task::spawn_blocking(|| {
        crate::checks::run_all_checks().map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| format!("Task join error: {}", e))?
}
