

/**
 * 
 * render the file in the UI
 * the component contains png/svg to show illustrating the file type 
 * the file size 
 * and the file status 
 */

import { ArrowDownCircleIcon, PauseCircleIcon } from '@heroicons/react/24/outline';
import Image from 'next/image';

/**
 * 
 * @function computeFileSize - compute file size to human readable format
 * @param size - file size in byte
 * @returns file size and extension e.g 3.5 MB
 */

function computeFileSize(size: number) {
    if (size > 1024 * 1024 * 1024) {
        return (size / (1024 * 1024 * 1024)).toFixed(1).toString() + " TB";
    } else if (size > 1024 * 1024) {
        return (size / (1024 * 1024)).toFixed(1).toString() + " GB";
    } else if (size > 1024) {
        return (size / 1024).toFixed(1).toString() + " MB";
    } else {
        return size.toString() + " KB";
    }
}

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
        <div className="flex justify-between items-center my-6 flex-wrap bg-[#edeffb] p-3 rounded-lg shadow-sm shadow-gray-300 cursor-pointer hover:shadow-sm hover:shadow-gray-400 transition-shadow ease-in-out">
            {/**flex one of three -> holding the image */}
            <Image
                src={__filetype__} // Route of the image file
                height={144} // Desired size with correct aspect ratio
                width={144} // Desired size with correct aspect ratio
                alt="file card icon"
                className='w-[64px]'// automatic height calculation
            />
            {/**flex two of three - holding the file description or download progess */}
            <div className='flex flex-col'>
                <h5 className='font-semibold'>{fileName}</h5>
                <div className='flex gap-3 mt[1.5px] text-gray-600  text-xs height={30} // Desired size with correct aspect ratio
                width={30} '>
                    <span>{__fileSize__}</span> <span>{__downloadStatus__}</span>
                </div>
            </div>
            {/**flex three of three holding the file  download progress of waiting icon */}
            {
                downloadInProgress == true ?
                    <PauseCircleIcon className='w-8 h-8 text-sf_green-500' /> :
                    <ArrowDownCircleIcon className='w-8 h-8 text-sf_green-500' />
            }
        </div>
    )
}


// export Props as FileInterface