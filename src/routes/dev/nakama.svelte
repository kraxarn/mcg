<script context="module" lang="ts">
	import type { Load } from "@sveltejs/kit"

	export const load: Load = async ({ fetch }) => {
		const response = await fetch("/nakama/client")
		const json = await response.json()
		return {
			props: json,
		}
	}
</script>

<script lang="ts">
	import Banner from "$lib/components/Banner.svelte"
	import Panel from "$lib/components/Panel.svelte"
	import { Client } from "@heroiclabs/nakama-js"

	export let serverKey: string
	export let host: string
	let messages = [
			`Connecting to ${host}...`,
		]

	;(async () => {
		try {
			const client = new Client(serverKey, host)
			const session = await client.authenticateDevice(
				"00000000-0000-0000-0000-000000000000",
				false,
			)
			messages.push(`Connected: ${session.token}`)
		} catch (e: Error) {
			messages.push(`${e.name}: ${e.message}`)
		}
	})().then(() => messages.push("Done"))
</script>

<Banner title="Nakama" />

<Panel>
	<pre>
		{#each messages as message}
			{message}<br />
		{/each}
	</pre>
</Panel>
