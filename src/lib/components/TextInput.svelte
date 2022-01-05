<script lang="ts">
	export let id: string
	export let type: "text" | "password"
</script>

<div id="input-container">
	<input {id} {type} />
</div>

<style>
	#input-container {
		display: flex;
		flex-direction: row;
		border-image: url("/images/input.webp") 8;
		border-image-repeat: repeat;
		border-width: 8px;
		border-style: solid;
	}

	input {
		background-color: #fff1d2;
		border: 0;
		flex: 1;
		padding: 6px;
	}
</style>
