import type { Load } from '@sveltejs/kit';

export const authLoad: Load = async ({ session }) => {
	if (!session?.user) {
		return {
			status: 302,
			redirect: '/login',
		};
	}
	return {
		props: {
			user: session.user,
		},
	};
};
