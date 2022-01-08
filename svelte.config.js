import adapter from "@sveltejs/adapter-static"
import preprocess from "svelte-preprocess"
import * as fs from "fs"

// TODO: Workaround for #3246
(() => {
	const url = new URL("node_modules/@heroiclabs/nakama-js/package.json",
		import.meta.url)
	const json = JSON.parse(fs.readFileSync(url, "utf8"))
	json.main = json.module
	json.type = "module"
	fs.writeFileSync(url, JSON.stringify(json))
})()

/** @type {import("@sveltejs/kit").Config} */
const config = {
	// Consult https://github.com/sveltejs/svelte-preprocess
	// for more information about preprocessors
	preprocess: preprocess(),

	kit: {
		adapter: adapter(),

		// hydrate the <div id="svelte"> element in src/app.html
		target: "#svelte",
	},
}

export default config
