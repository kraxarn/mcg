import type { GetSession, Handle } from "@sveltejs/kit"
import { v4 as uuid } from "uuid"
import { userId } from "$lib/user"

export const handle: Handle = async ({ event, resolve }) => {
	if (!userId) {
		userId.set(uuid())
	}

	event.locals.userId = userId
	return resolve(event)
}

export const getSession: GetSession = async (request) => {
	return {
		userId: request.locals.userId,
	}
}
