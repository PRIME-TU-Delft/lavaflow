/** @type {import('tailwindcss').Config}*/
const config = {
	content: [
		'./src/**/*.{html,js,svelte,ts}',
		'./node_modules/flowbite-svelte/**/*.{html,js,svelte,ts}'
	],

	daisyui: {
		themes: [
			{
				dark: {
					...require("daisyui/src/theming/themes")["[data-theme=dark]"],
					"primary": "#e11d48",
					"secondary": "#fda4af",
				},
				light: {
					...require("daisyui/src/theming/themes")["[data-theme=light]"],
					"primary": "#e11d48",
					"secondary": "#fda4af",
				},
			},
		],
	},

	theme: {
		extend: {}
	},
	plugins: [require('@tailwindcss/typography'), require('daisyui')]
};

module.exports = config;
