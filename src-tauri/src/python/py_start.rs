use log::info;
use std::{fs, thread};
use tokio::runtime::Runtime;

use crate::{
    public::{lib::run_command, zip::unzip_file},
    utils::resolve_resource_path,
};

// 解压 script 可执行文件
pub fn unzip_python() -> Result<(), Box<dyn std::error::Error>> {
    // 使用示例
    let red_dir = resolve_resource_path("../");
    let unzip_dst = red_dir.clone() + "/pydist.zip"; // 解压目标目录
    let zip_dir = red_dir.clone() + "/script";
    info!("unzip_dst:{}", unzip_dst);
    info!("zip_dir:{}", zip_dir);
    if let Err(e) = unzip_file(&unzip_dst, &zip_dir) {
        info!("Error decompressing file: {}", e);
        return Ok(());
    }
    // 删除
    let _ = fs::remove_file(unzip_dst);

    let test_python = format!("{}/dist/main/main", zip_dir);
    let _ = run_command("chmod", &["+x", &test_python], None).expect("Command execution failure");

    thread::spawn(move || {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            // 在这里写你的异步代码
            let _ = run_command(&test_python, &[""], None);
        });
    });
    Ok(())
}

#[tauri::command]
pub fn execute_python_script(cmds: Vec<String>) -> Result<String, String> {
    let py_path = if cfg!(target_os = "windows") {
        resolve_resource_path("../script/dist/main/main.exe")
    } else {
        resolve_resource_path("../script/dist/main/main")
    };

    // 将 `Vec<String>` 转换为 `Vec<&str>` 以满足 `run_command` 函数的参数要求
    let cmd_refs: Vec<&str> = cmds.iter().map(|s| s.as_str()).collect();

    run_command(&py_path, &cmd_refs, None)
}
