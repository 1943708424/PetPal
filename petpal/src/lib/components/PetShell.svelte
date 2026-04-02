<script lang="ts">
	import { clampWindowPosition } from "$lib/window-layout";

	type WorkArea = { x: number; y: number; w: number; h: number };
	type WindowSize = { w: number; h: number };
	type Position = { x: number; y: number };

	let {
		workArea = { x: 0, y: 0, w: 800, h: 600 },
		windowSize = { w: 200, h: 150 },
		position = { x: 0, y: 0 },
	}: {
		workArea?: WorkArea;
		windowSize?: WindowSize;
		position?: Position;
	} = $props();

	const clamped = $derived(
		clampWindowPosition(
			position.x,
			position.y,
			windowSize.w,
			windowSize.h,
			workArea.x,
			workArea.y,
			workArea.w,
			workArea.h
		)
	);
</script>

<!-- 宠物主窗体占位：后续接入精灵与拖动，当前输出钳制后的逻辑坐标 -->
<div data-testid="pet-shell" class="pet-shell" role="region" aria-label="Pet viewport">
	<span data-testid="clamped-x">{clamped.x}</span>
	<span data-testid="clamped-y">{clamped.y}</span>
</div>

<style>
	.pet-shell {
		display: inline-flex;
		gap: 0.5rem;
		padding: 0.5rem;
		border-radius: 8px;
		background: color-mix(in srgb, var(--pet-bg, #2a2a2a) 90%, transparent);
		font-variant-numeric: tabular-nums;
	}
</style>
