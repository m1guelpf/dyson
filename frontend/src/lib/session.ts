import { COOKIE_NAME } from './consts'
import { NextRequest, NextResponse } from 'next/server'
import { sealData, unsealData } from 'iron-session/edge'
import { ResponseCookies } from 'next/dist/compiled/@edge-runtime/cookies'
import { ReadonlyRequestCookies } from 'next/dist/server/web/spec-extension/adapters/request-cookies'

export const SESSION_OPTIONS = {
	ttl: 60 * 60 * 24 * 30, // 30 days
	password: process.env.SESSION_SECRET!,
}

export type ISession = {
	challenge?: string
	userId?: string
}

class Session {
	challenge?: string
	userId?: string

	constructor(session?: ISession) {
		this.challenge = session?.challenge
		this.userId = session?.userId
	}

	static async fromCookies(cookies: ReadonlyRequestCookies): Promise<Session> {
		const sessionCookie = cookies.get(COOKIE_NAME)?.value

		if (!sessionCookie) throw new Error('Not authenticated')
		return new Session(await unsealData<ISession>(sessionCookie, SESSION_OPTIONS))
	}

	static async fromRequest(req: NextRequest): Promise<Session> {
		const sessionCookie = req.cookies.get(COOKIE_NAME)?.value

		if (!sessionCookie) return new Session()
		return new Session(await unsealData<ISession>(sessionCookie, SESSION_OPTIONS))
	}

	clear(res: NextResponse | ResponseCookies): Promise<void> {
		this.challenge = undefined
		this.userId = undefined

		return this.persist(res)
	}

	toJSON(): ISession {
		return { challenge: this.challenge, userId: this.userId }
	}

	async persist(res: NextResponse | ResponseCookies): Promise<void> {
		let cookies: ResponseCookies
		if (isCookies(res)) cookies = res
		else cookies = res.cookies

		cookies.set(COOKIE_NAME, await sealData(this.toJSON(), SESSION_OPTIONS), {
			httpOnly: true,
			secure: process.env.NODE_ENV === 'production',
		})
	}
}

const isCookies = (cookies: NextResponse | ResponseCookies): cookies is ResponseCookies => {
	return (cookies as ResponseCookies).set !== undefined
}

export default Session
