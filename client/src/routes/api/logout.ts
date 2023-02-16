import Backend from '$services/BackendApi';
import type { LogoutResponse } from '$shared/api/logout';

/** @type {import('@sveltejs/kit').RequestHandler} */
export async function POST(): Promise<LogoutResponse> {
	return Backend.post("/user/logout");
}
