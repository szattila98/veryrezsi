import backendConfig from '$server/backend.config';
import serverConfig from '$server/server.config';
import type { RequestHandler } from './$types';

export const POST = (async ({ cookies }) => {
	const sessionId = cookies.get(serverConfig.clientSessionCookieName);

	if (!sessionId) {
		return new Response('Not logged in', {
			status: 500
		});
	}

	const response = await fetch(backendConfig.baseUrl + '/user/logout', {
		method: 'POST',
		headers: {
			cookie: backendConfig.serverSessionCookieName + '=' + sessionId,
			...backendConfig.baseHeaders
		}
	});

	const options = {
		status: response.status,
		headers: new Headers()
	};

	if (!response.ok) {
		return new Response('Logout failed', options);
	}

	options.headers = new Headers({
		'Set-Cookie': generateClientSessionCookie(response)
	});

	return new Response('Logged out', options);
}) satisfies RequestHandler;

/**
 * Generates a Set-Cookie string with the correct path
 * @param response The response of backend API
 * @returns A Set-Cookie value string
 */
function generateClientSessionCookie(backendResponse: Response): string {
	const apiSessionCookie = backendResponse.headers.get('Set-Cookie')?.split('=')[1];
	return serverConfig.clientSessionCookieName + '=' + apiSessionCookie + ' ; Path=/';
}
