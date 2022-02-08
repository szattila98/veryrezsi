import { AxiosResponse, response } from '../_common/axios_response';

import users from '../../entities/user.json';

/**
 * Returns a mock response to the - me / whoami / currently authenticated user - request.
 * @param {Object} data 
 * @returns {AxiosResponse}
 */
let mockWhoAmI = (data) => {
		// TODO: Check presence of cookie?
		let currentUser = users[9];
		
		return response(200, {user: currentUser});
}

export {
	mockWhoAmI as me,
}