import type { LoginRequestData, LoginResponse } from '../mock/api/models/login_model';
import type { LogoutRequest, LogoutResponse } from '../mock/api/models/logout_model';
import type { RegisterRequestData, RegisterResponse } from '../mock/api/models/register_model';
import type { MeRequestData, MeResponse } from '../mock/api/models/me_model';

import { mockRegister } from '../mock/api/user/register';
import { mockLogin } from '../mock/api/user/login';
import { mockLogout } from '../mock/api/user/logout';
import { mockWhoAmI } from '../mock/api/user/me';

const register = async (registerRequestData: RegisterRequestData): Promise<RegisterResponse> => {
	return Promise.resolve(mockRegister(registerRequestData));
};

const login = async (loginRequestData: LoginRequestData): Promise<LoginResponse> => {
	const res = mockLogin(loginRequestData);
	if (res.status === 401) {
		return Promise.reject(res);
	} else {
		return Promise.resolve(res);
	}
};

const logout = async (logoutRequest: LogoutRequest): Promise<LogoutResponse> => {
	return Promise.resolve(mockLogout(logoutRequest));
};

const me = async (meRequestData: MeRequestData): Promise<MeResponse> => {
	return Promise.resolve(mockWhoAmI(meRequestData));
};

export const user_api = {
	register,
	login,
	logout,
	me,
};
