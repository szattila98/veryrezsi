import { redirect } from '@sveltejs/kit';
import type { LayoutServerLoad, LayoutRouteId } from './$types';

export const load: LayoutServerLoad = ({ locals, route }) => {
	if (!locals.user && !isPublicRoute(route.id)) {
		throw redirect(307, '/login?referrer=' + route.id); // id refers to route files, this will not support [slug] pages
	}

	const { user } = locals; // locals.user set by hooks.server.ts/handle()

	return {
		user
	};
};

function isPublicRoute(route: LayoutRouteId) {
	const publicRoutes = ['login', 'register'];

	if (!route) {
		return false;
	}

	return publicRoutes.some((publicRoute) => {
		return route.endsWith(publicRoute);
	});
}
