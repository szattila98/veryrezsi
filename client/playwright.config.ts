import type { PlaywrightTestConfig } from '@playwright/test';

const config: PlaywrightTestConfig = {
	webServer: {
		command: 'npm run build:ci && npm run preview',
		port: 4173
	},
	testDir: 'tests'
};

export default config;
