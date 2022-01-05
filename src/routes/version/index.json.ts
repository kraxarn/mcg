import type { RequestHandler } from "@sveltejs/kit"
import type { Locals } from "$lib/types"

// GET /version.json
export const get: RequestHandler<Locals> = async () => {
	return {
		status: 200,
		body: {
			version: process.env.npm_package_version,
		},
	}
}
