"use client";

import { useRouter } from "next/router";
import { useEffect, useState } from "react";
import { useSearchParams } from "next/navigation";
import FileCard, { FileInterface } from "@/components/thumbnail";
import { AppData } from "@/types";
import { invoke } from "@tauri-apps/api";
import LoaderCircle from "@/components/loaders/LoaderCircle";
import QuickAccessLayout from "@/components/layout/PageLayout";
import { shareFile } from "@/utils";

export default function PreviewMediaPage() {
  const router = useRouter();
  const searchParams = useSearchParams();
  const filePath = searchParams.get("filePath");
  const isFolder = searchParams.get("isFolder");
  const fileType = searchParams.get("fileType");

  // if it is a folder, get the files nd list them
  if (isFolder) {
    // eslint-disable-next-line react-hooks/rules-of-hooks
    const [data, setData] = useState(null);
    // eslint-disable-next-line react-hooks/rules-of-hooks
    const [isLoading, setLoading] = useState(false);

    // get the data from the application core
    // eslint-disable-next-line react-hooks/rules-of-hooks
    useEffect(() => {
      setLoading(true);
      invoke("read_dir", { path: filePath?.trim() }).then((res) => {
        setData(res as any);
        setLoading(false);
      });
    }, []);

    // typecast the response into AppData type
     const fetchedFiles = data as unknown as AppData<Array<FileInterface>>;
     if (isLoading) {
       return (
         <>
           <LoaderCircle />
           <h2 className="font-xl font-bold mt-8">Loading...</h2>
           <p className="leading-5 text-gray-400">
             Please wait while we load your documents. This might take a while.
           </p>
         </>
       );
     }


     // render them 
      return (
        <QuickAccessLayout
          pageTitle={"Document"}
          includeSearchBar={true}
          searchBarText="search document"
        >
          <div>
            <div className="flex flex-wrap  flex-grow gap-4 justify-start mt-12">
              {fetchedFiles?.data.map((file, index) => (
                <FileCard
                  key={index}
                  fileName={file.fileName}
                  fileSize={file.fileSize}
                  fileFormat={file.fileFormat}
                  filePath={file.filePath}
                  action={() => shareFile(file.filePath)}
                  isHidden={file.isHidden}
                  isFolder={file.isFolder}
                />
              ))}
            </div>
          </div>
        </QuickAccessLayout>
      );
  }


  if (isFolder) {
    return;
  }
  return (
    <div>
      <h1> file path: {filePath} </h1>
      <h1>file type {fileType}</h1>
      <h1>is folder {isFolder}</h1>
      Lorem ipsum dolor sit amet consectetur adipisicing elit. Repellat iste
      sequi aliquam aliquid! Iusto doloribus veritatis facere aliquam mollitia
      consequuntur hic, nostrum, fugiat pariatur incidunt, modi voluptatum illo
      doloremque maxime?
    </div>
  );
}
