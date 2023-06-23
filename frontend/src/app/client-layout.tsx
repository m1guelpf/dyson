'use client'

import { SWRConfig } from 'swr'
import { FC, PropsWithChildren } from 'react'
import { IconContext } from '@phosphor-icons/react/dist/lib/context'

const fetcher = (path: string) => fetch(path).then(res => res.json())

export const ClientLayout: FC<PropsWithChildren<{}>> = ({ children }) => (
	<SWRConfig value={{ fetcher }}>
		<IconContext.Provider value={{ color: 'currentColor', size: '' }}>{children}</IconContext.Provider>
	</SWRConfig>
)
