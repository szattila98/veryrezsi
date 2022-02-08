import { AxiosResponse, response } from '../_common/axios_response'; 

/**
 * 
 * @param {Object} data 
 * @returns {AxiosResponse}
 */
let mockRegister = (data) => {
		return response(200);
}

export {
	mockRegister as register,
}