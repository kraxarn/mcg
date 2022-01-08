import adapter from "@sveltejs/adapter-static"
import preprocess from "svelte-preprocess"
import * as fs from "fs"

// TODO: Workaround for heroiclabs/nakama-js#107
;(() => {
	const base = new URL("node_modules/@heroiclabs/nakama-js/", import.meta.url)
	// nakama-js.esm.mjs
	fs.rename(
		new URL("dist/nakama-js.esm.js", base),
		new URL("dist/nakama-js.esm.mjs", base),
		(err) => void err
	)
	// package.json
	const packageJson = new URL("package.json", base)
	const json = JSON.parse(fs.readFileSync(packageJson, "utf8"))
	json.module = "dist/nakama-js.esm.mjs"
	json.exports = {
		"./package.json": "./package.json",
		".": {
			import: "./dist/nakama-js.esm.mjs",
			require: "./dist/nakama-js.cjs.js",
		},
	}
	fs.writeFileSync(packageJson, JSON.stringify(json))
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
