

/**
 * 
 * render the file in the UI
 * the component contains png/svg to show illustrating the file type 
 * the file size 
 * and the file status 
 */

import { computeFileSize } from '@/utils';
import { ArrowDownCircleIcon, ArrowUpCircleIcon, CheckCircleIcon, CheckIcon, PauseCircleIcon, PlayCircleIcon } from '@heroicons/react/24/outline';
import Image from 'next/image';


export enum FileTransferStatus {
    DOWNLOADING = "downloading",
    PAUSED = "paused",
    WAITING = "waiting",
    COMPLETED = "completed"

}
// the required data to render the file card component
// the data will be passed dynamically
export interface FileInterface {
    fileType: string,
    fileName: string,
    fileSize: number,
    status: FileTransferStatus
}



// the component
export default function FileCard({ fileName, fileSize, fileType, status}: FileInterface) {
 
    return (
        <>
            <div className="flex justify-between items-center my-8 flex-wrap bg-[#edeffb]  border-gray-900  p-3 rounded-lg shadow-sm shadow-gray-300 cursor-pointer dark:shadow-none hover:shadow-sm hover:shadow-gray-400 dark:hover:border-gray-700 dark:hover:shadow-none transition-shadow ease-in-out">
                {/**flex one of three -> holding the image */}
                <Image
                    src={`/images/mime/${fileType}.png`} // Route of the image file
                    height={144} // Desired size with correct aspect ratio
                    width={144} // Desired size with correct aspect ratio
                    alt="file card icon"
                    className='w-[48px] col-span-1 '// automatic height calculation
                />
                {/**flex two of three - holding the file description or download progress */}
                <div className='flex flex-col text-ellipsis'>
                    <h5 className='font-semibold text-gray-500 overflow-clip text-ellipsis'>{fileName}</h5>
                    <div className='flex gap-3 mt[1.5px] text-gsray-400  italic text-xs height={30} // Desired size with correct aspect ratio
                width={30} '>
                        <span>{computeFileSize(fileSize)}</span> <span>{` transfer ${status}`}</span>
                    </div>
                </div>
                {/**flex three of three holding the file  download progress of waiting icon */}
                <div className="hidden lg:block">
                    {
                        status == FileTransferStatus.COMPLETED ?
                            <CheckCircleIcon className='w-8 h-8 text-sf_green-500 ' /> :
                            status == FileTransferStatus.DOWNLOADING ?
                                <PauseCircleIcon className='w-8 h-8 text-sf_green-500 ' /> :
                                status == FileTransferStatus.PAUSED ?
                                    <PlayCircleIcon className='w-8 h-8 text-sf_green-500 ' /> :
                                    status == FileTransferStatus.WAITING ?
                                        < ArrowUpCircleIcon className='w-8 h-8 text-sf_green-500 ' /> :
                                        <ArrowDownCircleIcon className='w-8 h-8 text-sf_green-500 ' />

                    }
                </div>
               
            </div>
        </>
    )
}


// export Props as FileInterface