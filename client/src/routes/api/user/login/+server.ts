import backendConfig from '$server/backend.config';
import type { RequestHandler } from './$types';

export const POST = (async ({ fetch, request, cookies }) => {
	const response = await fetch(backendConfig.baseUrl + '/user/auth', {
		method: 'POST',
		headers: backendConfig.baseHeaders,
		body: await request.text()
	});

	if (!response.ok) {
		return new Response('Login failed', {
			status: response.status
		});
	}

	const authCookie = response.headers
		.get('Set-Cookie')
		?.replace(`${backendConfig.serverSessionCookieName}=`, '');
	if (!authCookie) {
		return new Response('Login failed, no cookie in server response', {
			status: 500
		});
	}

	cookies.set(backendConfig.serverSessionCookieName, authCookie, { path: '/' });
	return new Response('Login successful', {
		status: response.status
	});
}) satisfies RequestHandler;
