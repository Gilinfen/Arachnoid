use crate::{public::lib::run_command, utils::resolve_resource_path};

#[tauri::command]
pub async fn execute_python_script(cmd_type: &str) -> Result<String, String> {
    let py_path = resolve_resource_path("../python/dist/main/main");

    println!("{}", &py_path);

    run_command(&py_path, &["app1"], None).await
}
