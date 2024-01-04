// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use std::env;

use public::{lib, verify, window};
use python::{chorme_v, py_start};

mod config;
mod globalstate;
mod public;
mod python;
mod utils;
use dotenv::dotenv;

fn main() {
    dotenv().ok(); // 加载 .env 文件

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            chorme_v::download_chromedriver,
            py_start::execute_python_script,
            config::update_json_command,
            config::read_json_command,
            config::get_os_info,
            verify::use_verify_signature,
            lib::close_app,
            window::app_ready,
        ])
        .setup(|app: &mut tauri::App| {
            // 保存 app 为全局变量
            globalstate::APP_HANDLE
                .set(app.handle().clone())
                .expect("Failed to set app handle");

            public::public_setup(
                app,
                |val1| {
                    println!("首次执行安装");
                    // 初始化 settings
                    config::init_settings(&val1.handle());
                },
                |val2| {
                    println!("应用启动");
                    config::init_settings(&val2.handle());
                    // config::init_settings(&app.handle());
                },
                |_| {
                    println!("激活成功");
                },
            );
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
