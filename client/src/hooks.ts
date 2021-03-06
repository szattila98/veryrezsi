import type { RequestEvent } from '@sveltejs/kit';

import { parse } from 'cookie';

import { getSession as getSessionById } from '$api/me';

/** @type {import('@sveltejs/kit').Handle} */
export async function handle({
	event,
	resolve,
}: {
	event: RequestEvent;
	resolve(event: RequestEvent): Awaited<Promise<Response>>;
}) {
	const cookies = parse(event.request.headers.get('cookie') || '');

	if (!cookies.JSESSIONID) {
		event.locals.user = null;
	} else if (!event.locals.user) {
		const session = await getSessionById(cookies.JSESSIONID);

		if (session) {
			event.locals.user = session.data.user;
			const response = resolve(event);
			return response;
		}
	}

	const response = resolve(event);
	return response;
}

/** @type {import('@sveltejs/kit').GetSession} */
export function getSession(event: RequestEvent) {
	return event.locals.user
		? {
				// Only publish public data here. You might not need everything from event.locals.user.
				user: {
					id: event.locals.user.id,
					email: event.locals.user.email,
					username: event.locals.user.username,
				},
		  }
		: {};
}

process.on('SIGINT', function () {
	process.exit();
}); // Docker - Ctrl+C
process.on('SIGTERM', function () {
	process.exit();
}); // Docker - Terminated
