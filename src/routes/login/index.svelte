<script context="module" lang="ts">
	export const prerender = true
</script>

<script lang="ts">
	import Button from "$lib/components/Button.svelte"
	import {
		faGoogle,
		faApple,
		faFacebook,
		faTwitter,
		faGithub,
	} from "@fortawesome/free-brands-svg-icons"
	import { faPhone, faUserSecret } from "@fortawesome/free-solid-svg-icons"
	import PlayingCard from "$lib/components/PlayingCard.svelte"
	import PlayingCardSuit from "$lib/enums/PlayingCardSuit"
	import PlayingCardValue from "$lib/enums/PlayingCardValue"
</script>

<!--<img src="/images/logo.webp" alt="mcg logo" />-->

<!-- Move to component -->
<div id="logo">
	<PlayingCard
		suit="{PlayingCardSuit.Hearts}"
		value="{PlayingCardValue.Queen}" />
	<PlayingCard
		suit="{PlayingCardSuit.Spades}"
		value="{PlayingCardValue.Ten}" />
</div>

<h1>
	Welcome to<br />
	Mobile Card Games
</h1>

<h3>Choose a login method</h3>

<div id="login-methods">
	<Button content="Google" icon={faGoogle} />
	<Button content="Apple" icon={faApple} />
	<Button content="Facebook" icon={faFacebook} />
	<Button content="Twitter" icon={faTwitter} />
	<Button content="GitHub" icon={faGithub} />
	<Button content="Phone number" icon={faPhone} />
	<Button content="Anonymous" icon={faUserSecret} />
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
</style>
