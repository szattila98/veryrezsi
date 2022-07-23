import type { RegisterRequestData, RegisterResponse } from '$shared/api/register';
import { response } from '../_common/axios_response';
import { response_message } from '../_common/response_body';
import { generateSessionCookieForUser } from './login';

const mockRegister = (data: RegisterRequestData): RegisterResponse => {
	const header = generateSessionCookieForUser(data.user);

	return response(200, response_message('Registration succeeded.'), header) as RegisterResponse;
};

export { mockRegister };
