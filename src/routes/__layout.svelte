<script lang="ts">
	import "../app.css"

	const version = process.env.npm_package_version
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
