import { writable } from "svelte/store"

export const user = writable<string>(window.localStorage.getItem("user"))

user.subscribe((value: string) => {
	window.localStorage.setItem("user", value)
})

export { user as default }
