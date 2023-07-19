import Image from "next/image";
import { AudioFile } from "@/types";
import { computeFileSize } from "@/utils";
import { convertFileSrc } from "@tauri-apps/api/tauri";

// type Props = AudioFile;
interface Props extends AudioFile {
  onClick: (argz: string) => void;
}

export default function MusicFile({
  fileName,
  fileFormat,
  fileSize,
  filePath,
}: Props) {
  const fileIcon = "/mime/extra/music-player.png";
  return (
    <div
      className="flex w-full  flex-wrap items-center gap-4 border-b border-b-gray-100   cursor-pointer px-4 py-2 last:mb-10"
    >
      <div className="hidden">
        <input
          id="default-checkbox"
          type="checkbox"
          value=""
          className="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"
        />
      </div>
      

      <div>
        {
          <Image
            src={fileIcon} // Route of the image file
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
        <div
          className="flex  gap-3 mt[1.5px] text-gray-600  text-xs height={30} // Desired size with correct aspect ratio
                width={30} "
        >
          <span>{fileSize}</span> <span>{/**file duration goes here */}</span>
        </div>
      </div>
    </div>
  );
}
