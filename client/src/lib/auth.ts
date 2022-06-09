import type { Load } from '@sveltejs/kit';

/**
 * Checks if the user is logged in and provides its details as props
 * Load function implementation for SvelteKit. Should be used in module context of a page or layout to obtain user prop.
 *
 * @link SvelteKit docs on Load function: https://kit.svelte.dev/docs/loading
 * @typedef  {Object} loadContext SvelteKit passing this object, that is available before page render.
 * @property {Object} loadContext.session Session object that will not be available after render.
 * @returns  {Object} User details as props.
 */
export const loadUser: Load = async ({ session }) => {
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
