"use client";

import { computeFileSize } from "@/utils";
import Modal from "antd/es/modal/Modal";
import { useRouter } from "next/router";
import { useState } from "react";
import { File } from "../../../core/bindings/File";
import PreviewMedia, { FileType } from "./preview-media";
import { getFileIcon } from "./media-icon-maker";
import ThumbnailIcon from "./icon-renderer";

export default function FileCard({
  fileName,
  fileFormat,
  filePath,
  fileSize,
  isFolder,
  isHidden,
}: File) {
  const [openModal, setOpenModal] = useState(false);
  const fileMeta = getFileIcon(fileFormat);
  const router = useRouter();

  // if it is a folder open in folder renderer
  // otherwise open in file renderer
  let path:string;
  if (isFolder) {
    path = `/render?filePath=${filePath}&fileType=${fileFormat}&isFolder=${isFolder}`;
  } else {
    path = `/render?filePath=${filePath}&fileType=${fileFormat}&isFolder=${isFolder}`;
  }

  return (
    <>
      <Modal
        title={"Preview Media"}
        open={openModal}
        onOk={() => setOpenModal(false)}
        onCancel={() => setOpenModal(false)}
        centered
        okButtonProps={{ hidden: true }}
        cancelButtonProps={{ hidden: true }}
        width={600}
      >
        <>
          <div className="h-[600px] ">
            <PreviewMedia fileType={fileFormat} filePath={filePath}/>
          </div>
        </>
      </Modal>
      <div
        onClick={() => {
          isFolder ? router.push(path) : setOpenModal(true);
        }}
        className="flex w-full hover:shadow hover:rounded-lg rouned bg-[#f9fbfe] flex-wrap items-center gap-2  cursor-pointer px-4 py-2 last:mb-10 "
      >
        <ThumbnailIcon isFolder={isFolder} fileFormat={fileFormat} />
        <div className="flex flex-col justify-between mt-3">
          <h6 className=" dark:text-gray-500 small overflow-clip  w-[240px] lg:w-[400px]  truncate select-none">
            {fileName}
          </h6>

          <div
            className="flex gap-3 mt[1.5px] text-gray-600  text-xs height={30} // Desired size with correct aspect ratio
                width={30} "
          >
            <span className="select-none">
              {computeFileSize(fileSize as unknown as number)}
            </span>
          </div>
        </div>
      </div>
    </>
  );
}



export interface FileInterface extends File {}