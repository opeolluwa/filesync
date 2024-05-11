// provide file and folder icon
import { getFileIcon } from "./media-icon-maker";
import folderIcon from "@/assets/common/folder-icon.png";
import { IonImg } from "@ionic/react";

export interface ThumbnailIconInterface {
  fileFormat: string;
}

export default function ThumbnailIcon({ fileFormat }: ThumbnailIconInterface) {
  const thumbnail = getFileIcon(fileFormat).icon;
  return (
    <img
      src={thumbnail}
      height={144}
      width={144}
      alt="file card icon"
      className="w-[32px] h-[32px]  mr-4"
    />
  );
}
