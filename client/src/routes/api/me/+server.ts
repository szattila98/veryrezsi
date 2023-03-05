import serverConfig from '$server/server.config';
import type { User } from '$shared/domain';

// /** @type {import('@sveltejs/kit').RequestHandler} */
// export async function GET({ cookies }: { cookies: Cookies }) {
// 	const sessionId = cookies.get(serverConfig.sessionCookieName);

// 	if (!sessionId) {
// 		return redirect(307, '/login');
// 	}

// 	const user = await callWhoAmIApi(sessionId);

// 	return json(user);
// }

export async function callWhoAmIApi(sessionId: string): Promise<User | null> {
	const response = await fetch(serverConfig.baseUrl + '/user/me', {
		method: 'GET',
		headers: {
			Cookie: serverConfig.sessionCookieName + '=' + sessionId,
			...serverConfig.baseHeaders
		}
	});

	if (!response.ok) {
		return Promise.resolve(null);
	}

	return response.json();
}
