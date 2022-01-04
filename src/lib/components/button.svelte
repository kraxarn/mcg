<script lang="ts">
	import Fa from "svelte-fa"

	export let id: string
	export let content: string
	export let icon: string
</script>

<button {id}>
	<Fa {icon} />
	<span>
		{content}
	</span>
</button>

<style>
	button {
		display: flex;
		flex-direction: row;
		align-items: center;
		gap: 8px;

		padding: 12px;
		background-color: brown;
		border: 0;
		border-radius: 8px;
		color: var(--foreground-alt-color);
		font-size: 1.2em;
	}

	button span {
		flex: 1;
	}
</style>
