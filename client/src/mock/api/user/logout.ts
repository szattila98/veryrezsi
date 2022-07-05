import type { LogoutRequest, LogoutResponse } from '$shared/api/logout';
import { parse, serialize } from 'cookie';

import { response } from '../_common/axios_response';
import { response_message } from '../_common/response_body';

const mockLogout = (request: LogoutRequest): LogoutResponse => {
	const cookies = parse(request.headers.get('cookie') || '');

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
