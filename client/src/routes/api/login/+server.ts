import serverConfig from '$server/server.config';
import type { LoginRequestData } from '$shared/api/login';

/** @type {import('@sveltejs/kit').RequestHandler} */
export async function POST({ request }: { request: Request }) {
	const data: LoginRequestData = await request.json();
	const response = await fetch(serverConfig.baseUrl + '/user/auth', {
		method: 'POST',
		headers: serverConfig.baseHeaders,
		body: JSON.stringify(data)
	});

	const apiSessionCookie = response.headers.get('Set-Cookie')?.split('=')[1];

	// We need to modify the path, so we can receive the cookie during regular page loads during SSR
	const headers: Headers = new Headers({
		'Set-Cookie': serverConfig.sessionCookieName + '=' + apiSessionCookie + ' ; Path=/'
	});

	const options = {
		status: 200,
		headers: headers
	};

	return new Response(String({}), options);
}
