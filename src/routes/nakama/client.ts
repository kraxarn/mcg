import type { RequestHandler } from "@sveltejs/kit"
import type { Locals } from "$lib/types"
import { encrypt } from "xor-cryptor"

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
			serverKey: encrypt(serverKey, host),
			host,
		},
	}
}
