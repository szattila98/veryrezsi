import type { LogoutRequest, LogoutResponse } from '$mock/api/models/logout_model';

import { mockLogout } from '$mock/api/user/logout';

/** @type {import('@sveltejs/kit').RequestHandler} */
export async function post({ request }: { request: LogoutRequest }): Promise<LogoutResponse> {
	return mockLogout(request);
}
