export type Cookie = {
	'Set-Cookie': string;
};

export const cookieToObject = (cookieString: string) => {
	const valueArray: string[] = cookieString.split('; ');
	let result = {};

	for (const i in valueArray) {
		try {
			result = { ...result, ...parseCookieObjectValue(valueArray[i]) };
		} catch (e) {
			console.debug('Not key-value pair found in cookie. Will not parse it to the object');
		}
	}

	return result;
};

const parseCookieObjectValue = (cookieObjectValue: string): object => {
	const valueParts = cookieObjectValue.split('=');

	if (valueParts.length != 2) {
		throw Error('Not key value');
	} else {
		return { [valueParts[0]]: valueParts[1] };
	}
};
