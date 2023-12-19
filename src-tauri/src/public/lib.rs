use std::{
    fs::{self, File},
    io::{self, Read},
    path::PathBuf,
};

use serde::{de::DeserializeOwned, Serialize};
use tauri::api::path::app_data_dir;
use tauri::AppHandle;

/// 读取 json
pub fn read_json<T: DeserializeOwned>(file_path: &str) -> io::Result<T> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let data: T = serde_json::from_str(&contents)?;
    Ok(data)
}

/// 修改 json
pub fn update_json<T: Serialize>(file_path: &str, data: &T) -> io::Result<()> {
    let contents = serde_json::to_string(data)?;
    fs::write(file_path, contents)?;
    Ok(())
}

pub fn get_app_data_dir(app_handle: &AppHandle) -> PathBuf {
    app_data_dir(&app_handle.config()).expect("msg")
}

pub fn get_resource_path(app_handle: &AppHandle, resource_path: &str) -> String {
    // 生产路径
    app_handle
        .path_resolver()
        .resolve_resource(resource_path)
        .expect("failed to resolve resource")
        .to_str()
        .unwrap()
        .to_string()
}
