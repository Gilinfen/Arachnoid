use crate::{
    public::lib::{read_json, update_json},
    utils::resolve_resource_path,
};
use log::info;
use tauri::AppHandle;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct JsonData {
    /// 资源目录
    pub res_dir: String,
    /// 系统信息
    pub os_info: String,
    /// chromedriver 路径
    pub chromedriver: String,
}

#[tauri::command]
pub fn read_json_command() -> Result<JsonData, String> {
    let file_path = resolve_resource_path("../settings.json");
    read_json::<JsonData>(&file_path).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_json_command(data: JsonData) -> Result<(), String> {
    let file_path = resolve_resource_path("../settings.json");
    update_json(&file_path, &data).map_err(|e| e.to_string())
}

/// 获取系统信息
#[tauri::command]
pub fn get_os_info() -> &'static str {
    match (std::env::consts::OS, std::env::consts::ARCH) {
        ("macos", "aarch64") => "Mac_Arm",
        ("macos", _) => "Mac",
        ("windows", "x86_64") => "Win64",
        ("windows", "x86") => "Win",
        _ => "unknown",
    }
}

/// 初始化 setting
pub fn init_settings(app: &AppHandle) {
    // 设置 os info
    let os_val: String = get_os_info().to_string();
    match read_json_command() {
        Ok(mut settings_data) => {
            // 生产资源路径
            let res_dir = app
                .path_resolver()
                .resolve_resource("../")
                .expect("failed to resolve resource")
                .to_str()
                .unwrap()
                .to_string();
            // 成功获取 settings_data，现在可以修改它
            settings_data.os_info = os_val;
            settings_data.res_dir = res_dir;

            let _ = update_json_command(settings_data);
        }
        Err(e) => {
            // 处理读取 JSON 数据时的错误
            info!("Error reading settings data: {}", e);
        }
    }
}
