import type { LoginResponse, LoginRequestData } from '$shared/api/login';
import Backend from '$services/BackendApi';

/** @type {import('@sveltejs/kit').RequestHandler} */
export async function POST({ request }: { request: Request }): Promise<LoginResponse> {
	const body: LoginRequestData = await request.json();
	console.log(body)
	return Backend.post("/user/auth", body);
}
