import backendConfig from '$server/backend.config';
import { transferSessionCookie } from '$shared/cookie';
import type { RequestHandler } from './$types';

export const DELETE = (async ({ fetch, cookies, params }) => {
	const id = params.id;
	const response = await fetch(`${backendConfig.baseUrl}/transaction/${id}`, {
		method: 'DELETE',
		headers: {
			...transferSessionCookie(cookies),
			...backendConfig.baseHeaders
		}
	});
	const options = { status: response.status, headers: backendConfig.baseHeaders };
	if (!response.ok) return new Response('Deleting transaction failed', options);
	return new Response(await response.text(), options);
}) satisfies RequestHandler;
