import NavBar from './NavBar'
import prisma from '@/db/prisma'
import Session from '@/lib/session'
import { cookies } from 'next/headers'
import { notFound } from 'next/navigation'
import { FC, PropsWithChildren } from 'react'

const AppLayout: FC<PropsWithChildren<{}>> = async ({ children }) => {
	const session = await Session.fromCookies(cookies())
	const user = await prisma.user.findUnique({ where: { id: session.userId } })
	console.log(user)

	if (!user) return notFound()

	return (
		<div className="min-h-screen flex p-4 gap-4">
			<div className="bg-neutral-950">
				<NavBar user={user} />
			</div>
			<div className="flex-1 bg-neutral-900 border rounded-xl border-neutral-800 flex flex-col p-10">
				{children}
			</div>
		</div>
	)
}

export default AppLayout
