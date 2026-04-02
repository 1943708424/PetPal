/** 与架构设计 §4.1 一致：发往原生层的窗口意图（拖动结束后可结合钳制结果）。 */
export interface WindowLayoutIntent {
	x: number;
	y: number;
	alwaysOnTop: boolean;
}
