import backendConfig from '$server/backend.config';
import type { RequestHandler } from './$types';

export const POST = (async ({ fetch, url }) => {
    const token = url.searchParams.get('token');

    if (!token) {
        return new Response("Missing activation token", {status: 400});
    }

	const response = await fetch(backendConfig.baseUrl + `/user/activate/${token}`, {
		method: 'POST'
	});

	return new Response(`Activation ${response.ok ? 'successful' : 'failed'}`, {
		status: response.status
	});
}) satisfies RequestHandler;
