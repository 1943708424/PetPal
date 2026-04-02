//! window-shell 模块：窗口在工作区内的位置钳制（集成测试，对应架构「边界钳制」）。

use petpal_lib::window_layout::clamp_window_position;

#[test]
fn clamps_when_overflowing_right_edge() {
    let (x, y) = clamp_window_position(900, 10, 100, 80, 0, 0, 800, 600);
    assert_eq!(x, 700);
    assert_eq!(y, 10);
}

#[test]
fn clamps_when_overflowing_left_and_top() {
    let (x, y) = clamp_window_position(-50, -20, 120, 100, 0, 0, 800, 600);
    assert_eq!(x, 0);
    assert_eq!(y, 0);
}

#[test]
fn pins_to_work_area_origin_when_window_wider_than_area() {
    let (x, y) = clamp_window_position(100, 50, 900, 80, 0, 0, 800, 600);
    assert_eq!(x, 0);
    assert_eq!(y, 50);
}

#[test]
fn pins_to_work_area_origin_when_window_taller_than_area() {
    let (x, y) = clamp_window_position(10, 100, 100, 700, 0, 0, 800, 600);
    assert_eq!(x, 10);
    assert_eq!(y, 0);
}

#[test]
fn treats_zero_window_size_as_one_for_clamping() {
    let (x, y) = clamp_window_position(2000, 3000, 0, 0, 0, 0, 800, 600);
    assert_eq!(x, 799);
    assert_eq!(y, 599);
}

#[test]
fn respects_non_zero_work_area_offset() {
    let (x, y) = clamp_window_position(0, 0, 100, 80, 100, 100, 800, 600);
    assert_eq!(x, 100);
    assert_eq!(y, 100);
}
