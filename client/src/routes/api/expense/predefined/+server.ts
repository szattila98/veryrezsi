import backendConfig from '$server/backend.config';
import { transferSessionCookie } from '$shared/cookie';
import type { RequestHandler } from './$types';

export const GET = (async ({ fetch, cookies }) => {
	const response = await fetch(`${backendConfig.baseUrl}/expense/predefined`, {
		method: 'GET',
		headers: {
			...transferSessionCookie(cookies),
			...backendConfig.baseHeaders
		}
	});
	const options = { status: response.status, headers: backendConfig.baseHeaders };
	if (!response.ok) return new Response('Fetching predefined expenses failed', options);
	return new Response(await response.text(), options);
}) satisfies RequestHandler;

export const POST = (async ({ fetch, cookies, request }) => {
	const response = await fetch(`${backendConfig.baseUrl}/expense/predefined`, {
		method: 'POST',
		headers: {
			...transferSessionCookie(cookies),
			...backendConfig.baseHeaders
		},
		body: await request.text()
	});
	const options = { status: response.status, headers: backendConfig.baseHeaders };
	if (!response.ok) return new Response('Creating predefined expense failed', options);
	return new Response(await response.text(), options);
}) satisfies RequestHandler;
