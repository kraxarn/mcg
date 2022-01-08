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

<script lang="ts">
	import Spinner from "$lib/components/Spinner.svelte"

	let errorMessage: string
</script>

<div id="container">
	<img src="/images/logo.webp" alt="logo" />
	{#if errorMessage}
		<span class="error-message">
			{errorMessage}
		</span>
	{:else}
		<Spinner />
	{/if}
</div>

<style>
	#container {
		position: absolute;
		top: 50%;
		transform: translate(0, -50%);
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 32px;
		color: var(--foreground-alt-color);
	}

	.error-message {
		color: var(--foreground-error-color);
		font-weight: var(--font-weight-title1);
		font-size: 1.2em;
	}
</style>
