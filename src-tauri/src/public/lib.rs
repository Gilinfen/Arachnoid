use std::{
    fs::{self, File},
    io::{self, Read},
    path::PathBuf,
    process::Command,
    str,
};

use log::info;
use serde::{de::DeserializeOwned, Serialize};
use tauri::api::path::app_data_dir;
use tauri::AppHandle;

use crate::utils::resolve_resource_path;

use super::zip::unzip_file;

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

/// 运行终端指令
pub async fn run_command(
    command: &str,
    args: &[&str],
    res_dir: Option<&str>,
) -> Result<String, String> {
    let mut command = Command::new(command);
    command.args(args);

    if let Some(dir) = res_dir {
        command.current_dir(dir);
    }

    let output = command.output();

    match output {
        Ok(o) => {
            if o.status.success() {
                let output_str = match str::from_utf8(&o.stdout) {
                    Ok(s) => s.trim().to_string(),
                    Err(_) => "Failed to parse output".to_string(),
                };
                info!("SUCCESS: {}", output_str);
                Ok(output_str)
            } else {
                let err_str = match str::from_utf8(&o.stderr) {
                    Ok(s) => s.trim().to_string(),
                    Err(_) => "Unknown error".to_string(),
                };
                info!("ERROR: {}", err_str);
                Err(err_str)
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

/// 通用的函数，允许传入不同的命令
// pub async fn find_command_path(command_name: &str) -> Result<String, String> {
//     let command = if cfg!(target_os = "windows") {
//         "where"
//     } else {
//         "which"
//     };

//     run_command(command, &[command_name], None).await
// }

// 解压 python 可执行文件
pub async fn unzip_python() -> Result<(), Box<dyn std::error::Error>> {
    // 使用示例
    let red_dir = resolve_resource_path("../");
    let unzip_dst = red_dir.clone() + "/pydist.zip"; // 解压目标目录
    let zip_dir = red_dir.clone() + "/";
    info!("unzip_dst:{}", unzip_dst);
    info!("zip_dir:{}", zip_dir);
    if let Err(e) = unzip_file(&unzip_dst, &zip_dir) {
        info!("Error decompressing file: {}", e);
        return Ok(());
    }
    let test_python = zip_dir + "/python/dist/main/main";
    let _ = run_command("chmod", &["+x", &test_python], None).await?;
    let _ = run_command(&test_python, &[""], None).await?;
    Ok(())
}
