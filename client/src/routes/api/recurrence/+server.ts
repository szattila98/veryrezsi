import backendConfig from '$server/backend.config';
import { transferSessionCookie } from '$shared/cookie';
import type { RequestHandler } from './$types';

export const GET = (async ({ fetch, cookies }) => {
	const response = await fetch(`${backendConfig.baseUrl}/recurrence`, {
		method: 'GET',
		headers: {
			...transferSessionCookie(cookies),
			...backendConfig.baseHeaders
		}
	});
	const options = { status: response.status, headers: backendConfig.baseHeaders };
	if (!response.ok) return new Response('Fetching recurrences failed', options);
	return new Response(await response.text(), options);
}) satisfies RequestHandler;
