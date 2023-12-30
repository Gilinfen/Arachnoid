use std::{
    fs::{self, File},
    io::{self, Read},
    path::PathBuf,
};

use base64::{engine::general_purpose, Engine as _};
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

// pub fn get_resource_path(app_handle: &AppHandle, resource_path: &str) -> String {
//     // 生产路径
//     app_handle
//         .path_resolver()
//         .resolve_resource(resource_path)
//         .expect("failed to resolve resource")
//         .to_str()
//         .unwrap()
//         .to_string()
// }

// 获取授权公钥匙
pub const ACCREDIT_PUBLIC_KEY_PEM: &str = r#"-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAs2EL0Qo4czxE4kPSRX7P
1j4nIoDY3wjz1N4o+DpBuGyF5qkT3eWOjP8wrvJQ71Y04GJ6Cv7tZBMRXbKGkuPA
YaLFPb0u8I6lQt0MC23SMzzHiFBUSGOBfJPk4vFqD10z9wDz57nNPL0XsYkPZYOY
+Uny1nZezrBF0mEMd2U0EEYGeKJJnm9kN7J807hlYr2tQLbfFD8fBdgj+a8sNxCp
Cb+eStom+1nPBJ78kaGXvQUYgr3GzKbR/6MEiZvyAoX+9+fN2/YnPO7wA91ruy7Y
5WIj9nT1ar1YiUL/edF72eY/Ah4b45NPUVwu069zrJNQeXwwId79sZz/bgQedPPR
FQIDAQAB
-----END PUBLIC KEY-----"#;

pub fn decode_str(base_code: String) -> String {
    let decoded_bytes = general_purpose::STANDARD_NO_PAD
        .decode(&base_code)
        .expect("解码失败");

    let pem_str = std::str::from_utf8(&decoded_bytes);

    match pem_str {
        Ok(p) => p.to_owned(),
        Err(e) => format!("无法转换为UTF-8字符串：{}", e),
    }
}
