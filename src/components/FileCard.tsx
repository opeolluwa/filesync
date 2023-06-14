/**
 *
 * render the file in the UI
 * the component contains png/svg to show illustrating the file type
 * the file size
 * and the file status
 */

import { computeFileSize, isClient } from "@/utils";
import {
  ArrowDownCircleIcon,
  ArrowUpCircleIcon,
  CheckCircleIcon,
  PauseCircleIcon,
  PlayCircleIcon,
} from "@heroicons/react/24/outline";
import Image from "next/image";
//import 'file-icons-js/css/style.css'
export enum FileTransferStatus {
  DOWNLOADING = "downloading",
  PAUSED = "paused",
  PENDING = "pending",
  COMPLETED = "completed",
}
// the required data to render the file card component
// the data will be passed dynamically
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
}: FileInterface) {
  return (
    <>
      <div className="flex justify-between items-center my-8 flex-wrap bg-[#edeffb]  border-gray-900  p-3 rounded-lg shadow-md shadow-gray-300 cursor-pointer dark:shadow-none hover:shadow-sm hover:shadow-gray-400 transition-shadow ease-in-out">
        <FileIcon fileType={fileType} />
        <div className="flex flex-col text-ellipsis">
          <h5 className="font-medium text-gray-500 overflow-clip text-ellipsis">
            {fileName}
          </h5>
          <div
            className="flex gap-3 mt[1.5px] text-gray-400  italic text-xs height={30} 
                width={30} "
          >
            <span>{computeFileSize(fileSize)}</span>{" "}
            <span>{` transfer ${status}`}</span>
          </div>
        </div>

        <div className="hidden lg:block">
          {status == FileTransferStatus.COMPLETED ? (
            <CheckCircleIcon className="w-8 h-8 text-gray-400 " />
          ) : status == FileTransferStatus.DOWNLOADING ? (
            <PauseCircleIcon className="w-8 h-8 text-gray-400 " />
          ) : status == FileTransferStatus.PAUSED ? (
            <PlayCircleIcon className="w-8 h-8 text-gray-400 " />
          ) : status == FileTransferStatus.PENDING ? (
            <ArrowUpCircleIcon className="w-8 h-8 text-gray-400 " />
          ) : (
            <ArrowDownCircleIcon className="w-8 h-8 text-gray-400 " />
          )}
        </div>
      </div>
    </>
  );
}

function FileIcon({ fileType }: { fileType: string }) {
  const fileExtension = fileType.trim().toLowerCase();
  const images = [
    "ai",
    "dxf",
    "odg",
    "fodg",
    "svg",
    "svgz",
    "bmp",
    "gif",
    "ico",
    "jpg",
    "jpeg",
    "png",
    "psd",
    "pdd",
    "tga",
    "tiff",
    "xcf",
    "xpm",
  ];
  const audio = [
    "au",
    "aif",
    "aifc",
    "aiff",
    "wav",
    "flac",
    "la",
    "pac",
    "m4a",
    "ape",
    "wv",
    "wma",
    "ast",
    "mp2",
    "mp3",
    "spx",
    "aac",
    "mpc",
    "ra",
    "ogg",
    "mid",
    "m3u",
    "pls",
  ];
  const powerpoint = [
    "ppt",
    "pot",
    "pps",
    "pptx",
    "pptm",
    "potx",
    "potm",
    "ppam",
    "ppsx",
    "ppsm",
    "sldx",
    "sldm",
    "odp",
    "fodp",
    "otp",
  ];
  const video = [
    "webm",
    "mkv",
    "flv",
    "vob",
    "ogv",
    "drc",
    "avi",
    "mov",
    "qt",
    "wmv",
    "rm",
    "rmvb",
    "asf",
    "mp4",
    "m4p",
    "m4v",
    "mpg",
    "mpeg",
    "mpe",
    "mpv",
    "3gp",
    "3g2",
    "mxf",
    "aff",
    "m2ts",
    "mts",
  ];
  const excel = [
    "xls",
    "xlt",
    "xlm",
    "xlsx",
    "xlsm",
    "xltx",
    "xltm",
    "xla",
    "xlam",
    "ods",
    "fods",
    "ots",
  ];
  return (
    <>
      {images.includes(fileExtension) ? (
        <Image
          src={`/images/mime/${fileExtension}.png`}
          height={120}
          width={120}
          alt="file card icon"
          className="w-[48px] col-span-1"
        />
      ) : audio.includes(fileExtension) ? (
        <Image
          src={`/images/mime/${fileExtension}.png`}
          height={120}
          width={120}
          alt="file card icon"
          className="w-[48px] col-span-1"
        />
      ) : video.includes(fileExtension) ? (
        <Image
          src={`/images/mime/${fileExtension}.png`}
          height={120}
          width={120}
          alt="file card icon"
          className="w-[48px] col-span-1"
        />
      ) : excel.includes(fileExtension) ? (
        <Image
          src={`/images/mime/${fileExtension}.png`}
          height={120}
          width={120}
          alt="file card icon"
          className="w-[48px] col-span-1"
        />
      ) : powerpoint.includes(fileExtension) ? (
        <Image
          src={`/images/mime/${fileExtension}.png`}
          height={120}
          width={120}
          alt="file card icon"
          className="w-[48px] col-span-1"
        />
      ) : (
        <Image
          src={`/images/mime/file.png`}
          height={120}
          width={120}
          alt="file card icon"
          className="w-[48px] col-span-1"
        />
      )}
    </>
  );
}
