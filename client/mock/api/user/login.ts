import { response } from '../_common/axios_response';
import type { LoginRequestData, LoginResponse } from '../models/login_model';

const mockLogin = (data: LoginRequestData): LoginResponse => {
		const mockSessionIdBase = 'itsjustacookiefor_';

		// You can login only with the same username - password.
		const success = data.user ===  data.password;
		const userId = getUserIdFromUsername(data.user);

		// Base64 encode - btoa: Binary to ASCII
		const sessionId = btoa(mockSessionIdBase + userId);

		let header = {
			'Set-Cookie': `JSESSIONID:${sessionId}`
		}; 
		
		return success ? response(200, {}, header) : response(401);
}

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
}

export {
	mockLogin as login,
}