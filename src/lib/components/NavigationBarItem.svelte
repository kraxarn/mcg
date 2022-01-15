<script lang="ts">
	import Fa from "svelte-fa"

	export let icon: string
	export let title: string
</script>

<div class="navigation-item-container">
	<Fa {icon}/>
	<span>{title}</span>
	<div class="icon-center">
		<Fa size="1.5x" {icon}/>
	</div>
</div>

<style>
	.navigation-item-container {
		background-color: #fafafa;
		/* 140x190 */
		position: relative;
		width: 8.4em;
		height: 11.4em;
		border: 0.15em solid #bbbbbb;
		border-radius: 8px;
		padding: 8px;
		box-sizing: border-box;
		box-shadow: 0 4px 8px #000000cc;

		transform: translate(0, -10%);
	}

	.selected {
		transform: translate(0, -20%);
	}

	.icon-center {
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
	}
</style>
