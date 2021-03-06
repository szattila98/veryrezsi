import { mockWhoAmI } from '$mock/api/user/me';
import type { MeResponse, MeRequestData } from '$shared/api/me';

/** @type {import('@sveltejs/kit').RequestHandler} */
export async function get({ request }: { request: Request }): Promise<MeResponse> {
	const body: MeRequestData = await request.json();
	return mockWhoAmI(body);
}

export async function getSession(sessionId: string) {
	const body: MeRequestData = { sessionId: sessionId };
	return mockWhoAmI(body);
}
