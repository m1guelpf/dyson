'use client'

import useSWR from 'swr'
import { useRouter } from 'next/navigation'
import { supported, get } from '@github/webauthn-json'
import { FormEvent, useCallback, useState } from 'react'

const Home = () => {
	const router = useRouter()
	const [username, setUsername] = useState('')
	const { data: challenge } = useSWR<string>('/auth')

	const login = useCallback(
		async (event: FormEvent<HTMLFormElement>) => {
			event.preventDefault()

			const available = await PublicKeyCredential.isUserVerifyingPlatformAuthenticatorAvailable()
			if (!available || !supported() || !challenge) {
				alert("Seems like your browser doesn't support passkeys...")
				throw new Error('WebAuthn is not supported')
			}

			const credential = await get({ publicKey: { challenge, timeout: 60000, userVerification: 'required' } })

			const result = await fetch('/auth/login/api', {
				method: 'POST',
				body: JSON.stringify({ username, credential }),
				headers: {
					'Content-Type': 'application/json',
				},
			})

			if (result.ok) {
				return router.push('/dashboard')
			}

			alert('Something went wrong...')
		},
		[username, challenge]
	)

	return (
		<div className="flex flex-col items-center space-y-6">
			<p className="text-neutral-300">👋 Hello there! Please sign in to continue.</p>
			<form onSubmit={login} className="w-full max-w-md">
				<label htmlFor="username" className="block text-neutral-300">
					Username
				</label>
				<input
					required
					autoFocus
					type="text"
					id="username"
					value={username}
					placeholder="@miguel"
					pattern="^@[a-zA-Z][a-zA-Z0-9._]{2,14}$"
					onChange={event => setUsername(event.target.value)}
					className="mt-1 block w-full p-3 bg-neutral-800 text-neutral-300 placeholder:text-neutral-600 focus:outline-none"
				/>
				<div className="mt-2 flex w-full justify-end">
					<button type="submit" className="bg-neutral-800  text-neutral-400 px-5 py-2 text-lg">
						Sign in
					</button>
				</div>
			</form>
		</div>
	)
}

export default Home
