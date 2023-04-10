import backendConfig from '$server/backend.config';
import { transferSessionCookie } from '$shared/cookie';
import type { RequestHandler } from './$types';

export const POST = (async ({ fetch, cookies, request }) => {
	const response = await fetch(`${backendConfig.baseUrl}/expense`, {
		method: 'POST',
		headers: {
			...transferSessionCookie(cookies),
			...backendConfig.baseHeaders
		},
		body: await request.text()
	});
	const options = { status: response.status, headers: backendConfig.baseHeaders };
	if (!response.ok) return new Response('Creating user expense failed', options);
	return new Response(await response.text(), options);
}) satisfies RequestHandler;
