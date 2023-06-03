import backendConfig from '$server/backend.config';
import type { Cookies } from '@sveltejs/kit';

type CookieHeader = {
	cookie: string;
};

export const transferSessionCookie = (cookies: Cookies): CookieHeader | Record<string, never> => {
	const sessionId = cookies.get(backendConfig.serverSessionCookieName);
	return sessionId ? { cookie: `${backendConfig.serverSessionCookieName}=${sessionId}` } : {};
};
