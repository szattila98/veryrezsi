import { mockLogout } from '$mock/api/user/logout';
import type { LogoutRequest, LogoutResponse } from '$shared/api/logout';

/** @type {import('@sveltejs/kit').RequestHandler} */
export async function POST({ request }: { request: LogoutRequest }): Promise<LogoutResponse> {
	return mockLogout(request);
}
