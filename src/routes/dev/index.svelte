<script>
	import Button from "$lib/components/Button.svelte"
	import Banner from "$lib/components/Banner.svelte"
	import Panel from "$lib/components/Panel.svelte"
	import {
		faChevronLeft,
		faRocket,
	} from "@fortawesome/free-solid-svg-icons"
</script>

<Banner title="Developer" />

<Panel>
	<Button href="/dev/nakama" content="Nakama" icon="{faRocket}" />
	<Button href="/" content="Back" icon="{faChevronLeft}" />
</Panel>
