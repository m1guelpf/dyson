import { registerOTel } from '@vercel/otel'

export function register() {
	registerOTel('dyson-frontend')
}
