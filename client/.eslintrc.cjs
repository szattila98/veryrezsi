module.exports = {
	root: true,
	parser: '@typescript-eslint/parser',
	extends: [
		'plugin:@typescript-eslint/recommended',
		'plugin:@typescript-eslint/recommended-requiring-type-checking',
		'prettier'
	],
	plugins: ['svelte3', '@typescript-eslint', 'prettier'],
	ignorePatterns: ['*.cjs', 'playwright.config.ts', 'svelte.config.js'],
	overrides: [{ files: ['*.svelte'], processor: 'svelte3/svelte3' }],
	settings: {
		'svelte3/typescript': () => require('typescript')
	},
	parserOptions: {
		sourceType: 'module',
		ecmaVersion: 2020,
		project: true,
		tsconfigRootDir: __dirname
	},
	env: {
		browser: true,
		es2017: true,
		node: true
	},
	rules: {
		'prettier/prettier': 'warn',
		'@typescript-eslint/semi': 'warn'
	}
};
