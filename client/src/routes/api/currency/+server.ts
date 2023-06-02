import { requestAsProxy } from '$shared/proxy';
import type { RequestHandler } from './$types';

export const GET = (async ({ fetch, cookies }) => {
	return await requestAsProxy({
		fetch,
		cookies,
		method: 'GET',
		path: '/currency'
	});
}) satisfies RequestHandler;
