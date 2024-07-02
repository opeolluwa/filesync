// provide file and folder icon
import { StaticImageData } from "next/image";
import { getFileIcon } from "../Thumbnail/media-icon-maker";
import folderIcon from "@/assets/common/folder-icon.png";
import Image from "next/image";

export interface ThumbnailIconInterface {
  isFolder: boolean;
  fileFormat: string;
}

export default function ThumbnailIcon({
  isFolder,
  fileFormat,
}: ThumbnailIconInterface) {
  let thumbnail: StaticImageData;
  if (isFolder) {
    thumbnail = folderIcon;
  } else {
    thumbnail = getFileIcon(fileFormat).icon;
  }

  return (
    <Image
      src={thumbnail}
      height={144}
      width={144}
      alt="file card icon"
      className="w-[32px]  mr-4"
    />
  );
}
