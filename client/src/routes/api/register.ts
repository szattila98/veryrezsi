import Backend from '$services/BackendApi';
import type { RegisterResponse, RegisterRequestData } from '$shared/api/register';

/** @type {import('@sveltejs/kit').RequestHandler} */
export async function POST({ request }: { request: Request }): Promise<RegisterResponse> {
	const body: RegisterRequestData = await request.json();

	return mockRegister(body);
}
