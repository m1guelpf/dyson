import { Metadata } from 'next'
import { Barricade } from '@/lib/icons'

export const metadata = {
	title: 'Dashboard',
} satisfies Metadata

const DashboardPage = () => {
	return (
		<div className="flex-1 flex flex-col items-center justify-center">
			<Barricade className="w-36 h-36 text-neutral-400" />
		</div>
	)
}

export default DashboardPage
