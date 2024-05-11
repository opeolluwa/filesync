"use client"

import FileCard, { FileInterface } from "@/components/thumbnail";
import QuickAccessLayout from "@/components/layout/PageLayout";
import { AppData} from "@/types";
import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from "react";
import LoaderCircle from "@/components/loaders/LoaderCircle";


export default function Document() {
  const [data, setData] = useState(null);
  const [isLoading, setLoading] = useState(false);

  // get the data from the application core
  useEffect(() => {
    setLoading(true);
    invoke("read_dir", { path: "deownloads" }).then((res) => {
      console.log(JSON.stringify(res));
      setData(res as any);
      setLoading(false);
    });
  }, []);

  // typecast the response into AppData type
  const fetchedDocuments = data as unknown as AppData<Array<FileInterface>>;
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

  return (
    <QuickAccessLayout
      pageTitle={"Downloads"}
      includeSearchBar={true}
      searchBarText="search downloads"
    >
      <div>
        <div className="flex flex-wrap  flex-grow gap-4 justify-start mt-12">
          {fetchedDocuments?.data.map((file, index) => (
            <FileCard
              key={index}
              fileName={file.fileName}
              fileSize={file.fileSize}
              fileFormat={file.fileFormat}
              filePath={file.filePath}
              isHidden={file.isHidden}
              isFolder={file.isFolder}
            />
          ))}
        </div>
      </div>
    </QuickAccessLayout>
  );
}
