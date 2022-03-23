import type { LoginRequestData, LoginResponse } from '$mock/api/models/login_model';

import { mockLogin } from '$mock/api/user/login';

/** @type {import('@sveltejs/kit').RequestHandler} */
export async function post({ request }: { request: Request }): Promise<LoginResponse> {
	const body: LoginRequestData = await request.json();

	return mockLogin(body);
}
