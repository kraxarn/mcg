<div class="panel-container" on:click>
	<div class="panel-content">
		<slot />
	</div>
</div>

<style>
	.panel-container {
		border-image: url("/images/panel.webp") 16;
		border-image-repeat: repeat;
		border-width: 8px;
		border-style: solid;
		width: 50%;
		flex: 1;
		overflow-y: hidden;
	}

	.panel-content {
		width: 100%;
		height: 100%;
		display: flex;
		gap: 12px;
		padding: 12px;
		box-sizing: border-box;
		flex-direction: column;
		background-color: #fff1d3;
		overflow-y: auto;
	}

	@media screen and (orientation: portrait) {
		.panel-container {
			width: 95%;
		}
	}
</style>
