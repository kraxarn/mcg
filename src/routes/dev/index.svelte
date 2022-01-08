<script>
	import Button from "$lib/components/Button.svelte"
	import Banner from "$lib/components/Banner.svelte"
	import Panel from "$lib/components/Panel.svelte"
</script>

<Banner title="Developer" />

<Panel>
	<Button>
		<a sveltekit:prefetch href="/dev/nakama"> Nakama </a>
	</Button>
	<Button>
		<a sveltekit:prefetch href="/"> Back </a>
	</Button>
</Panel>
