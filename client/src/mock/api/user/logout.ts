import { parse, serialize } from 'cookie';
import { LogoutRequest, LogoutResponse } from '../models/logout_model';

import { response } from '../_common/axios_response';
import { response_message } from '../_common/response_body';

const mockLogout = (req: LogoutRequest): LogoutResponse => {
	const cookies = parse(req.header.cookie || '');

	if (!cookies.JSESSIONID) {
		return response(400, response_message('Invalid sessionId, unable to logout')) as LogoutResponse;
	}

	// Delete session from db.

	const header = {
		'Set-Cookie': serialize('JSESSIONID', '', {
			path: '/',
			expires: new Date(0),
		}),
	};

	return response(200, response_message('Logout succeeded.'), header) as LogoutResponse;
};

export { mockLogout };
