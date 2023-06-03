import { requestAsProxy } from '$shared/proxy';
import type { RequestHandler } from './$types';

export const GET = (async ({ fetch, cookies }) => {
	return await requestAsProxy({
		fetch,
		cookies,
		method: 'GET',
		path: `/expense/predefined`
	});
}) satisfies RequestHandler;

export const POST = (async ({ fetch, cookies, request }) => {
	return await requestAsProxy({
		fetch,
		cookies,
		method: 'POST',
		path: `/expense/predefined`,
		request
	});
}) satisfies RequestHandler;
