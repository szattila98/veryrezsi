const esModules = ['@smui', 'svelte-forms'].join('|');

module.exports = {
	transform: {
		'^.+\\.svelte$': ['svelte-jester', { preprocess: './svelte.config.test.cjs' }],
		'^.+\\.ts$': 'ts-jest',
		'^.+\\.js$': require.resolve('babel-jest'),
	},
	moduleFileExtensions: ['js', 'ts', 'svelte'],
	moduleNameMapper: {
		'^\\$lib(.*)$': '<rootDir>/src/lib$1',
		'^\\$app(.*)$': [
			'<rootDir>/.svelte-kit/dev/runtime/app$1',
			'<rootDir>/.svelte-kit/build/runtime/app$1',
		],
		'^\\$mock(.*)$': '<rootDir>/src/mock$1',
	},
	modulePathIgnorePatterns: ['util'],
	setupFilesAfterEnv: ['<rootDir>/jest-setup.ts'],
	collectCoverageFrom: ['src/**/*.{ts,tsx,svelte,js,jsx}'],
	transformIgnorePatterns: [`/node_modules/(?!${esModules}).+\\.js$`],
};
