import { RegisterRequestData, RegisterResponse } from 'mock/api/models/register_model';
import { mockRegister } from '../mock/api/user/register';

const register = async (registerRequestData: RegisterRequestData): Promise<RegisterResponse> => {
	return Promise.resolve(mockRegister(registerRequestData));
};

export const user_api = {
	register,
};
