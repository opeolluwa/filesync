import { File } from "./file";
import ThumbnailIcon from "./icon-renderer";

interface Props extends File {}

export default function Thumbnail({
  fileName,
  fileFormat,
  filePath,
  fileSize,
  isFolder,
  isHidden,
}: Props) {
  return (
    <div className="flex  items-center justify-start">
      <ThumbnailIcon fileFormat={fileFormat} />
      <div className="flex flex-col justify-between mt-3">
        <h6 className=" dark:text-gray-500 small overflow-clip  w-[240px] lg:w-[400px]  truncate select-none">
          {fileName}
        </h6>

        <div
          className="flex gap-3 mt[1.5px] text-gray-600  text-xs height={30} // Desired size with correct aspect ratio
                width={30} "
        >
          <span className="select-none">{fileSize}</span>
        </div>
      </div>
    </div>
  );
}
