import { MaybePromise } from '@sveltejs/kit/types/helper';
import { RequestEvent } from '@sveltejs/kit';

import { parse } from 'cookie';

import { getSession as getSessionById } from '$api/me';

/** @type {import('@sveltejs/kit').Handle} */
export async function handle({
	event,
	resolve,
}: {
	event: RequestEvent;
	resolve(event: RequestEvent): MaybePromise<Response>;
}) {
	const cookies = parse(event.request.headers.get('cookie') || '');

	if (!cookies.JSESSIONID) {
		event.locals.user = null;
	} else if (!event.locals.user) {
		const session = await getSessionById(cookies.JSESSIONID);

		if (session) {
			event.locals.user = session.data.user;
			const response = await resolve(event);
			return response;
		}
	}

	const response = await resolve(event);
	return response;
}

/** @type {import('@sveltejs/kit').GetSession} */
export function getSession(event: RequestEvent) {
	return event.locals.user
		? {
				// Only publish public data here.
				user: {
					email: event.locals.user.email,
					username: event.locals.user.username,
				},
		  }
		: {};
}
