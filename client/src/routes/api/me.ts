import Backend from '$services/BackendApi';
import type { MeResponse } from '$shared/api/me';

/** @type {import('@sveltejs/kit').RequestHandler} */
export async function GET(): Promise<MeResponse> {
	return getSession();
}

export async function getSession() {
	return Backend.get("/user/me")
}
