import prisma from '@/db/prisma'
import Session from '@/lib/session'
import { cookies } from 'next/headers'
import { notFound } from 'next/navigation'

const UsersPage = async () => {
	const session = await Session.fromCookies(cookies())
	const user = await prisma.user.findUnique({ where: { id: session.userId } })
	if (!user) return notFound()
	const users = await prisma.user.findMany()

	return (
		<div>
			<div className="flex items-center justify-between">
				<h2 className="text-4xl text-neutral-200">Users</h2>
				<button className="border-2 text-neutral-300 border-neutral-300 px-4 py-2 hover:bg-neutral-300 hover:text-neutral-900 duration-200">
					Invite User
				</button>
			</div>
			<div className="mt-8 flow-root">
				<div className="-mx-4 -my-2 overflow-x-auto sm:-mx-6 lg:-mx-8">
					<div className="inline-block min-w-full py-2 align-middle sm:px-6 lg:px-8">
						<table className="min-w-full divide-y divide-gray-700">
							<thead>
								<tr>
									<th
										scope="col"
										className="py-3.5 pl-4 pr-3 text-left text-sm font-semibold text-white sm:pl-0"
									>
										Name
									</th>
									<th scope="col" className="px-3 py-3.5 text-left text-sm font-semibold text-white">
										Role
									</th>
									<th scope="col" className="relative py-3.5 pl-3 pr-4 sm:pr-0">
										<span className="sr-only">Edit</span>
									</th>
								</tr>
							</thead>
							<tbody className="divide-y divide-gray-800">
								{users.map(user => (
									<tr key={user.id}>
										<td className="whitespace-nowrap py-4 pl-4 pr-3 text-sm font-medium text-white sm:pl-0">
											{user.username}
										</td>
										<td className="whitespace-nowrap px-3 py-4 text-sm text-gray-300 capitalize">
											{user.role.toLowerCase()}
										</td>
										<td className="relative whitespace-nowrap py-4 pl-3 pr-4 text-right text-sm font-medium sm:pr-0">
											<a href="#" className="text-indigo-400 hover:text-indigo-300">
												Edit<span className="sr-only">, {user.username}</span>
											</a>
										</td>
									</tr>
								))}
							</tbody>
						</table>
					</div>
				</div>
			</div>
		</div>
	)
}

export default UsersPage
