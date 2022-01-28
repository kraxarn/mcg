import { writable } from "svelte/store"
import { browser } from "$app/env"

export const userId = writable<string>()

if (browser) {
	userId.set(localStorage.getItem("userId"))

	userId.subscribe((value: string) => {
		window.localStorage.setItem("userId", value)
	})
}
