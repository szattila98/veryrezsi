import { response } from '../_common/axios_response';
import type { RegisterRequestData, RegisterResponse } from '../models/register_model';

const mockRegister = (data: RegisterRequestData): RegisterResponse => {
	return response(200);
};

export { mockRegister };
