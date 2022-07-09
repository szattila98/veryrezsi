import { sveltekit } from '@sveltejs/kit/vite';
import { resolve } from 'path';

/** @type {import('vite').UserConfig} */
const config = {
	plugins: [sveltekit()],
	resolve: {
		alias: {
			$mock: resolve('./src/mock'),
			$routes: resolve('./src/routes'),
			$api: resolve('./src/routes/api'),
		},
	},
};

export default config;
