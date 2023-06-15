import PageLayout from "@/components/PageLayout";
import { SetStateAction, useState } from "react";

/**
 * @function helpPage -  A page responsible for guiding users on various actions
 * @returns tsx
 */
export default function ShareFiles() {
  const [file, setFile] = useState(null);
  const handleChange = (file: SetStateAction<null>) => {
    setFile(file);
  };

  return (
    <>
      <PageLayout pageTitle={"Shared files"} includeSearchBar={false}>
        <div className=" p-8">
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
            <form action="" encType="multipart">
              <input type="file" name="file_upload" className="hidden" />
            </form>
          </label>
        </div>
      </PageLayout>
    </>
  );
}
