import backendConfig from '$server/backend.config';
import { transferSessionCookie } from '$shared/cookie';
import type { RequestHandler } from './$types';

export const GET = (async ({ fetch, cookies }) => {
	const response = await fetch(backendConfig.baseUrl + '/user/me', {
		method: 'GET',
		headers: {
			...transferSessionCookie(cookies),
			...backendConfig.baseHeaders
		}
	});

	return new Response(await response.text(), {
		status: response.status,
		headers: backendConfig.baseHeaders
	});
}) satisfies RequestHandler;
