import FileCard from "./FileCard";

export default function () {
    return (
        <aside className='col-span-4 pt-10 px-8  bg-[rgba(226,233,252,255)] dark:bg-[#1a1b1b] dark:tet-gray-400' >
            <h2 className='font-medium dark:text-gray-400 flex items-center justify-between mb-10'>
                Download
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" className="w-6 h-6 font-bold">
                    <path strokeLinecap="round" strokeLinejoin="round" d="M6.75 12a.75.75 0 11-1.5 0 .75.75 0 011.5 0zM12.75 12a.75.75 0 11-1.5 0 .75.75 0 011.5 0zM18.75 12a.75.75 0 11-1.5 0 .75.75 0 011.5 0z" />
                </svg>
            </h2>
            {
                /**
                 * use state management to display files here
                 * a procedure to determine the file type too and the right file icon should be aded
                 */
            }
            <FileCard />
            <button>

            </button>
        </aside>
    )
}