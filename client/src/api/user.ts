import { LoginRequestData, LoginResponse } from '../mock/api/models/login_model';
import { RegisterRequestData, RegisterResponse } from '../mock/api/models/register_model';
import { mockLogin } from '../mock/api/user/login';
import { mockRegister } from '../mock/api/user/register';

const login = async (loginRequestData: LoginRequestData): Promise<LoginResponse> => {
	const res = mockLogin(loginRequestData);
	if (res.status === 401) {
		return Promise.reject(res);
	} else {
		return Promise.resolve(res);
	}
};

const register = async (registerRequestData: RegisterRequestData): Promise<RegisterResponse> => {
	return Promise.resolve(mockRegister(registerRequestData));
};

export const user_api = {
	register,
	login,
};
