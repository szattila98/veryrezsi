import { serialize } from 'cookie';

import { response } from '../_common/axios_response';
import { response_message } from '../_common/response_body';

import type { LoginRequestData, LoginResponse } from '../models/login_model';

const USERID_IS_NOT_A_NUMBER_MESSGE =
	'UserId is not a number. Use "randomName_1" username format to provide userId';

const mockLogin = (data: LoginRequestData): LoginResponse => {
	// You can login only with the same username - password.
	const success = data.user === data.password;
	let header = {};

	if (!success) {
		return buildFailedResponse();
	}

	try {
		header = generateSessionCookieForUser(data.user);
		return buildSuccessResponseWithHeader(header);
	} catch (e) {
		return buildFailedResponse();
	}
};

const generateSessionCookieForUser = (user: string): object => {
	const mockSessionIdBase = 'itsjustacookiefor_';
	const userId = getUserIdFromUsername(user);

	// Base64 encode - Binary to ASCII encoded string
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

/**
 * Selects usersId from username (for mock login procedure)
 * You can provide userId in this format:
 * - joe_32 -> 32
 * - test_01 -> 1
 * - admin_denis_12 -> 12
 * @throws Error, if userId part is not a valid number
 */
const getUserIdFromUsername = (username: string): number => {
	const splitUser = username.split('_');
	const userIdPart = splitUser[splitUser.length - 1];
	const userId = parseInt(userIdPart);

	if (isNaN(userId)) {
		throw Error(USERID_IS_NOT_A_NUMBER_MESSGE);
	}
	return userId;
};

const buildSuccessResponseWithHeader = (header: object): LoginResponse => {
	return response(200, response_message('Login succeeded.'), header) as LoginResponse;
};

const buildFailedResponse = (): LoginResponse => {
	return response(401, response_message('Failed to login.')) as LoginResponse;
};

export { USERID_IS_NOT_A_NUMBER_MESSGE, mockLogin, generateSessionCookieForUser };
