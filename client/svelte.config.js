import adapter from '@sveltejs/adapter-node';
import { vitePreprocess } from '@sveltejs/kit/vite';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://kit.svelte.dev/docs/integrations#preprocessors
	// for more information about preprocessors
	preprocess: vitePreprocess(),
	kit: {
		// adapter-auto only supports some environments, see https://kit.svelte.dev/docs/adapter-auto for a list.
		// If your environment is not supported or you settled on a specific environment, switch out the adapter.
		// See https://kit.svelte.dev/docs/adapters for more information about adapters.
		adapter: adapter(),
		alias: {
			$routes: 'src/routes',
			'$routes/*': 'src/routes/*',
			$shared: 'src/lib/shared',
			'$shared/*': 'src/lib/shared/*',
			$api: 'src/lib/api',
			'$api/*': 'src/lib/api/*',
			$server: 'src/lib/server',
			'$server/*': 'src/lib/server/*'
		}
	}
};

export default config;
