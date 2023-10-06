import { FileTransferStatus } from "@/store/context";
import { computeFileSize, isClient } from "@/utils";

import {
  ArrowDownCircleIcon,
  ArrowUpCircleIcon,
  CheckCircleIcon,
  ExclamationCircleIcon,
  PauseCircleIcon,
  PlayCircleIcon,
} from "@heroicons/react/24/outline";
import Image from "next/image";
import { getFileIcon } from "../thumbnail";

// the required data to render the file card component
// the data will be passed dynamically

type TFileType = {
  fileType: string;
  fileName: string;
  fileSize: number;
  status: FileTransferStatus;
  // status: 'error' | 'done' | 'pending' | 'completed' | 'downloading' | 'paused';
};

// the component
export function FileTransferCard({
  fileName,
  fileSize,
  fileType,
  status,
}: TFileType) {
  return (
    <>
      <div className="flex justify-between items-center my-8 flex-wrap bg-[#edeffb]  border-gray-900  p-3 rounded-lg shadow-md shadow-gray-300 cursor-pointer dark:shadow-none  dark:bg-dark-800 hover:shadow-sm hover:shadow-gray-400 transition-shadow ease-in-out">
        <FileIcon fileType={fileType} />
        <div className="flex flex-col text-ellipsis">
          <h5
            className="font-medium text-gray-500 truncate overflow-clip text-ellipsis"
            style={{ width: "180px" }}
          >
            {fileName}
          </h5>
          <div className="flex gap-3 mt[1.5px] text-gray-400  italic text-xs height={30} 
                width={30} ">
            <span>{computeFileSize(fileSize)}</span>{" "}
            <span>{` transfer ${status}`}</span>
          </div>
        </div>

        <div className="hidden lg:block">
          {status == FileTransferStatus.COMPLETED
            ? <CheckCircleIcon className="w-8 h-8 text-gray-400 " />
            : status == FileTransferStatus.DOWNLOADING
            ? <PauseCircleIcon className="w-8 h-8 text-gray-400 " />
            : status == FileTransferStatus.ERROR
            ? <ExclamationCircleIcon className="w-8 h-8 text-gray-400 " />
            : status == FileTransferStatus.PAUSED
            ? <PlayCircleIcon className="w-8 h-8 text-gray-400 " />
            : status == FileTransferStatus.PENDING
            ? <ArrowUpCircleIcon className="w-8 h-8 text-gray-400 " />
            : <ArrowDownCircleIcon className="w-8 h-8 text-gray-400 " />}
        </div>
      </div>
    </>
  );
}

function FileIcon({ fileType }: { fileType: string }) {
  const thumbnail = getFileIcon(fileType);
  return (
    <>
      <Image
        src={thumbnail}
        height={120}
        width={120}
        alt="file card icon"
        className="w-[40px] col-span-1"
      />
    </>
  );
}
