import { twMerge } from 'tailwind-merge'
import { clsx, type ClassValue } from 'clsx'

export const tap = async <T>(value: T, cb: (value: T) => Promise<unknown>): Promise<T> => {
	await cb(value)
	return value
}

export const cn = (...inputs: ClassValue[]): string => twMerge(clsx(inputs))

export const binaryToBase64url = (bytes: Uint8Array) => {
	let str = ''

	bytes.forEach(charCode => {
		str += String.fromCharCode(charCode)
	})

	return btoa(str).replace(/\+/g, '-').replace(/\//g, '_').replace(/=/g, '')
}
