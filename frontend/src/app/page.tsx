'use client'

import Logo from './components/Logo'
import { motion } from 'framer-motion'
import LocalFont from 'next/font/local'
import GitHub from './components/icons/GitHub'
import { Darker_Grotesque } from 'next/font/google'

const rocaTwo = LocalFont({
	preload: true,
	display: 'swap',
	adjustFontFallback: false,
	src: 'fonts/RocaOne.woff2',
})

const darkerGrotesque = Darker_Grotesque({
	preload: true,
	weight: '900',
	display: 'swap',
	subsets: ['latin'],
	adjustFontFallback: false,
})

const HomePage = () => {
	return (
		<div className="flex min-h-screen p-4 md:p-10 flex-col items-center justify-between">
			<motion.div
				initial={{ opacity: 0 }}
				animate={{ opacity: 1 }}
				transition={{ duration: 0.7, delay: 0.3 }}
				className="flex items-center justify-between w-full max-w-screen-2xl mx-auto"
			>
				<div className="flex items-center space-x-8">
					<Logo className="w-10 h-10 text-neutral-400/40" />
				</div>
				<div className="flex items-center space-x-4 md:space-x-8">
					<a
						target="_blank"
						className="text-sm text-neutral-500 hover:text-neutral-600 transition-colors duration-200"
						href="https://github.com/m1guelpf/dyson#readme"
					>
						Docs
					</a>
					<motion.a
						target="_blank"
						whileTap={{ scale: 0.98 }}
						whileHover={{ scale: 1.02 }}
						href="https://github.com/m1guelpf/dyson"
						className="flex items-center space-x-2 text-sm text-neutral-500 bg-neutral-100 py-2 px-3 -my-2 -mx-3 rounded-md"
					>
						<span>Star on GitHub</span>
						<GitHub className="w-4 h-4" />
					</motion.a>
				</div>
			</motion.div>
			<div className="flex flex-col items-center md:-mt-16">
				<div>
					<motion.svg
						initial={{ opacity: 0, scale: '60%' }}
						animate={{ opacity: 1, scale: '100%' }}
						className="-mb-24 md:-mb-52"
						xmlns="http://www.w3.org/2000/svg"
						viewBox="0 0 157.027 157.031"
					>
						<defs>
							<linearGradient id="big-globe" x1="50%" x2="50%" y1="0%" y2="87.172%">
								<stop offset="0%" stop-color="#9F26DF" />
								<stop offset="33.479%" stop-color="#C336E2" stop-opacity=".929" />
								<stop offset="56.57%" stop-color="#EA71EC" stop-opacity=".563" />
								<stop offset="96.255%" stop-color="#E8D5E8" stop-opacity="0" />
							</linearGradient>
						</defs>
						<path
							fill="url(#big-globe)"
							d="M105.03 136.879 73.78 115.19c-1.963-1.364-1.824-3.36.31-4.438l32.233-16.29c2.133-1.077 3.812-.006 3.73 2.384l-1.305 38.168c-.081 2.389-1.754 3.23-3.718 1.865Zm9.676-53.808 22.113-31.135c1.385-1.95.563-3.63-1.824-3.738l-38.001-1.715c-2.388-.108-3.488 1.564-2.447 3.716l15.748 32.505c1.042 2.152 3.027 2.317 4.411.367Zm.09 51.61 25.914-29.204c1.586-1.789 1.032-3.877-1.232-4.643l-21.882-7.392c-2.264-.766-4.183.563-4.265 2.952l-1.27 37.194c-.082 2.39 1.148 2.881 2.735 1.093Zm-52.57 9.637 38.857-2.882c2.384-.178 2.728-1.436.764-2.8l-30.392-21.09c-1.964-1.365-4.196-.626-4.96 1.638l-7.211 21.34c-.765 2.264.559 3.972 2.942 3.794Zm49.586-126.73 25.408 22.54c1.788 1.586 3.105.934 2.928-1.448l-.691-9.315c-.178-2.383-2.175-4.959-4.44-5.722l-22.338-7.55c-2.265-.763-2.655-.09-.867 1.494Zm-10.981-.035-7.209 21.339c-.765 2.264.562 4.203 2.95 4.311l36.956 1.666c2.388.108 2.879-1.104 1.091-2.688l-29.147-25.86c-1.787-1.586-3.876-1.032-4.641 1.232Zm6.178-6.813 18.356 6.202c2.265.764 2.49.306.5-1.02l-16.927-11.28c-1.681-1.12-3.586-.47-4.232 1.443a3.683 3.683 0 0 0 2.303 4.655Zm-38.853 93.345 6.9-20.428a4.36 4.36 0 0 0-2.725-5.508l-19.95-6.741c-2.263-.766-3.263.367-2.22 2.518l14.71 30.365c1.041 2.151 2.52 2.058 3.285-.206Zm17.628-52.178c.765-2.264-.354-3.234-2.487-2.156l-30.11 15.217c-2.134 1.078-2.026 2.587.237 3.353l19.951 6.738a4.356 4.356 0 0 0 5.507-2.724l6.902-20.428ZM73.731 107.28l30.11-15.217c2.133-1.078 2.026-2.586-.239-3.35l-19.949-6.741a4.36 4.36 0 0 0-5.508 2.727l-6.902 20.426c-.765 2.264.354 3.233 2.488 2.155ZM43.698 60.638l1.27-37.212c.082-2.389-1.149-2.881-2.735-1.092L16.316 51.557c-1.586 1.786-1.031 3.876 1.234 4.64l21.882 7.394c2.265.764 4.185-.564 4.266-2.953Zm4.579-38.62-1.302 38.168c-.081 2.389 1.597 3.46 3.73 2.382l32.234-16.29c2.134-1.078 2.273-3.075.309-4.44L51.995 20.153c-1.963-1.364-3.636-.524-3.718 1.865Zm95.372 19.768c.177 2.383 1.785 5.631 3.572 7.218l4.084 3.626c1.788 1.587 2.382 1.133 1.321-1.008l-7.774-15.692c-1.06-2.141-1.783-1.944-1.606.439l.403 5.417ZM22.032 108.838l38.003 1.71c2.388.108 3.488-1.564 2.446-3.715L46.732 74.327c-1.041-2.151-3.026-2.317-4.41-.367L20.206 105.1c-1.384 1.95-.563 3.63 1.825 3.738ZM94.8 12.714l-38.857 2.88c-2.384.18-2.728 1.437-.764 2.802l30.394 21.09c1.964 1.364 4.196.626 4.961-1.638l7.209-21.341c.764-2.264-.56-3.972-2.943-3.793Zm12.068 70.393-14.71-30.364c-1.042-2.152-2.52-2.059-3.285.205l-6.903 20.43a4.36 4.36 0 0 0 2.725 5.509l19.95 6.739c2.266.766 3.265-.367 2.223-2.519ZM78.595 1.14l18.368 6.206a3.685 3.685 0 0 0 4.656-2.304c.646-1.914-.474-3.586-2.49-3.715L78.814.027c-2.385-.152-2.484.348-.22 1.112Zm59.994 53.938-21.55 30.343c-1.386 1.95-.665 4.17 1.6 4.935l21.88 7.392c2.265.764 3.972-.56 3.795-2.942l-2.887-38.94c-.178-2.382-1.454-2.738-2.838-.788Zm-88.572 91.216-18.373-6.206c-2.263-.766-2.488-.307-.5 1.016l16.944 11.288c1.68 1.12 3.586.471 4.233-1.443a3.684 3.684 0 0 0-2.304-4.655Zm51.892-1.65-33.872 2.514c-2.384.176-2.482.946-.217 1.712l22.336 7.545c2.264.766 5.415-.07 7.001-1.859l6.201-6.982c1.586-1.789.935-3.107-1.45-2.93Zm-23.49 11.247-18.357-6.202a3.682 3.682 0 0 0-4.654 2.304c-.647 1.912.472 3.584 2.488 3.713l20.302 1.299c2.386.153 2.485-.348.22-1.114Zm-22.224-16.41 7.211-21.34c.765-2.264-.562-4.204-2.95-4.312l-36.915-1.663c-2.389-.109-2.88 1.103-1.093 2.69l29.108 25.856c1.787 1.587 3.875 1.033 4.64-1.23Zm64.494 3.764-5.447.408c-2.384.176-5.631 1.784-7.218 3.573l-3.605 4.06c-1.587 1.79-1.132 2.383 1.011 1.325l15.697-7.762c2.143-1.058 1.945-1.78-.438-1.604Zm35.197-64.63-6.2 18.353a3.685 3.685 0 0 0 2.303 4.655c1.913.647 3.583-.473 3.713-2.49L157 78.835c.152-2.385-.349-2.485-1.114-.22Zm-16.444 33.2-22.546 25.41c-1.586 1.79-.934 3.105 1.45 2.929l9.317-.696c2.384-.176 4.96-2.175 5.724-4.439l7.547-22.338c.765-2.264.094-2.654-1.492-.865Zm5.197-56.695 2.514 33.872c.177 2.383.948 2.48 1.713.216l7.544-22.331c.765-2.264-.07-5.415-1.858-7.002l-6.985-6.202c-1.788-1.587-3.105-.936-2.928 1.447Zm6.307 49.59a3.683 3.683 0 0 0-4.655 2.304l-6.201 18.351c-.765 2.264-.307 2.489 1.02.501l11.279-16.923c1.12-1.683.47-3.586-1.443-4.233ZM36.321 13.795l5.449-.403c2.383-.176 5.63-1.784 7.216-3.573L52.6 5.74c1.586-1.788 1.13-2.385-1.012-1.326L35.881 12.19c-2.142 1.059-1.945 1.783.439 1.606Zm-18.735 31.42L40.104 19.82c1.586-1.788.934-3.106-1.45-2.93l-9.29.688c-2.383.176-4.959 2.172-5.724 4.436L16.093 44.35c-.765 2.264-.093 2.654 1.493.866ZM1.14 78.42l6.2-18.353a3.685 3.685 0 0 0-2.302-4.655c-1.913-.647-3.584.473-3.714 2.489L.027 78.2c-.152 2.384.349 2.484 1.114.22Zm4.94-26.098a3.682 3.682 0 0 0 4.655-2.302l6.2-18.35c.765-2.265.307-2.49-1.018-.501L4.638 48.092c-1.119 1.68-.47 3.583 1.444 4.23Zm39.133 87.122L19.8 116.898c-1.788-1.585-3.107-.933-2.932 1.452l.685 9.31c.175 2.383 2.17 4.959 4.435 5.722l22.358 7.556c2.266.764 2.655.091.867-1.494Zm9.9-127.053L88.99 9.877c2.384-.177 2.482-.947.218-1.713L66.85.611c-2.264-.766-5.413.072-6.998 1.86l-6.19 6.988c-1.585 1.788-.931 3.108 1.452 2.932ZM18.431 101.97l21.56-30.357c1.384-1.95.664-4.169-1.601-4.935l-21.884-7.392c-2.264-.766-3.972.558-3.796 2.94l2.884 38.954c.177 2.383 1.453 2.74 2.837.79Zm-6.052-.083L9.872 68.04c-.176-2.382-.946-2.48-1.711-.216L.615 90.158c-.765 2.264.072 5.412 1.86 7l6.973 6.178c1.788 1.587 3.107.934 2.93-1.45Zm.993 13.337c-.176-2.382-1.782-5.63-3.57-7.216L5.71 104.38c-1.787-1.585-2.383-1.13-1.323 1.012l7.775 15.71c1.06 2.143 1.785 1.945 1.609-.438l-.4-5.44Z"
						/>
					</motion.svg>
					<motion.h1
						initial={{ opacity: 0, y: 40 }}
						animate={{ opacity: 1, y: 0 }}
						className={`text-center text-4xl md:text-7xl ${rocaTwo.className}`}
					>
						Welcome to{' '}
						<span className={`font-black text-5xl md:text-8xl ${darkerGrotesque.className}`}>Dyson</span>
					</motion.h1>
					<motion.p
						initial={{ opacity: 0, y: 40 }}
						animate={{ opacity: 1, y: 0 }}
						className="mt-3 text-xl text-center text-neutral-500 max-w-md mx-auto"
					>
						<span className="text-transparent bg-clip-text bg-gradient-to-bl from-purple-600 to-purple-400 font-medium">
							Your hardware, your rules.
						</span>{' '}
						Turn your server into the ultimate ML cloud, with zero setup.
					</motion.p>
				</div>
				<motion.button
					initial={{ opacity: 0, scale: 0.8 }}
					animate={{ opacity: 1, scale: 1, transition: { delay: 0.2 } }}
					whileHover={{ scale: 1.02 }}
					whileTap={{ scale: 0.98 }}
					className="mt-10 bg-neutral-900 text-neutral-100 py-3 w-full max-w-xs rounded-md"
				>
					Get Started
				</motion.button>
			</div>
			<motion.div
				initial={{ opacity: 0 }}
				animate={{ opacity: 1 }}
				transition={{ duration: 0.7, delay: 0.3 }}
				className="max-w-md mx-auto flex flex-col items-center space-y-4 md:space-y-6"
			>
				<svg
					className="w-6 h-6 md:w-8 md:h-8 text-neutral-400/40"
					xmlns="http://www.w3.org/2000/svg"
					viewBox="0 0 256 256"
				>
					<path
						fill="currentColor"
						d="M221.69 199.77 160 96.92V40h8a8 8 0 0 0 0-16H88a8 8 0 0 0 0 16h8v56.92L34.31 199.77A16 16 0 0 0 48 224h160a16 16 0 0 0 13.72-24.23Zm-90.08-42.91c-15.91-8.05-31.05-12.32-45.22-12.81l24.47-40.8a7.93 7.93 0 0 0 1.14-4.11V40h32v59.14a7.93 7.93 0 0 0 1.14 4.11L183.36 167c-11.96 2.34-29.07 1.34-51.75-10.14Z"
					/>
				</svg>
				<p className="text-center text-neutral-400 text-xs md:text-sm">
					You&apos;re running Dyson αlpha, which is not recommended for production use. Please report any bugs{' '}
					<a className="hover:underline" href="https://github.com/m1guelpf/dyson/issues/new" target="_blank">
						on GitHub
					</a>
					.
				</p>
			</motion.div>
		</div>
	)
}

export default HomePage
