import backendConfig from '$server/backend.config';
import type { RequestHandler } from './$types';

export const GET = (async ({ fetch, cookies }) => {
	const sessionId = cookies.get(backendConfig.serverSessionCookieName)
	const response = await fetch(backendConfig.baseUrl + '/user/me', {
		method: 'GET',
		headers: {
			'Cookie': `${backendConfig.serverSessionCookieName}=${sessionId}`,
            ...backendConfig.baseHeaders
        }
	});

	return new Response(await response.text(), {
		status: response.status,
        headers: backendConfig.baseHeaders
	});
}) satisfies RequestHandler;