use tauri::{AppHandle, Manager};

pub fn app_ready(app: AppHandle) {
    // 关闭加载窗口
    if let Some(loading_window) = app.get_window("activate") {
        loading_window.close().unwrap();
    }

    // 显示主窗口
    if let Some(main_window) = app.get_window("main") {
        main_window.show().unwrap();
    }
}
