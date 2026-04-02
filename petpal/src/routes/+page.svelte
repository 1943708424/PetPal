<script lang="ts">
	import PetShell from "$lib/components/PetShell.svelte";
</script>

<main class="page">
	<h1>PetPal</h1>
	<p class="hint">window-shell：工作区内坐标钳制（P0 模块）</p>
	<PetShell
		workArea={{ x: 0, y: 0, w: 800, h: 600 }}
		windowSize={{ w: 200, h: 150 }}
		position={{ x: 900, y: 10 }}
	/>
</main>

<style>
	.page {
		padding: 1rem;
		font-family: system-ui, sans-serif;
	}
	.hint {
		opacity: 0.85;
		font-size: 0.9rem;
		margin-bottom: 1rem;
	}
</style>
