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

	const { encrypted, err } = encrypt(serverKey, host)
	if (err) {
		return {
			status: 500,
			body: {
				message: err,
			},
		}
	}

	return {
		status: 200,
		body: {
			serverKey: encrypted,
			host,
		},
	}
}
