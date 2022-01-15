<script lang="ts">
	import Fa from "svelte-fa"
	import { faSpinner } from "@fortawesome/free-solid-svg-icons"
</script>

<div class="spinner-container">
	<Fa icon={faSpinner} spin />
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
	}
</style>
