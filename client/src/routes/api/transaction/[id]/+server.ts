import { requestAsProxy } from '$shared/proxy';
import type { RequestHandler } from './$types';

export const DELETE = (async ({ fetch, cookies, params }) => {
	const id = params.id;
	return await requestAsProxy({
		fetch,
		cookies,
		method: 'DELETE',
		path: `/transaction/${id}`
	});
}) satisfies RequestHandler;
