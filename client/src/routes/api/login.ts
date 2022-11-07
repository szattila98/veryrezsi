import { mockLogin } from '$mock/api/user/login';
import type { LoginResponse, LoginRequestData } from '$shared/api/login';

/** @type {import('@sveltejs/kit').RequestHandler} */
export async function POST({ request }: { request: Request }): Promise<LoginResponse> {
	const body: LoginRequestData = await request.json();

	return mockLogin(body);
}
