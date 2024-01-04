use tauri::Manager;

use self::verify::get_verify_signature;

mod install;
pub mod lib;
pub mod logger;
pub mod verify;
pub mod window;

// 生命周期
pub fn public_setup<F, S, A>(app: &mut tauri::App, flrst: F, start_app: S, active: A)
where
    F: FnOnce(&mut tauri::App),
    S: FnOnce(&mut tauri::App),
    A: FnOnce(&mut tauri::App),
{
    // 注册日志监听
    logger::configure_logging(app.handle().clone());

    if get_verify_signature(&app.handle()) {
        println!("激活成功");
        active(app);
        window::app_ready(app.handle().clone());
    } else {
        println!("激活失败");
        if let Some(activate_window) = app.get_window("activate") {
            activate_window.show().unwrap();
        }
    }

    let app_config = app.config();

    // 应用激活后 启动安装
    if install::is_first_run(&app_config) {
        flrst(app);
        // 安装标识
        install::create_app_data_flag(&app_config, "installed.flag")
            .expect("failed to create installation flag");
    } else {
        start_app(app)
    }
}
