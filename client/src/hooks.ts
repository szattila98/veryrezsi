import { goto } from '$app/navigation';
import { parse } from 'cookie';
import { user_api } from './api/user';

/** @type {import('@sveltejs/kit').Handle} */
export async function handle({ request, resolve }) {
    const cookies = parse(request.headers.cookie || '');

    if (!cookies.JSESSIONID) {
		request.locals.user = null;
		goto('/login');
    }

	if (!request.locals.user) {
		const session = await user_api.me({ sessionId: cookies.JSESSIONID });


		if (session) {
			request.locals.user = session.data.user;
			return resolve(request);
		}
	}

    return resolve(request);
}

/** @type {import('@sveltejs/kit').GetSession} */
export function getSession(request) {
    return request?.locals?.user ? {
             user: {
                 email: request.locals.user.email,
				 username: request.locals.user.username,
             },
			} : {};
}