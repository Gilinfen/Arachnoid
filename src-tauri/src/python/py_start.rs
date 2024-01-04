use crate::{public::lib::run_command, utils::resolve_resource_path};

#[tauri::command]
pub fn execute_python_script(cmds: Vec<String>) -> Result<String, String> {
    let py_path = resolve_resource_path("../python/out/main.bin");

    // 将 `Vec<String>` 转换为 `Vec<&str>` 以满足 `run_command` 函数的参数要求
    let cmd_refs: Vec<&str> = cmds.iter().map(|s| s.as_str()).collect();

    run_command(&py_path, &cmd_refs, None)
}
