import { ORIGIN } from '$env/static/private';

export default {
	// eslint-disable-next-line @typescript-eslint/no-unnecessary-type-assertion
	baseUrl: (ORIGIN as string) + '/api',
	baseHeaders: {
		'Content-Type': 'application/json'
	},
	serverSessionCookieName: 'JSESSIONID'
};
