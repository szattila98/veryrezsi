import { requestAsProxy } from '$shared/proxy';
import type { RequestHandler } from './$types';

export const POST = (async ({ fetch, cookies, request }) => {
	return await requestAsProxy({
		fetch,
		cookies,
		method: 'POST',
		path: `/transaction`,
		request
	});
}) satisfies RequestHandler;
