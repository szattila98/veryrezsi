import { generateSessionCookieForUser, USERID_IS_NOT_A_NUMBER_MESSGE } from '$mock/api/user/login';
import { Cookie, cookieToObject} from './util/CookieTestUtil';

describe('Session cookie generation', () => {
	test('should result in the same session id, for the same user ids', () => {
		const username = 'JackTheRapper_09';
		const username2 = 'JackTheReaper_9';

		const result: SessionCookieContent = getGeneratedSessionCookieObject(username);
		const result2: SessionCookieContent = getGeneratedSessionCookieObject(username2);

		expect(result.JSESSIONID).toEqual('aXRzanVzdGFjb29raWVmb3JfOQ%3D%3D');
		expect(result2.JSESSIONID).toEqual('aXRzanVzdGFjb29raWVmb3JfOQ%3D%3D');
	});

	test('should throw an error when the last part of the username is not a number', () => {
		const username = 'admin_mf';
		expect(() => getGeneratedSessionCookieObject(username)).toThrowError(USERID_IS_NOT_A_NUMBER_MESSGE);
	});
})

const getGeneratedSessionCookieObject = (username: string) : SessionCookieContent => {
	return cookieToObject((generateSessionCookieForUser(username) as Cookie)['Set-Cookie']) as SessionCookieContent
}

type SessionCookieContent = {
	'JSESSIONID': string;
	'Max-Age': number;
	'SameSite': string;
	'Path': string;
}