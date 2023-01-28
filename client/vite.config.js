import { sveltekit } from '@sveltejs/kit/vite';
import { resolve } from 'path';

/** @type {import('vite').UserConfig} */
const config = {
	plugins: [sveltekit()],
	resolve: {
		alias: {
			$mock: resolve('./src/mock'),
			$api: resolve('./src/routes/api'),
			$routes: resolve('./src/routes'),
			$services: resolve('./src/services'),
		},
	},
};

export default config;
