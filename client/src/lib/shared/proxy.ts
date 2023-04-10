import backendConfig from '$server/backend.config';
import type { Cookies } from '@sveltejs/kit';
import { transferSessionCookie } from './cookie';

interface RequestProxyConfig {
	fetch: typeof fetch;
	cookies: Cookies;
	method: 'GET' | 'POST' | 'DELETE';
	path: string;
	request?: Request;
}

export const requestAsProxy = async ({
	method,
	cookies,
	path,
	request
}: RequestProxyConfig): Promise<Response> => {
	const response = await fetch(`${backendConfig.baseUrl}${path}`, {
		method,
		headers: {
			...transferSessionCookie(cookies),
			...backendConfig.baseHeaders
		},
		body: request ? await request.text() : undefined
	});
	return new Response(await response.text(), {
		status: response.status,
		headers: backendConfig.baseHeaders
	});
};
