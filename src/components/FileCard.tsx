/**
 *
 * render the file in the UI
 * the component contains png/svg to show illustrating the file type
 * the file size
 * and the file status
 */

import { computeFileSize, isClient } from '@/utils';
import {
	ArrowDownCircleIcon,
	ArrowUpCircleIcon,
	CheckCircleIcon,
	PauseCircleIcon,
	PlayCircleIcon,
} from '@heroicons/react/24/outline';
import Image from 'next/image';
export enum FileTransferStatus {
	DOWNLOADING = 'downloading',
	PAUSED = 'paused',
	PENDING = 'pending',
	COMPLETED = 'completed',
}
// the required data to render the file card component
// the data will be passed dynamically

type TFileType = {
	fileType: string;
	fileName: string;
	fileSize: number;
	status: 'error' | 'done' | 'pending' | 'completed' | 'downloading' | 'paused';
};
export interface FileInterface {
	fileType: string;
	fileName: string;
	fileSize: number;
	status: FileTransferStatus;
}

// the component
export default function FileCard({
	fileName,
	fileSize,
	fileType,
	status,
}: TFileType) {
	console.log(fileType);
	return (
		<>
			<div className='flex justify-between items-center my-8 flex-wrap bg-[#edeffb]  border-gray-900  p-3 rounded-lg shadow-md shadow-gray-300 cursor-pointer dark:shadow-none hover:shadow-sm hover:shadow-gray-400 transition-shadow ease-in-out'>
				<FileIcon fileType={fileType} />
				<div className='flex flex-col text-ellipsis'>
					<h5 className='font-medium text-gray-500 truncate overflow-clip text-ellipsis'>
						{fileName}
					</h5>
					<div
						className='flex gap-3 mt[1.5px] text-gray-400  italic text-xs height={30} 
                width={30} '
					>
						<span>{computeFileSize(fileSize)}</span>{' '}
						<span>{` transfer ${status}`}</span>
					</div>
				</div>

				<div className='hidden lg:block'>
					{status == FileTransferStatus.COMPLETED ? (
						<CheckCircleIcon className='w-8 h-8 text-gray-400 ' />
					) : status == FileTransferStatus.DOWNLOADING ? (
						<PauseCircleIcon className='w-8 h-8 text-gray-400 ' />
					) : status == FileTransferStatus.PAUSED ? (
						<PlayCircleIcon className='w-8 h-8 text-gray-400 ' />
					) : status == FileTransferStatus.PENDING ? (
						<ArrowUpCircleIcon className='w-8 h-8 text-gray-400 ' />
					) : (
						<ArrowDownCircleIcon className='w-8 h-8 text-gray-400 ' />
					)}
				</div>
			</div>
		</>
	);
}

function FileIcon({ fileType }: { fileType: string }) {
	const fileExtension = fileType.trim().toLowerCase();
	const images = ['ai', 'jpg', 'jpeg', 'psd', 'svg'];
	const audio = ['avi', 'mp3'];
	const video = ['avi', 'mkv', 'mp4'];
	const document = [
		'csv',
		'doc',
		'docx',
		'odt',
		'ppt',
		'pptx',
		'rtf',
		'txt',
		'xls',
		'xml',
		'zip',
		'xlsb',
		'xlsx',
	];

	return (
		<>
			{images.includes(fileExtension) ? (
				<Image
					src={`/mime/images${fileExtension}.png`}
					height={120}
					width={120}
					alt='file card icon'
					className='w-[48px] col-span-1'
				/>
			) : audio.includes(fileExtension) ? (
				<Image
					src={`/mime/audio/${fileExtension}.png`}
					height={120}
					width={120}
					alt='file card icon'
					className='w-[48px] col-span-1'
				/>
			) : video.includes(fileExtension) ? (
				<Image
					src={`/mime/video/${fileExtension}.png`}
					height={120}
					width={120}
					alt='file card icon'
					className='w-[48px] col-span-1'
				/>
			) : document.includes(fileExtension) ? (
				<Image
					src={`/mime/document/${fileExtension}.png`}
					height={120}
					width={120}
					alt='file card icon'
					className='w-[48px] col-span-1'
				/>
			) : (
				<Image
					src={`/mime/extras/file.png`}
					height={120}
					width={120}
					alt='file card icon'
					className='w-[48px] col-span-1'
				/>
			)}
		</>
	);
}
