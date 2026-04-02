//! window-shell 模块：窗口在工作区内的位置钳制（集成测试，对应架构「边界钳制」）。

use petpal_lib::window_layout::{clamp_window_position, ClampWindowPositionRequest};

#[test]
fn clamps_when_overflowing_right_edge() {
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
    let (x, y) = clamp_window_position(req);
    assert_eq!(x, 700);
    assert_eq!(y, 10);
}

#[test]
fn clamps_when_overflowing_left_and_top() {
    let req = ClampWindowPositionRequest {
        pos_x: -50,
        pos_y: -20,
        win_w: 120,
        win_h: 100,
        area_x: 0,
        area_y: 0,
        area_w: 800,
        area_h: 600,
    };
    let (x, y) = clamp_window_position(req);
    assert_eq!(x, 0);
    assert_eq!(y, 0);
}

#[test]
fn pins_to_work_area_origin_when_window_wider_than_area() {
    let req = ClampWindowPositionRequest {
        pos_x: 100,
        pos_y: 50,
        win_w: 900,
        win_h: 80,
        area_x: 0,
        area_y: 0,
        area_w: 800,
        area_h: 600,
    };
    let (x, y) = clamp_window_position(req);
    assert_eq!(x, 0);
    assert_eq!(y, 50);
}

#[test]
fn pins_to_work_area_origin_when_window_taller_than_area() {
    let req = ClampWindowPositionRequest {
        pos_x: 10,
        pos_y: 100,
        win_w: 100,
        win_h: 700,
        area_x: 0,
        area_y: 0,
        area_w: 800,
        area_h: 600,
    };
    let (x, y) = clamp_window_position(req);
    assert_eq!(x, 10);
    assert_eq!(y, 0);
}

#[test]
fn treats_zero_window_size_as_one_for_clamping() {
    let req = ClampWindowPositionRequest {
        pos_x: 2000,
        pos_y: 3000,
        win_w: 0,
        win_h: 0,
        area_x: 0,
        area_y: 0,
        area_w: 800,
        area_h: 600,
    };
    let (x, y) = clamp_window_position(req);
    assert_eq!(x, 799);
    assert_eq!(y, 599);
}

#[test]
fn respects_non_zero_work_area_offset() {
    let req = ClampWindowPositionRequest {
        pos_x: 0,
        pos_y: 0,
        win_w: 100,
        win_h: 80,
        area_x: 100,
        area_y: 100,
        area_w: 800,
        area_h: 600,
    };
    let (x, y) = clamp_window_position(req);
    assert_eq!(x, 100);
    assert_eq!(y, 100);
}
