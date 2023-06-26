import { goToPrevPage, goToNextPage } from "@/utils"
import { ChevronLeftIcon, ChevronRightIcon } from "@heroicons/react/24/outline"

interface Props {
    children: React.ReactNode,
    pageTitle: string
}



export default function QuickAccessLayout({ children, pageTitle }: Props) {
    return (
        <>
            <h2 className="flex mb-10 font-medium dark:text-gray-300 justify-between items-center">
                <ChevronLeftIcon className="text-gary-300 dark:text-gray-200 dark:hover:text-shilo-700 w-6 h-6 text-medium cursor-pointer mr-8" onClick={goToPrevPage} />
                {pageTitle}
                <ChevronRightIcon className="text-gary-300 dark:text-gray-200 dark:hover:text-shilo-700 w-6 h-6 text-medium cursor-pointer mr-8" onClick={goToNextPage} />
            </h2>
            <div className="dark:text-gray-400 mt-10">
                {/**inject child element  */}
                {children}
            </div>
        </>

    )
}
