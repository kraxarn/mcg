import cookie from "cookie"
import type { Handle } from "@sveltejs/kit"

export const handle: Handle = async ({ request, resolve }) => {
	const cookies = cookie.parse(request.headers.cookie || "")
	request.locals.userid = cookies.userid

	// TODO https://github.com/sveltejs/kit/issues/1046
	const method = request.url.searchParams.get("_method")
	if (method) {
		request.method = method.toUpperCase()
	}

	return resolve(request)
}
