/**
 * 与 `src-tauri/src/window_layout.rs` 中算法保持一致，供前端离线预览与单测。
 * 坐标均为整型像素语义（调用方应传入整数）。
 */
export function clampWindowPosition(
	posX: number,
	posY: number,
	winW: number,
	winH: number,
	areaX: number,
	areaY: number,
	areaW: number,
	areaH: number
): { x: number; y: number } {
	const win_w = Math.max(1, winW | 0);
	const win_h = Math.max(1, winH | 0);
	const area_w = Math.max(1, areaW | 0);
	const area_h = Math.max(1, areaH | 0);
	const ax = areaX | 0;
	const ay = areaY | 0;

	const max_x = ax + area_w - win_w;
	const max_y = ay + area_h - win_h;

	const x = max_x < ax ? ax : Math.min(Math.max(posX | 0, ax), max_x);
	const y = max_y < ay ? ay : Math.min(Math.max(posY | 0, ay), max_y);

	return { x, y };
}
