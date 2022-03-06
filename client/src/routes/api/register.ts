import type { RegisterRequestData, RegisterResponse } from '$mock/api/models/register_model';

import { mockRegister } from '$mock/api/user/register';

/** @type {import('@sveltejs/kit').RequestHandler} */
export async function post({
	registerRequestData,
}: {
	registerRequestData: RegisterRequestData;
}): Promise<RegisterResponse> {
	return Promise.resolve(mockRegister(registerRequestData));
}
