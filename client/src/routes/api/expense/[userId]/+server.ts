import { requestAsProxy } from '$shared/proxy';
import type { RequestHandler } from './$types';

export const GET = (async ({ fetch, cookies, params }) => {
	const id = params.userId;
	return await requestAsProxy({
		fetch,
		cookies,
		method: 'GET',
		path: `/expense/${id}`
	});
}) satisfies RequestHandler;
