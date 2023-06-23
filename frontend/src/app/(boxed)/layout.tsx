import { FC, PropsWithChildren } from 'react'

const BoxLayout: FC<PropsWithChildren<{}>> = ({ children }) => {
	return (
		<div className="min-h-screen flex p-5">
			<div className="bg-neutral-900 border rounded-xl border-neutral-800 flex-1 flex flex-col items-center justify-center">
				{children}
			</div>
		</div>
	)
}

export default BoxLayout
