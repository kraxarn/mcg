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
		padding: 0;

		color: var(--foreground-alt-color);
		background-color: #a47137;
		font-size: 1.4em;

		border-width: 18px;
		border-image: url("images/button.webp") 32;
	}

	button span {
		flex: 1;
	}
</style>
