import { response } from '../_common/axios_response';
import { response_message } from '../_common/response_body';

import type { RegisterRequestData, RegisterResponse } from '../models/register_model';

const mockRegister = (data: RegisterRequestData): RegisterResponse => {
	return response(200, response_message('Registration succeeded.')) as RegisterResponse;
};

export { mockRegister };
