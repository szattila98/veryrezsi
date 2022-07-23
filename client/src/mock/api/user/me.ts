import { response } from '../_common/axios_response';

import users from '../../entities/user.json';
import type { MeRequestData, MeResponse } from '../../../shared/api/me';
import { response_user } from '../../../shared/api/me';

/**
 * Returns a mock response to the - me / whoami / currently authenticated user - request.
 */
const mockWhoAmI = (data: MeRequestData): MeResponse => {
	if (!data.sessionId) {
		return response(400, response_user(0, '', '')) as MeResponse;
	}

	const userId = getUserIdFromSessionId(data.sessionId);

	if (isNaN(userId)) {
		console.warn(
			'Invalid username (no user number), you authenticated via mock register, dont you?'
		);
		return response(400, {}) as MeResponse;
	}

	const currentUser = users[userId];
	return response(
		200,
		response_user(currentUser.id, currentUser.email, currentUser.username)
	) as MeResponse;
};

const getUserIdFromSessionId = (sessionId: string): number => {
	const decodedSessionId = Buffer.from(sessionId, 'base64').toString('binary');
	const splitSessionId = decodedSessionId.split('_');
	return parseInt(splitSessionId[splitSessionId.length - 1]);
};

export { mockWhoAmI };
