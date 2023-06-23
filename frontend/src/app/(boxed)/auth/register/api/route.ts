import * as z from 'zod'
import prisma from '@/db/prisma'
import Session from '@/lib/session'
import { binaryToBase64url, tap } from '@/lib/utils'
import { NextRequest, NextResponse } from 'next/server'
import { VerifiedRegistrationResponse, verifyRegistrationResponse } from '@simplewebauthn/server'

const usernameSchema = z
	.string()
	.min(3)
	.max(15)
	.regex(/^@[a-zA-Z][a-zA-Z0-9._]{2,14}$/)

export const GET = async (req: NextRequest): Promise<NextResponse> => {
	const username = usernameSchema.safeParse(req.nextUrl.searchParams.get('username'))
	if (!username.success) return NextResponse.json({ error: username.error.message }, { status: 422 })

	const user = await prisma.user.findUnique({ where: { username: username.data } })

	if (user) return NextResponse.json({ error: 'Username is not available' }, { status: 422 })
	return NextResponse.json({ available: true })
}

const registerRequestSchema = z.object({
	username: usernameSchema,
	credential: z.object({}).nonstrict(),
})

export const POST = async (req: NextRequest): Promise<NextResponse> => {
	const session = await Session.fromRequest(req)
	const { username, credential } = await req.json()

	const registerRequest = registerRequestSchema.safeParse({ username, credential })
	if (!registerRequest.success) return NextResponse.json({ error: registerRequest.error.message }, { status: 422 })

	let verification: VerifiedRegistrationResponse
	try {
		verification = await verifyRegistrationResponse({
			response: credential,
			requireUserVerification: true,
			expectedOrigin: req.nextUrl.origin,
			expectedRPID: req.nextUrl.hostname,
			expectedChallenge: session.challenge!,
		})
	} catch (error) {
		console.error(error)
		return NextResponse.json({ error: (error as Error).message }, { status: 422 })
	}

	if (!verification.verified) return NextResponse.json({ error: 'Could not verify credential' }, { status: 422 })
	const { credentialID, credentialPublicKey } = verification.registrationInfo ?? {}
	if (credentialID == null || credentialPublicKey == null) {
		return NextResponse.json({ error: 'Registration failed' }, { status: 422 })
	}

	const user = await prisma.user.create({
		data: {
			username,
			credentials: {
				create: {
					externalId: binaryToBase64url(credentialID),
					publicKey: Buffer.from(credentialPublicKey),
				},
			},
		},
	})

	session.userId = user.id
	session.challenge = undefined

	return tap(NextResponse.json(user), res => session.persist(res))
}
