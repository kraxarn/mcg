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

<a id="version" sveltekit:prefetch href="/dev">
	v{version}
</a>

<style>
	main {
		display: flex;
		flex-direction: column;
		align-items: center;
		height: 100vh;
		padding: 1em 0;
		box-sizing: border-box;
	}

	#version {
		color: var(--foreground-color);
		z-index: -1;
		position: fixed;
		font-family: monospace;
		bottom: 12px;
		left: 12px;
	}

	@media screen and (orientation: portrait) {
		main {
			margin-bottom: 32px;
		}

		#version {
			bottom: 1.5%;
			left: 3%;
		}
	}
</style>
