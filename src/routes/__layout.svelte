<script context="module" lang="ts">
	import type { Load } from "@sveltejs/kit"

	export const load: Load = async ({ fetch }) => {
		const response = await fetch("/version.json")
		const json = await response.json()
		return {
			props: {
				version: json.version,
			},
		}
	}
</script>

<script lang="ts">
	import "../app.css"

	export let version: string
</script>

<main>
	<slot />
</main>

<span id="version">
	v{version}
</span>

<style>
	main {
		display: flex;
		flex-direction: column;
		align-items: center;
	}

	#version {
		position: fixed;
		font-family: monospace;
		bottom: 12px;
		left: 12px;
	}
</style>
