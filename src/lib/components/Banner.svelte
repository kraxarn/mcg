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
		background-image: url("/images/banner.webp");
		background-size: contain;
		background-repeat: no-repeat;
		background-position: center;
		color: var(--foreground-alt-color);
		margin-bottom: 16px;
	}

	h1 {
		text-align: center;
	}
</style>
