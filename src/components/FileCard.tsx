

/**
 * 
 * render the file in the UI
 * the component contains png/svg to show illustrating the file type 
 * the file size 
 * and the file status 
 */

import { computeFileSize } from '@/utils';
import { ArrowDownCircleIcon, PauseCircleIcon } from '@heroicons/react/24/outline';
import Image from 'next/image';


// the reqired data to render the file card component
// the data will be passed dynamically
export interface FileInterface {
    fileType: string,
    fileName: string,
    fileSize: number,
    downloadInProgress: boolean
}



// the component
export default function FileCard({ fileName, fileSize, fileType, downloadInProgress }: FileInterface) {
    const __filetype__ = `/images/mime/${fileType}.png`;
    const __fileSize__ = computeFileSize(fileSize);
    const __downloadStatus__ = downloadInProgress == true ? "downloading ..." : "Waiting for download";

    return (
        <>
            <div className="flex justify-between items-center my-6 flex-wrap bg-[#edeffb] dark:bg-mirage-600 dark:border border-gray-900  p-3 rounded-lg shadow-sm shadow-gray-300 cursor-pointer dark:shadow-none hover:shadow-sm hover:shadow-gray-400 dark:hover:border-gray-700 dark:hover:shadow-none transition-shadow ease-in-out">
                {/**flex one of three -> holding the image */}
                <Image
                    src={__filetype__} // Route of the image file
                    height={144} // Desired size with correct aspect ratio
                    width={144} // Desired size with correct aspect ratio
                    alt="file card icon"
                    className='w-[32px] dark:brightness-75'// automatic height calculation
                />
                {/**flex two of three - holding the file description or download progess */}
                <div className='flex flex-col'>
                    <h5 className='font-semibold dark:text-gray-500'>{fileName}</h5>
                    <div className='flex gap-3 mt[1.5px] text-gray-600  text-xs height={30} // Desired size with correct aspect ratio
                width={30} '>
                        <span>{__fileSize__}</span> <span>{__downloadStatus__}</span>
                    </div>
                </div>
                {/**flex three of three holding the file  download progress of waiting icon */}
                {
                    downloadInProgress == true ?
                        <PauseCircleIcon className='w-8 h-8 text-sf_green-500 dark:text-shilo-900' /> :
                        <ArrowDownCircleIcon className='w-8 h-8 text-sf_green-500 dark:text-sf_green-900' />
                }
            </div>
        </>
    )
}


// export Props as FileInterface