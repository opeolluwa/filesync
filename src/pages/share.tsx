import PageLayout from "@/components/PageLayout";
import { SetStateAction, useRef, useState } from "react";

/**
 * @function helpPage -  A page responsible for guiding users on various actions
 * @returns tsx
 */
export default function ShareFiles() {
  const ref = useRef<HTMLInputElement>(null);
  // 1. add state for tracking the selected files
  const [selectedFiles, setSelectedFiles] = useState<File[]>([]);

  // 2. pass the click event to the hidden input element to trigger the file selection.
  const handleClick = () => {
    ref.current?.click();
  };

  // 3. convert FileList to File[]
  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const files = Array.from(e.currentTarget.files ?? []);
    setSelectedFiles(files);
  };
  return (
    <>
      <PageLayout pageTitle={"Shared files"} includeSearchBar={false}>
        <div className=" p-8" >
          <label className="flex justify-center w-full h-[500px] px-4 transition bg-transparent border-[2px] border-gray-300 border-dashed rounded-md appearance-none cursor-pointer hover:border-gray-400 focus:outline-none">
            <span className="flex items-center space-x-2">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                className="w-6 h-6 text-gray-600"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
                stroke-width="2"
              >
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12"
                />
              </svg>
              <span className="font-medium text-gray-600">
                Drop files to share, or
                <span className="text-app-600 underline"> browse</span>
              </span>
            </span>
            {/* <form onSubmit={handleUpload} encType="multipart"> */}
            <div onClick={handleClick}>
              <input
                type="file"
                name="file_upload"
                className="hidden"
                multiple={true}
                onChange={handleChange}
              />
            </div>
            {/* </form> */}
          </label>
        </div>

        {/* 6. display selected files */}
        {!!selectedFiles.length && (
          <div className="p-4 mt-4 bg-violet-50 overflow-hidden text-ellipsis">
            <p>Selected Files:</p>
            {selectedFiles.map((file, i) => {
              return (
                <span key={i} className="text-violet-500 whitespace-nowrap">
                  {file.name}
                </span>
              );
            })}
          </div>
        )}
      </PageLayout>
    </>
  );
}
