import backendConfig from '$server/backend.config';
import { redirect } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

export const GET = (async ({ fetch }) => {
	const response = await fetch(backendConfig.baseUrl + '/user/me', {
		method: 'GET',
		headers: {
            ...backendConfig.baseHeaders
        }
	});

	if (!response.ok) {
		throw redirect(307, '/getting-started');
	}

	return new Response(await response.text(), {
		status: 200,
        headers: backendConfig.baseHeaders
	});
}) satisfies RequestHandler;