import { BASE_URL } from '$env/static/private';

export default {
	baseUrl: BASE_URL,
	baseHeaders: {
		'Content-Type': 'application/json'
	},
	sessionCookieName: 'JSESSIONID'
};
