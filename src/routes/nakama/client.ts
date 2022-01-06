import type { RequestHandler } from "@sveltejs/kit"
import type { Locals } from "$lib/types"

// GET /client
export const get: RequestHandler<Locals> = async () => {
	const serverKey = process.env.NAKAMA_SERVER_KEY
	const host = process.env.NAKAMA_HOST

	if (!serverKey || !host) {
		return {
			status: 500,
		}
	}

	return {
		status: 200,
		body: {
			serverKey,
			host,
		},
	}
}
