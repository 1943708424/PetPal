import { describe, expect, it } from "vitest";
import { clampWindowPosition } from "./window-layout";

describe("clampWindowPosition", () => {
	it("clamps when overflowing right edge", () => {
		const r = clampWindowPosition(900, 10, 100, 80, 0, 0, 800, 600);
		expect(r).toEqual({ x: 700, y: 10 });
	});

	it("clamps when overflowing left and top", () => {
		const r = clampWindowPosition(-50, -20, 120, 100, 0, 0, 800, 600);
		expect(r).toEqual({ x: 0, y: 0 });
	});

	it("pins when window wider than work area", () => {
		const r = clampWindowPosition(100, 50, 900, 80, 0, 0, 800, 600);
		expect(r).toEqual({ x: 0, y: 50 });
	});

	it("pins when window taller than work area", () => {
		const r = clampWindowPosition(10, 100, 100, 700, 0, 0, 800, 600);
		expect(r).toEqual({ x: 10, y: 0 });
	});

	it("treats zero window size as one", () => {
		const r = clampWindowPosition(2000, 3000, 0, 0, 0, 0, 800, 600);
		expect(r).toEqual({ x: 799, y: 599 });
	});

	it("respects non-zero work area offset", () => {
		const r = clampWindowPosition(0, 0, 100, 80, 100, 100, 800, 600);
		expect(r).toEqual({ x: 100, y: 100 });
	});
});
