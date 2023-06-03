import backendConfig from '$server/backend.config';
import serverConfig from '$server/server.config';
import type { RequestHandler } from './$types';

export const POST = (async ({ fetch, cookies }) => {
	const sessionId = cookies.get(serverConfig.clientSessionCookieName);
	if (!sessionId) {
		return new Response('Cookie not set', {
			status: 400
		});
	}

	const response = await fetch(backendConfig.baseUrl + '/user/logout', {
		method: 'POST',
		headers: {
			Cookie: `${backendConfig.serverSessionCookieName}=${sessionId}`,
			...backendConfig.baseHeaders
		}
	});

	if (!response.ok) {
		return new Response('Logout failed', {
			status: response.status
		});
	}

	const authCookie = response.headers
		.get('Set-Cookie')
		?.replace(`${backendConfig.serverSessionCookieName}=`, '');
	if (!authCookie) {
		return new Response('Logout failed, no cookie in server response', {
			status: 500
		});
	}

	cookies.set(backendConfig.serverSessionCookieName, authCookie, { path: '/' });
	return new Response('Logged out', {
		status: response.status
	});
}) satisfies RequestHandler;
