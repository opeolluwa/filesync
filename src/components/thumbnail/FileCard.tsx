import { FileTransferStatus } from "@/store/context";
import { computeFileSize, isClient } from "@/utils";
import imageIcon from "@/assets/common/image.png";
import audioIcon from "@/assets/common/audio.png";
import presentationIcon from "@/assets/common/presentation.png";
import pdfIcon from "@/assets/common/pdf.png";
import videoIcon from "@/assets/common/video.png";
import csvIcon from "@/assets/common/csv.png";
import defaultIcon from "@/assets/common/default.png";
import archiveIcon from "@/assets/common/archived.png";
import documentIcon from "@/assets/common/document.png";
import textIcon from "@/assets/common/text.png";
import svgIcon from "@/assets/common/svg.png";

import {
  ArrowDownCircleIcon,
  ArrowUpCircleIcon,
  CheckCircleIcon,
  ExclamationCircleIcon,
  PauseCircleIcon,
  PlayCircleIcon,
} from "@heroicons/react/24/outline";
import Image from "next/image";

// the required data to render the file card component
// the data will be passed dynamically

type TFileType = {
  fileType: string;
  fileName: string;
  fileSize: number;
  status: FileTransferStatus;
  // status: 'error' | 'done' | 'pending' | 'completed' | 'downloading' | 'paused';
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
  return (
    <>
      <div className="flex justify-between items-center my-8 flex-wrap bg-[#edeffb]  border-gray-900  p-3 rounded-lg shadow-md shadow-gray-300 cursor-pointer dark:shadow-none hover:shadow-sm hover:shadow-gray-400 transition-shadow ease-in-out">
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
  function getFileIcon(fileExtension: string) {
    const imageExtensions = ["jpg", "jpeg", "png", "gif", "bmp"];
    const documentExtensions = [
      ".doc",
      ".docx",
      ".rtf",
      ".odt",
      ".ods",
      ".odp",
      ".odg",
    ];
    const svgExtensions = ["svg"];
    const textExtensions = ["txt"];
    const audioExtensions = ["mp3", "wav", "ogg", "flac"];
    const pdfExtensions = ["pdf"];
    const csvExtensions = ["csv"];
    const presentationExtensions = ["ppt", "pptx", "keynote"];
    const videoExtensions = ["mp4", "avi", "mkv", "mov"];
    const archiveExtensions = ["zip", "rar", "tar", "gz"];
    const extension = fileExtension.toLocaleLowerCase().trim();

    if (imageExtensions.includes(extension)) {
      return imageIcon;
    } else if (audioExtensions.includes(extension)) {
      return audioIcon;
    } else if (pdfExtensions.includes(extension)) {
      return pdfIcon;
    } else if (csvExtensions.includes(extension)) {
      return csvIcon;
    } else if (
      presentationExtensions.includes(extension)
    ) {
      return presentationIcon;
    } else if (videoExtensions.includes(extension)) {
      return videoIcon;
    } else if (archiveExtensions.includes(extension)) {
      return archiveIcon;
    } else if (documentExtensions.includes(extension)) {
      return documentIcon;
    } else if (textExtensions.includes(extension)) {
      return textIcon;
    } else if (svgExtensions.includes(extension)) {
      return svgIcon;
    } else {
      return defaultIcon;
    }
  }

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
