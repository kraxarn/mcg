<script>
	import Button from "$lib/components/Button.svelte"
	import Banner from "$lib/components/Banner.svelte"
	import Panel from "$lib/components/Panel.svelte"
</script>

<Banner title="Developer"/>

<Panel>
	<Button content="Deck"/>
	<Button content="Scroll"/>
	<Button content="Input"/>
	<Button content="Storage"/>
	<Button>
		<a sveltekit:prefetch href="/">
			Back
		</a>
	</Button>
</Panel>
