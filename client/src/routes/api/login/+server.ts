import backendConfig from '$server/backend.config';
import serverConfig from '$server/server.config';
import type { LoginRequestData } from '$shared/api/login';
import type { RequestHandler } from './$types';

export const POST = (async ({ request }) => {
	const data: LoginRequestData = await request.json();
	const response = await fetch(backendConfig.baseUrl + '/user/auth', {
		method: 'POST',
		headers: backendConfig.baseHeaders,
		body: JSON.stringify(data)
	});

	if (!response.ok) {
		return new Response('Login failed', {
			status: response.status
		});
	}

	const headers = new Headers({
		'Set-Cookie': generateClientSideSessionCookie(response),
		Location: '/getting-started'
	});

	return new Response(null, {
		status: 307,
		headers
	});
}) satisfies RequestHandler;

/**
 * Parses session cookie from backend response and maps it to a client side cookie
 * @param backendResponse that contains session cookie in its headers
 * @returns A Set-Cookie value string
 */
function generateClientSideSessionCookie(backendResponse: Response): string {
	const apiSessionCookie = backendResponse.headers.get('Set-Cookie')?.split('=')[1];

	return serverConfig.clientSessionCookieName + '=' + apiSessionCookie + ' ; Path=/';
}
