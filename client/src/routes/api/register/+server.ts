import serverConfig from '$server/backend.config';
import type { LoginRequestData } from '$shared/api/login';

/** @type {import('@sveltejs/kit').RequestHandler} */
export async function POST({ request }: { request: Request }) {
	const data: LoginRequestData = await request.json();
	const response = await fetch(serverConfig.baseUrl + '/user/auth', {
		method: 'POST',
		headers: serverConfig.baseHeaders,
		body: JSON.stringify(data)
	});

	const headers: Headers = new Headers({
		'Set-Cookie': generateClientSideSessionCookie(response)
	});

	const options = {
		status: 200,
		headers: headers
	};

	return new Response('', options);
}

/**
 *
 * @param backendResponse
 * @returns
 */
function generateClientSideSessionCookie(backendResponse: Response): string {
	const apiSessionCookie = backendResponse.headers.get('Set-Cookie')?.split('=')[1];

	return serverConfig.sessionCookieName + '=' + apiSessionCookie + ' ; Path=/';
}
