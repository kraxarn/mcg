/** @type {import("ts-jest/dist/types").InitialOptionsTsJest} */
export default {
	preset: "ts-jest/presets/js-with-ts-esm",
	testEnvironment: "node",
	globals: {
		"ts-jest": {
			useESM: true,
		},
	},
	moduleNameMapper: {
		"^(\\.{1,2}/.*)\\.js$": "$1",
	},
	testMatch: [
		"**/tests/**/*.ts",
	],
}
