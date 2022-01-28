import type { GetSession, Handle } from "@sveltejs/kit"
import { v4 as uuid } from "uuid"

export const handle: Handle = async ({ event, resolve }) => {
	let userId = localStorage.getItem("userId")

	if (userId === null) {
		userId = uuid()
		localStorage.setItem("userId", userId)
	}

	event.locals.userId = userId
	return resolve(event)
}

export const getSession: GetSession = async (request) => {
	return {
		userId: request.locals.userId,
	}
}
