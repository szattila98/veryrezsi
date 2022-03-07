import type { RegisterRequestData, RegisterResponse } from '$mock/api/models/register_model';

import { mockRegister } from '$mock/api/user/register';

/** @type {import('@sveltejs/kit').RequestHandler} */
export async function post({ request }: { request: Request }): Promise<RegisterResponse> {
	const body: RegisterRequestData = await request.json();

	return mockRegister(body);
}
