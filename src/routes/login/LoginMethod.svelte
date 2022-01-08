<script lang="ts">
	import Button from "$lib/components/Button.svelte"

	export let name: string
	export let icon: string
	export let selectedItem: string

	const item = name?.toLowerCase()
</script>

{#if !selectedItem || selectedItem === item}
	<Button id="login-{item}" content={name} {icon} />
	{#if selectedItem === item}
		<div class="login-item-container">
			<slot />
		</div>
	{/if}
{/if}

<style>
	.login-item-container {
		display: flex;
		flex-direction: column;
		align-items: center;
	}
</style>
