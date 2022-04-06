import { serialize } from 'cookie';

import { response } from '../_common/axios_response';
import { response_message } from '../_common/response_body';

import type { LoginRequestData, LoginResponse } from '../models/login_model';

const mockLogin = (data: LoginRequestData): LoginResponse => {
	// You can login only with the same username - password.
	const success = data.user === data.password;
	let header = {};

	if (success) {
		header = generateSessionCookieForUser(data.user);
	}

	return (
		success
			? response(200, response_message('Login succeeded.'), header)
			: response(401, response_message('Failed to login.'))
	) as LoginResponse;
};

/**
 * Selects usersId from username (for mock login procedure)
 * You can provide userId in this format:
 * - joe_32 -> 32
 * - test_01 -> 1
 * - admin_denis_12 -> 12
 */
const getUserIdFromUsername = (username: string): number => {
	const splitUser = username.split('_');
	return parseInt(splitUser[splitUser.length - 1]);
};

const generateSessionCookieForUser = (user: string): object => {
	const mockSessionIdBase = 'itsjustacookiefor_';
	const userId = getUserIdFromUsername(user);

	// Base64 encode - btoa: Binary to ASCII
	const sessionId = Buffer.from(mockSessionIdBase + userId, 'binary').toString('base64');

	return {
		'Set-Cookie': serialize('JSESSIONID', sessionId, {
			path: '/',
			httpOnly: true,
			sameSite: 'strict',
			secure: process.env.NODE_ENV === 'production',
			maxAge: 60 * 60 * 24 * 7, // one week
		}),
	};
};

export { mockLogin, generateSessionCookieForUser };
