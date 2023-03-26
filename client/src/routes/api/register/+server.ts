import serverConfig from '$server/backend.config';
import type { LoginRequestData } from '$shared/api/login';

/** @type {import('@sveltejs/kit').RequestHandler} */
export async function POST({ request }: { request: Request }) {
	const data: LoginRequestData = await request.json();
	const response = await fetch(serverConfig.baseUrl + '/user/register', {
		method: 'POST',
		headers: serverConfig.baseHeaders,
		body: JSON.stringify(data)
	});

	const options = {
		status: response.status
	};

	if (!response.ok) {
		return new Response('Registration failed', options);
	}

	return new Response('Registered', options);
}
