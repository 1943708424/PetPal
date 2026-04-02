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

#[cfg(test)]
mod tests {
    use super::clamp_window_to_work_area;
    use crate::window_layout::ClampWindowPositionRequest;

    #[test]
    fn command_clamps_right_edge() {
        let req = ClampWindowPositionRequest {
            pos_x: 900,
            pos_y: 10,
            win_w: 100,
            win_h: 80,
            area_x: 0,
            area_y: 0,
            area_w: 800,
            area_h: 600,
        };
        let (x, y) = clamp_window_to_work_area(req);
        assert_eq!((x, y), (700, 10));
    }

    #[test]
    fn command_handles_small_work_area() {
        let req = ClampWindowPositionRequest {
            pos_x: 30,
            pos_y: 40,
            win_w: 500,
            win_h: 400,
            area_x: 100,
            area_y: 100,
            area_w: 200,
            area_h: 150,
        };
        let (x, y) = clamp_window_to_work_area(req);
        assert_eq!((x, y), (100, 100));
    }
}
