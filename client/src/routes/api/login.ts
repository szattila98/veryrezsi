import type { LoginRequestData, LoginResponse } from '$mock/api/models/login_model';

import { mockLogin } from '$mock/api/user/login';

export async function post(loginRequestData: LoginRequestData): Promise<LoginResponse> {
	const res = mockLogin(loginRequestData);
	if (res.status === 401) {
		return Promise.reject(res);
	} else {
		return Promise.resolve(res);
	}
}
