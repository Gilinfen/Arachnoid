use crate::globalstate::APP_HANDLE;
use std::str;

/// 路径转换
pub fn resolve_resource_path(resource_path: &str) -> String {
    // let app_handle = APP_HANDLE.get().expect("AppHandle not set");
    let app_handle: &tauri::AppHandle = APP_HANDLE.get().expect("全局 Tauri App 访问失败");
    // 生产路径
    app_handle
        .path_resolver()
        .resolve_resource(resource_path)
        .expect("failed to resolve resource")
        .to_str()
        .unwrap()
        .to_string()
}
