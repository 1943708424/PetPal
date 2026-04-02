// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

pub mod window_layout;

use window_layout::{clamp_window_position, ClampWindowPositionRequest};

/// Tauri 命令：供前端在拖动结束后将意图坐标钳制到工作区（与架构 `WindowLayoutIntent` 配合使用）。
#[tauri::command]
fn clamp_window_to_work_area(req: ClampWindowPositionRequest) -> (i32, i32) {
    clamp_window_position(req)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![clamp_window_to_work_area])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
