import { generateSessionCookieForUser } from '$mock/api/user/login';

describe('Session cookie generation', () => {
	test('TBD', () => {
		const username = 'JackTheRapper_09';

		const result = generateSessionCookieForUser(username);

		expect(result).toEqual({});
	})
})