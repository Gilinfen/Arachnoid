use tauri::{AppHandle, Manager};

use super::verify::get_verify_signature;

#[tauri::command]
pub fn app_ready(app: AppHandle) {
    if get_verify_signature(&app) {
        // 显示主窗口
        if let Some(main_window) = app.get_window("main") {
            main_window.show().unwrap();
        }
    }
}
