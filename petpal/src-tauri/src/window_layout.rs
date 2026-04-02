//! window-shell：将窗口左上角钳制在显示器工作区内（纯函数，可单测）。
//!
//! 当窗口大于工作区时，按 SRS/架构约定在对应轴上贴齐工作区左上角（与 `area_x`/`area_y` 对齐）。

use serde::{Deserialize, Serialize};

/// 工作区钳制请求（将窗口左上角钳制到显示器工作区内）。
///
/// 将参数打包为结构体是为了减少函数签名参数数量，避免 `clippy::too_many_arguments`。
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ClampWindowPositionRequest {
    pub pos_x: i32,
    pub pos_y: i32,
    pub win_w: u32,
    pub win_h: u32,
    pub area_x: i32,
    pub area_y: i32,
    pub area_w: u32,
    pub area_h: u32,
}

/// 将窗口左上角 `(pos_x, pos_y)` 钳制在矩形工作区内；窗口外沿尺寸为 `win_w`×`win_h`。
/// `0` 宽高会按 `1` 处理，避免除零。
pub fn clamp_window_position(req: ClampWindowPositionRequest) -> (i32, i32) {
    let win_w = req.win_w.max(1) as i64;
    let win_h = req.win_h.max(1) as i64;
    let area_w = req.area_w.max(1) as i64;
    let area_h = req.area_h.max(1) as i64;
    let ax = req.area_x as i64;
    let ay = req.area_y as i64;

    let max_x = ax + area_w - win_w;
    let max_y = ay + area_h - win_h;

    let x = if max_x < ax {
        ax as i32
    } else {
        (req.pos_x as i64).clamp(ax, max_x) as i32
    };

    let y = if max_y < ay {
        ay as i32
    } else {
        (req.pos_y as i64).clamp(ay, max_y) as i32
    };

    (x, y)
}
