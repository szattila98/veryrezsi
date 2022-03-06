import { parse } from 'cookie';
import { user_api } from './api/user';

/** @type {import('@sveltejs/kit').Handle} */
export async function handle({ event, resolve }) {
	const cookies = parse(event.request.headers.cookie || '');

	console.log('SessionID: ', cookies.JSESSIONID);

	if (!cookies.JSESSIONID) {
		event.locals.user = null;
	} else if (!event.locals.user) {
		const session = await user_api.me({ sessionId: cookies.JSESSIONID });

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
export function getSession(event) {
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
