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
import Image from "next/image";
import { File } from "../../../core/bindings/File";

// to interface with audio files coming from the application core
// the type extends the AppData type
export interface FileInterface extends File {
}

// the required data to render the file card component
export interface FileCardInterface extends FileInterface {
  action?: () => void; // the action to perform when the file is clicked, for example it can be used to play an audio file
}

// the required data to render the file card component
// the data will be passed dynamically

type TFileType = {
  fileType: string;
  fileName: string;
  fileSize: number;
  status: FileTransferStatus;
  // status: 'error' | 'done' | 'pending' | 'completed' | 'downloading' | 'paused';
};
export interface FileTransferInterface {
  fileType: string;
  fileName: string;
  fileSize: number;
  status: FileTransferStatus;
}

interface Props extends FileCardInterface {
}

export default function FileCard(
  { fileName, fileFormat, filePath, fileSize, action }: Props,
) {
  const thumbnail = getFileIcon(fileFormat);
  return (
    <div
      onClick={action}
      className="flex w-full  flex-wrap items-center gap-2  cursor-pointer px-4 py-2 last:mb-10 dark:bg-dark-800"
    >
      <div>
        {
          <Image
            src={thumbnail} // Route of the image file
            height={144} // Desired size with correct aspect ratio
            width={144} // Desired size with correct aspect ratio
            alt="file card icon"
            className="w-[32px]  mr-10 " // automatic height calculation
          />
        }
      </div>
      <div className="flex flex-col justify-between mt-3">
        <h6 className=" dark:text-gray-500 small overflow-clip  w-[240px] lg:w-[400px]  truncate">
          {fileName}
        </h6>
        <div className="flex  gap-3 mt[1.5px] text-gray-600  text-xs height={30} // Desired size with correct aspect ratio
                width={30} ">
          <span>{computeFileSize(fileSize as unknown as number)}</span>{" "}
          <span>
            {/**file duration goes here */}
          </span>
        </div>
      </div>
    </div>
  );
}

export function getFileIcon(fileExtension: string) {
  const imageExtensions = [
    "jpg",
    "jpeg",
    "png",
    "gif",
    "bmp",
    "tiff",
    "raw",
    "svg",
    "ai",
    "eps",
    "psd",
    "xcf",
    "ico",
    "webp",
    "jxr",
    "hdr",
    "tif",
    "exif",
    "pgm",
    "ppm",
    "pbm",
    "pnm",
    "heic",
    "heif",
    "dng",
    "cr2",
    "nef",
    "arw",
    "orf",
    "rw2",
    "sr2",
    "raf",
    "mrw",
    "pef",
    "x3f",
    "3fr",
    "kdc",
    "srw",
    "nrw",
    "rwz",
    "rwl",
    "iiq",
    "rw1",
    "r3d",
    "fff",
    "yuv",
    "cin",
    "dpx",
    "jp2",
    "j2k",
    "jpf",
    "jpx",
    "jpm",
    "mj2",
    "wdp",
    "hdp",
    "dds",
    "pvr",
    "tga",
    "cur",
    "icl",
    "thm",
    "sai",
    "ora",
    "pdn",
    "kra",
    "cpt",
    "pdd",
    "mng",
    "apng",
    "svgz",
    "emf",
    "wmf",
  ];
  const documentExtensions = [
    "doc",
    "docx",
    "rtf",
    "odt",
    "ods",
    "odp",
    "odg",
    "odp",
    "fodp",
    "otp",
    "doc",
    "dot",
    "docx",
    "docm",
    "dotx",
    "dotm",
    "docb",
    "odt",
    "fodt",
  ];
  const svgExtensions = ["svg"];
  const textExtensions = ["txt"];
  const audioExtensions = [
    "3gp",
    "aa",
    "aac",
    "aax",
    "act",
    "aiff",
    "alac",
    "amr",
    "ape",
    "au",
    "awb",
    "dss",
    "dvf",
    "flac",
    "gsm",
    "iklax",
    "ivs",
    "m4a",
    "m4b",
    "m4p",
    "mmf",
    "movpkg",
    "mp3",
    "mpc",
    "msv",
    "nmf",
    "ogg",
    "oga",
    "mogg",
    "opus",
    "ra",
    "rm",
    "raw",
    "rf64",
    "sln",
    "tta",
    "voc",
    "vox",
    "wav",
    "wma",
    "wv",
    "webm",
    "8svx",
    "cda",
  ];
  const pdfExtensions = ["pdf"];
  const csvExtensions = ["csv"];
  const presentationExtensions = [
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
    "thmx",
  ];
  const videoExtensions = [
    "mp4",
    "mkv",
    "webm",
    "flv",
    "vob",
    "ogv",
    "ogg",
    "drc",
    "gif",
    "gifv",
    "mng",
    "avi",
    "MTS",
    "MT2S",
    "TS",
    "mov",
    "qt",
    "wmv",
    "yuv",
    "rm",
    "rmvb",
    "viv",
    "asf",
    "amv",
    "m4p",
    "m4v",
    "mpg",
    "mp2",
    "mpeg",
    "mpe",
    "mpv",
    "m2v",
    "svi",
    "3gp",
    "3g2",
    "mxf",
    "roq",
    "nsv",
    "f4v",
    "f4p",
    "f4a",
    "f4b",
  ];
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
