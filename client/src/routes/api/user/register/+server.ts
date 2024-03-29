import serverConfig from '$server/backend.config';
import type { RequestHandler } from './$types';

export const POST = (async ({ request }) => {
	const response = await fetch(serverConfig.baseUrl + '/user/register', {
		method: 'POST',
		headers: serverConfig.baseHeaders,
		body: await request.text()
	});

	const options = {
		status: response.status
	};

	if (!response.ok) {
		return new Response('Registration failed', options);
	}

	return new Response('Registered', options);
}) satisfies RequestHandler;
