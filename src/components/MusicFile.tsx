import Image from 'next/image';
import { AudioFile } from '@/types';
import { computeFileSize } from '@/utils';

// allowed audio  types 
// display type not listed here as broken file
// type AudioFile = 'au' | 'aif' | 'aifc' | 'aiff' | 'wav' | 'flac' | 'la' | 'pac' | 'm4a' | 'ape' | 'wv' | 'wma' | 'ast' | 'mp2' | 'mp3' | 'spx' | 'aac' | 'mpc' | 'ra' | 'ogg' | 'mid' | 'm3u' | 'pls';


type Props = AudioFile;

export default function MusicFile({ fileName, fileFormat, fileSize, filePath }: Props) {
    const __file_icon__ = "/images/mime/music-player.png";
    const __fileSize__ = computeFileSize(fileSize);

    return (
        <div className='flex w-full  flex-wrap items-center gap-2 border-b border-b-shilo-100 dark:border-b-mirage-600 hover:dark:border-b-mirage-400 hover:dark:brightness-75 cursor-pointer px-4 py-2' >
            <div>
                {
                    <Image
                        src={__file_icon__} // Route of the image file
                        height={144} // Desired size with correct aspect ratio
                        width={144} // Desired size with correct aspect ratio
                        alt="file card icon"
                        className='w-[32px] dark:brightness-75 mr-10 '// automatic height calculation
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
