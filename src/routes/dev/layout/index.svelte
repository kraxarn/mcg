<script>
	import NavigationBar from "$lib/components/NavigationBar.svelte"
	import NavigationBarItem from "$lib/components/NavigationBarItem.svelte"
	import {
		faHome,
		faUsers,
	} from "@fortawesome/free-solid-svg-icons"
</script>

<div id="main-layout">
	<div id="main-header">
		<h1>
			Welcome
		</h1>
	</div>
	<div id="main-content">
		<div>
			Main content
		</div>
	</div>
	<NavigationBar id="main-footer">
		<NavigationBarItem title="Home" icon="{faHome}"/>
		<NavigationBarItem title="Friends" icon="{faUsers}"/>
	</NavigationBar>
</div>

<style>
	:global(body) {
		margin: 0;
	}

	#main-layout {
		display: flex;
		flex-direction: column;
		height: 100vh;
	}

	#main-header {
		color: var(--foreground-alt-color);
		padding: 1em;
	}

	#main-header h1 {
		margin: 0;
	}

	#main-content {
		flex: 1;
		border-image: url("/images/panel.webp") 16;
		border-image-repeat: repeat;
		border-width: 8px;
		border-style: solid;
	}

	#main-content > :first-child {
		background-color: #fff1d3;
		height: 100%;
	}
</style>
