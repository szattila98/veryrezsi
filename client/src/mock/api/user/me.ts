import { response } from '../_common/axios_response';
import { response_user } from '../models/me_model';

import users from '../../entities/user.json';

import type { MeRequestData, MeResponse } from '../models/me_model';

/**
 * Returns a mock response to the - me / whoami / currently authenticated user - request.
 */
const mockWhoAmI = (data: MeRequestData): MeResponse => {
	if (!data.sessionId) {
		return response(400, response_user(0, '', '')) as MeResponse;
	}

	const userId = getUserIdFromSessionId(data.sessionId);
	const currentUser = users[userId];
	return response(
		200,
		response_user(currentUser.id, currentUser.email, currentUser.username)
	) as MeResponse;
};

const getUserIdFromSessionId = (sessionId: string): number => {
	const decodedSessionId = atob(sessionId);
	const splitSessionId = decodedSessionId.split('_');
	return parseInt(splitSessionId[splitSessionId.length - 1]);
};

export { mockWhoAmI };
