<script lang="ts">
	export let title: string
</script>

<div class="banner-container">
	<h1>
		{title}
	</h1>
</div>

<style>
	.banner-container {
		display: flex;
		flex-direction: column;
		width: 100%;
		min-height: 80px;
		background-image: url("/images/banner_top.webp");
		background-size: contain;
		background-repeat: no-repeat;
		background-position: center;
		color: var(--foreground-alt-color);
		margin-bottom: 16px;
	}

	h1 {
		margin-top: 20px;
		margin-bottom: 34px;
		text-align: center;
	}
</style>
