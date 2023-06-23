/** @type {import('next').NextConfig} */
const nextConfig = {
	output: 'standalone',
	experimental: {
		instrumentationHook: true,
	},
	modularizeImports: {
		'@phosphor-icons/react': {
			transform: '@phosphor-icons/react/dist/icons/{{member}}',
		},
	},
}

module.exports = nextConfig
