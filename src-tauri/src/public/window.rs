use tauri::{AppHandle, Manager};

#[tauri::command]
pub fn app_ready(app: AppHandle) {
    // 显示主窗口
    if let Some(main_window) = app.get_window("main") {
        main_window.show().unwrap();
    }
}
