import { useContext } from "react";
import FileCard, { FileInterface } from "./file/FileCard";
import Image from "next/image";
import { FileContext } from "@/store/context";

export default function Aside() {
  const { fileList } = useContext(FileContext);

  console.log(fileList);
  return (
    <aside className="hidden lg:block lg:flex-col items-center lg:col-span-3 pt-10 px-8  h-full bg-[rgba(226,233,252,255)]">
      {(fileList.length && (
        <h2 className="font-bold  dark:text-gray-400 flex items-center justify-between mb-10">
          Sent Files
        </h2>
      )) ||
        ""}
      {/**
       * use state management to display files here
       * a procedure to determine the file type and the right file icon should be added
       */}
      {fileList.length == 0 ? (
        <div className="flex flex-col items-center gap-4 mt-10">
          <Image
            src={"/icons/empty-state.svg"} // Route of the image file
            height={100} // Desired size with correct aspect ratio
            width={100} // Desired size with correct aspect ratio
            alt="file card icon"
            className="" // automatic height calculation
          />
          <h3 className="text-gray-400 mt-1 italic">no recent files </h3>
        </div>
      ) : (
        fileList.map((file, index) => (
          <FileCard
            key={index}
            fileName={file.name}
            fileType={file.name.split(".")[1]}
            fileSize={file.size}
            status={file.status}
          />
        ))
      )}
    </aside>
  );
}
