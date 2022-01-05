<script lang="ts">
	import PlayingCardSuit from "$lib/enums/PlayingCardSuit"
	import PlayingCardValue from "$lib/enums/PlayingCardValue"

	export let suit: PlayingCardSuit
	export let value: PlayingCardValue

	const width = 140
	const height = 190
	const padding = 4
	const spacing = 10

	const x = -padding - 24;
	const y = -4;

	const style = `--width:${width + padding}px;
	--height:${height + padding}px;
	--x:${x}px;
	--y:${y}px`
</script>

<div class="playing-card-container" style="--width:{width}px;--height:{height}px">
	<div class="playing-card" {style}>
		{suit} - {value}
	</div>
</div>

<style>
	.playing-card-container {
		width: var(--width);
		height: var(--height);
	}

	.playing-card {
		width: var(--width);
		height: var(--height);
		background: url("/images/playing_cards.svg") var(--x) var(--y);
		background-size: auto 100%;
		background-repeat: no-repeat;
		display: inline-block;
	}
</style>
