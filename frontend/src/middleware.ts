import Session from './lib/session'
import { NextResponse } from 'next/server'
import type { NextRequest } from 'next/server'

export const middleware = async (request: NextRequest) => {
	const path = request.nextUrl.pathname
	const session = await Session.fromRequest(request)

	if (!session.userId) {
		if (path.startsWith('/auth')) return
		return NextResponse.redirect(new URL('/auth/login', request.url))
	}

	if (path == '/' || path.startsWith('/auth')) {
		return NextResponse.redirect(new URL('/dashboard', request.url))
	}
}

export const config = {
	matcher: ['/', '/auth/:path*', '/dashboard/:path*'],
}
