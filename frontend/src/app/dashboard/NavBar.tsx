'use client'

import { FC } from 'react'
import Link from 'next/link'
import Image from 'next/image'
import { Role, User } from '@prisma/client'
import { usePathname } from 'next/navigation'
import purpleAvatar from '@images/avatars/purple.png'
import { ChartLine, HardDrives, Icon, IconWeight, Package, Planet, Sparkle, UsersThree } from '@phosphor-icons/react'

const NavBar: FC<{ user: User }> = ({ user }) => (
	<div className="bg-neutral-900 p-2 rounded-xl border border-neutral-800 space-y-4 flex flex-col items-center">
		<button className="p-1 rounded-full flex items-center justify-center border-t border-neutral-800 hover:bg-neutral-800 hover:border-neutral-700 duration-300">
			<Image src={purpleAvatar} width={40} height={40} alt="Dyson" placeholder="blur" />
		</button>
		<hr className="h-full w-full border-neutral-800" />
		<NavLink href="/dashboard" icon={Planet} weight="fill" />
		<NavLink href="/dashboard/server" icon={HardDrives} weight="fill" />
		<NavLink href="/dashboard/packages" icon={Package} weight="fill" />
		<NavLink href="/dashboard/interface" icon={Sparkle} weight="fill" />
		<NavLink href="/dashboard/stats" icon={ChartLine} weight="bold" />
		{user.role != Role.USER && <NavLink href="/dashboard/users" icon={UsersThree} weight="bold" />}
	</div>
)

const NavLink: FC<{ href: string; icon: Icon; weight?: IconWeight }> = ({ href, icon: Icon, weight }) => {
	const pathname = usePathname()

	return (
		<Link
			href={href}
			aria-selected={pathname == href}
			className="group flex items-center space-x-4 bg-neutral-950 p-2 rounded-lg border-t border-neutral-800 hover:bg-neutral-950/10 aria-selected:bg-neutral-800 aria-selected:border-neutral-700 duration-200"
		>
			<Icon
				weight={weight}
				className="text-neutral-700 group-hover:text-neutral-600 group-aria-selected:text-neutral-500 w-6 h-6 duration-200"
			/>
		</Link>
	)
}

export default NavBar
