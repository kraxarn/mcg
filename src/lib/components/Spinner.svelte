<script lang="ts">

	import Fa from "svelte-fa"
	import { faSpinner } from "@fortawesome/free-solid-svg-icons"
</script>

<div class="spinner-container">
	<Fa icon="{faSpinner}" />
</div>

<style>
	.spinner-container {
		font-size: 3em;
		width: 1.3em;
		height: 1.3em;
		display: flex;
		flex-direction: row;
		align-items: center;
		justify-content: center;
		animation-name: spinner-spin;
		animation-duration: 1.5s;
		animation-iteration-count: infinite;
		animation-timing-function: linear;
	}

	@keyframes spinner-spin {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}
</style>
