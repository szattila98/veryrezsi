import { response } from '../_common/axios_response';
import type { MeRequestData, MeResponse } from '../models/me_model';

import users from '../../entities/user.json';

/**
 * Returns a mock response to the - me / whoami / currently authenticated user - request.
 * @param {Object} data 
 * @returns {AxiosResponse}
 */
const mockWhoAmI = (data: MeRequestData) : MeResponse => {
		const userId = getUserIdFromSessionId(data.sessionId);
		const currentUser = users[userId];
		return response(200, { user: currentUser });
}

const getUserIdFromSessionId = (sessionId: string): number => {
	const decodedSessionId = atob(sessionId);
	const splitSessionId = decodedSessionId.split('_');
	return parseInt(splitSessionId[splitSessionId.length - 1]);
} 

export {
	mockWhoAmI as me,
}