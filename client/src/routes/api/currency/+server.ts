import backendConfig from '$server/backend.config';
import { transferSessionCookie } from '$shared/cookie';
import type { RequestHandler } from './$types';

export const GET = (async ({ fetch, cookies }) => {
	const response = await fetch(`${backendConfig.baseUrl}/currency`, {
		method: 'GET',
		headers: {
			...transferSessionCookie(cookies),
			...backendConfig.baseHeaders
		}
	});
	const options = { status: response.status, headers: backendConfig.baseHeaders };
	if (!response.ok) return new Response('Fetching currencies failed', options);
	return new Response(await response.text(), options);
}) satisfies RequestHandler;
