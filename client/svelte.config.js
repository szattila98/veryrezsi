import adapter from '@sveltejs/adapter-auto';
import preprocess from 'svelte-preprocess';
import { resolve } from 'path';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	compilerOptions: {
		enableSourcemap: true,
	},
	// Consult https://github.com/sveltejs/svelte-preprocess
	// for more information about preprocessors
	preprocess: preprocess({
		sourceMap: true,
	}),

	kit: {
		adapter: adapter(),
		vite: {
			resolve: {
				alias: {
					$mock: resolve('./src/mock'),
					$routes: resolve('./src/routes'),
					$api: resolve('./src/routes/api'),
				},
			},
		},
	},
};

export default config;
