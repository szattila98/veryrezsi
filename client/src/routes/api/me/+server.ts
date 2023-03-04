import serverConfig from '$server/server.config'
import type { User } from '$shared/domain';

// /** @type {import('@sveltejs/kit').RequestHandler} */
// export async function GET({ request }: { request: Request }) {
// 	request.headers.get

// 	return response;
// }

export async function callWhoAmIApi(sessionId: string): Promise<User|null> {
	const response = await fetch(serverConfig.baseUrl + "/user/me", {
		method: 'GET',
		headers: {
			Cookie: serverConfig.sessionCookieName + '=' + sessionId,
			...serverConfig.baseHeaders
		},
	})

	if(!response.ok) {
		return Promise.resolve(null)
	}

	return response.json()
}