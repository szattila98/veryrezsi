import { REZSI_API_BASE_URL } from '$env/static/private';

export default {
	// eslint-disable-next-line @typescript-eslint/no-unnecessary-type-assertion
	baseUrl: REZSI_API_BASE_URL as string,
	baseHeaders: {
		'Content-Type': 'application/json'
	},
	serverSessionCookieName: 'JSESSIONID'
};
