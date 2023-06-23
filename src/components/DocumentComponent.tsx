import Image from 'next/image';
import { AudioFile } from '@/types';
import { computeFileSize } from '@/utils';
import { shareMusicFile } from '@/pages/quick-access/music';

// type Props = AudioFile;
interface Props extends AudioFile {
    onClick: (argz: string) => void
}

export default function MusicFile({ fileName, fileFormat, fileSize, filePath }: Props) {
    const __file_icon__ = `/mime/document/${fileFormat || 'doc'}.png`;
    const __fileSize__ = fileSize;

    return (
        <div onClick={() => shareMusicFile(filePath)} className='flex w-full  flex-wrap items-center gap-2 border-b border-b-gray-100   cursor-pointer px-4 py-2 last:mb-10' >
            <div >
                {
                    <Image
                        src={__file_icon__} // Route of the image file
                        height={144} // Desired size with correct aspect ratio
                        width={144} // Desired size with correct aspect ratio
                        alt="file card icon"
                        className='w-[32px]  mr-10 '// automatic height calculation
                    />
                }
            </div>
            <div className='flex flex-col justify-between mt-3'>
                <h6 className=' dark:text-gray-500 small overflow-clip  w-[240px] lg:w-[400px]  truncate'>{fileName}</h6>
                <div className='flex  gap-3 mt[1.5px] text-gray-600  text-xs height={30} // Desired size with correct aspect ratio
                width={30} '>
                    <span>{__fileSize__}</span> <span>
                        {/**file duration goes here */}
                    </span>
                </div>
            </div>
        </div>
    )
}
