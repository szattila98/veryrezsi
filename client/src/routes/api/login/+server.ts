import serverConfig from '$server/server.config'
import type { LoginRequestData } from '$shared/api/login'

/** @type {import('@sveltejs/kit').RequestHandler} */
export async function POST({ request }: { request: Request }) {
	const data: LoginRequestData = await request.json();
	const response = await fetch(serverConfig.baseUrl + "/user/auth", {
		method: 'POST',
		headers: serverConfig.baseHeaders,
		body: JSON.stringify(data)
	})



	return response;
}