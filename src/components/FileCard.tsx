/**
 * 
 * render the file in the UI
 * the component contains png/svg to show illustrating the file type 
 * the file size 
 * and the file status 
 */
interface Props {
    fileType: string,
    fileName: string,
    fileSize: string,
    downloadStatus: boolean
}
import { ArrowDownCircleIcon } from '@heroicons/react/24/outline';
import Image from 'next/image';
export default function FileCard() {
    return (
        <div className="flex justify-between items-center my-4 flex-wrap bg-[#edeffb] p-3 rounded-lg shadow shadow-gray-300">
            {/**flex one of three -> holding the image */}
            <Image
                src="/images/mime/mp4.png" // Route of the image file
                height={144} // Desired size with correct aspect ratio
                width={144} // Desired size with correct aspect ratio
                alt="file card icon"
                className='w-[64px]'// automatic height calculation
            />
            {/**flex two of three - holding the file description or download progess */}
            <div className='flex flex-col'>
                <h5 className='font-semibold'>sample-file-name.mp4</h5>
                <div className='flex gap-3 font-light text-sm'>
                    <span>145.5mb</span> <span>waiting for download</span>
                </div>
            </div>
            {/**flex three of three holding the file  download progress of waiting icon */}
            <ArrowDownCircleIcon className='w-8 h-8 text-sf_green-500' />
        </div>
    )
}
