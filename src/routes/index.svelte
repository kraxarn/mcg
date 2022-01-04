<script context="module" lang="ts">
	import type { Load } from "@sveltejs/kit"

	export const load: Load = ({ session }) => {
		if (!session.user) {
			return {
				status: 302,
				redirect: "/login",
			}
		}
	}
</script>

<div id="container">
	<img src="images/logo.webp" alt="logo" />
</div>

<style>
	#container {
		position: absolute;
		top: 50%;
		transform: translate(0, -50%);
	}
</style>
