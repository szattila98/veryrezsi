import { AxiosResponse, response } from '../_common/axios_response'; 

/**
 * 
 * @param {Object} data 
 * @returns {AxiosResponse}
 */
let mockLogin = (data) => {
		let success = data.user === 'test' && data.password === 'test';
		
		return success ? response(200) : response(401);
}

export {
	mockLogin as login,
}