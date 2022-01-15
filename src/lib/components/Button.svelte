<script lang="ts">
	import Fa from "svelte-fa"
	import { goto } from "$app/navigation"

	export let id: string
	export let content: string
	export let icon: string
	export let href: string

	const onClick = () => href ? goto(href) : void 0
</script>

<button {id} on:click={onClick}>
	{#if icon !== undefined}
		<div class="button-icon">
			<Fa {icon} />
		</div>
	{/if}
	{#if content === undefined}
		<div class="button-content">
			<slot />
		</div>
	{:else}
		<span>
			{content}
		</span>
	{/if}
</button>

<style>
	button {
		display: flex;
		flex-direction: row;
		align-items: center;
		gap: 8px;
		padding: 0;

		color: var(--foreground-alt-color);
		background-color: #a47137;
		font-size: 1.4em;
		cursor: pointer;

		border-width: 18px;
		border-image: url("/images/button.webp") 32;
	}

	.button-icon {
		width: 1.2em;
	}

	.button-content {
		flex: 1;
	}
</style>
