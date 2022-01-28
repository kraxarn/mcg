import type { GetSession, Handle } from "@sveltejs/kit"
import { v4 as uuid } from "uuid"
import { userId } from "$lib/user"

export const handle: Handle = async ({ event, resolve }) => {
	userId.subscribe(u => {
		if (!u) {
			userId.set(uuid())
			return
		}
		event.locals.userId = u
	})

	return resolve(event)
}

export const getSession: GetSession = async (request) => {
	return {
		userId: request.locals.userId,
	}
}
