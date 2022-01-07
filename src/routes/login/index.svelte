<script context="module" lang="ts">
	import type { Load } from "@sveltejs/kit"

	export const prerender = true

	export const load: Load = async ({ fetch }) => {
		const response = await fetch("/nakama/client")
		const json = await response.json()
		return {
			props: json,
		}
	}
</script>

<script lang="ts">
	import Button from "$lib/components/Button.svelte"
	import {
		faGoogle,
		faApple,
		faFacebook,
		faSteam,
	} from "@fortawesome/free-brands-svg-icons"
	import {
		faAt,
		faUserSecret,
		faSignInAlt,
	} from "@fortawesome/free-solid-svg-icons"
	import TextInput from "$lib/components/TextInput.svelte"
	import LoginMethod from "./LoginMethod.svelte"
	import Spinner from "$lib/components/Spinner.svelte"

	let selectedItem: string

	const onLoginClicked = (event: MouseEvent) => {
		const button = (event.target as Element).closest("button")
		if (!button || !button.id.startsWith("login-")) {
			return
		}

		const method = button.id.substring(button.id.indexOf("-") + 1)
		selectedItem = method === selectedItem
			? undefined
			: method
	}
</script>

<img src="/images/logo.webp" alt="mcg logo" />

<h1>
	Welcome to<br />
	Mobile Card Games
</h1>

<h3>Choose a login method</h3>

<div id="login-methods" on:click={onLoginClicked}>
	<LoginMethod name="Apple" icon="{faApple}" {selectedItem}>
		<Spinner />
	</LoginMethod>

	<LoginMethod name="Facebook" icon="{faFacebook}" {selectedItem}>
		<Spinner />
	</LoginMethod>

	<LoginMethod name="Google" icon="{faGoogle}" {selectedItem}>
		<Spinner />
	</LoginMethod>

	<LoginMethod name="Steam" icon="{faSteam}" {selectedItem}>
		<Spinner />
	</LoginMethod>

	<LoginMethod name="Email" icon="{faAt}" {selectedItem}>
		<div id="login-email-details" class="login-details">
			<span>Email</span>
			<TextInput type="text" />
			<span>Password</span>
			<TextInput type="password" />
			<div></div>
			<Button id="email-go" content="Login" icon="{faSignInAlt}" />
		</div>
	</LoginMethod>

	<LoginMethod name="Anonymous" icon="{faUserSecret}" {selectedItem}>
		<div id="login-anonymous-details" class="login-details">
			<span>Username</span>
			<TextInput type="text" />
			<div></div>
			<Button id="anonymous-go" content="Login" icon="{faSignInAlt}" />
		</div>
	</LoginMethod>
</div>

<style>
	h1 {
		text-align: center;
	}

	#login-methods {
		display: flex;
		flex-direction: column;
		gap: 12px;
		width: 80%;
	}

	.login-details {
		display: grid;
		grid-template-columns: 1fr 2fr;
		width: 70%;
		row-gap: 12px;
		margin: 0 auto;
	}

	.login-details span {
		align-self: center;
	}
</style>
