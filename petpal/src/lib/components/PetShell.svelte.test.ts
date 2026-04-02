import { cleanup, render, screen } from "@testing-library/svelte";
import { afterEach, describe, expect, it } from "vitest";
import PetShell from "./PetShell.svelte";

afterEach(() => {
	cleanup();
});

describe("PetShell", () => {
	it("renders clamped coordinates for overflowing position", () => {
		render(PetShell, {
			props: {
				workArea: { x: 0, y: 0, w: 800, h: 600 },
				windowSize: { w: 100, h: 80 },
				position: { x: 900, y: 10 },
			},
		});
		expect(screen.getByTestId("clamped-x").textContent).toBe("700");
		expect(screen.getByTestId("clamped-y").textContent).toBe("10");
	});

	it("renders region for accessibility", () => {
		render(PetShell, {});
		expect(screen.getByRole("region", { name: /pet viewport/i })).toBeTruthy();
	});
});
