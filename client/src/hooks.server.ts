import backendConfig from '$server/backend.config';
import serverConfig from '$server/server.config';
import type { Handle, RequestEvent } from '@sveltejs/kit';
import type { User } from '$shared/domain';

// Invoked for each endpoint called, and initially for SSR router
export const handle: Handle = async ({ event, resolve }) => {
	const { cookies } = event;
	const sessionId = cookies.get(serverConfig.clientSessionCookieName);

	if (sessionId) await attachUserToRequestEvent(sessionId, event);
	if (!event.locals.user) cookies.delete(serverConfig.clientSessionCookieName);

	return await resolve(event);
};

// Attach authorization to each server request
async function attachUserToRequestEvent(sessionId: string, event: RequestEvent) {
	const user: User | null = await callWhoAmIApi(sessionId);

	if (user) {
		event.locals.user = {
			id: user.id,
			email: user.email,
			username: user.username
		};
	}
}

async function callWhoAmIApi(sessionId: string): Promise<User | null> {
	const response = await fetch(backendConfig.baseUrl + '/user/me', {
		method: 'GET',
		headers: {
			Cookie: backendConfig.serverSessionCookieName + '=' + sessionId,
			...backendConfig.baseHeaders
		}
	});

	if (!response.ok) {
		return Promise.resolve(null);
	}

	return response.json();
}
