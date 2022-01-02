const config = {
	mode: 'jit',
	content: ['./src/**/*.{html,js,svelte,ts}'],
	theme: {
		extend: {}
	},
	// Only add this if you installed the TailwindCSS-plugins:
	plugins: [require('@tailwindcss/typography'), require('@tailwindcss/forms')]
};

module.exports = config;
