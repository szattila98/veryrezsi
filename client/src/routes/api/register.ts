import { mockRegister } from '$mock/api/user/register';
import type { RegisterResponse, RegisterRequestData } from '$shared/api/register';

/** @type {import('@sveltejs/kit').RequestHandler} */
export async function post({ request }: { request: Request }): Promise<RegisterResponse> {
	const body: RegisterRequestData = await request.json();

	return mockRegister(body);
}
