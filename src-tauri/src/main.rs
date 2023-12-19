// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use log::info;
use public::verify;
use python::{chorme_v, pystart};

mod config;
mod globalstate;
mod public;
mod python;
mod utils;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            pystart::execute_python_script,
            pystart::init_python_path,
            chorme_v::get_chrome_version_command,
            chorme_v::download_chromedriver,
            config::update_json_command,
            config::read_json_command,
            config::get_os_info,
            verify::use_verify_signature,
        ])
        .setup(|app: &mut tauri::App| {
            // 保存 app 为全局变量
            globalstate::APP_HANDLE
                .set(app.handle().clone())
                .expect("Failed to set app handle");

            public::public_setup(
                app,
                |val1| {
                    info!("首次执行安装");
                    // 初始化 settings
                    config::init_settings(&val1.handle());
                },
                |_| {
                    info!("应用启动");
                    // config::init_settings(&app.handle());
                    let _ = pystart::activate_python_venv();
                },
            );
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
