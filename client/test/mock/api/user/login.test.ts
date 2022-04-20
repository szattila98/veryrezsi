import { generateSessionCookieForUser } from '$mock/api/user/login';

describe('Session cookie generation', () => {
	test('TBD', () => {
		const username = 'JackTheRapper_09';
		const username2 = 'JackTheRapper_9';

		const result: SessionCookie = generateSessionCookieForUser(username) as SessionCookie;
		const result2: SessionCookie = generateSessionCookieForUser(username2) as SessionCookie;

		expect(cookieToObject(result['Set-Cookie']).JSESSIONID).toEqual('aXRzanVzdGFjb29raWVmb3JfOQ%3D%3D');
		expect(cookieToObject(result2['Set-Cookie']).JSESSIONID).toEqual('aXRzanVzdGFjb29raWVmb3JfOQ%3D%3D');
	})
})

const cookieToObject = (cookieString: string): CookieContent => {
	const valueArray: string[] = cookieString.split('; ');
	let result = {};

	for (const i in valueArray) {
		const valueParts = valueArray[i].split('=');

		if(valueParts.length == 2) {
			result = {...result, ...{[valueParts[0]]: valueParts[1]}};
		}
	}

	return result as CookieContent;
}


type CookieContent = {
	'JSESSIONID': string;
	'Max-Age': number;
	'SameSite': string;
	'Path': string;
}

type SessionCookie = {
	'Set-Cookie': string;
};