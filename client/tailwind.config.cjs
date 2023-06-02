/** @type {import('tailwindcss').Config} */
module.exports = {
	content: ['./src/**/*.{html,js,svelte,ts}'],
	theme: {
		extend: {
			// https://v2.tailwindcss.com/docs/customizing-colors#extending-the-defaults
			colors: {
				primary: '#3B82F6', // blue-500
				primarydark: '#1D4ED8', // blue-700
				secondary: '#22C55E', // green-500
				secondarydark: '#16A34A', // green-600
				fontblack: '#3F3F46' // grey-700
			}
		}
	},
	plugins: []
};
