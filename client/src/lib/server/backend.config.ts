import { REZSI_API_BASE_URL } from '$env/static/private';

export default {
	baseUrl: REZSI_API_BASE_URL,
	baseHeaders: {
		'Content-Type': 'application/json'
	},
	serverSessionCookieName: 'JSESSIONID'
};
