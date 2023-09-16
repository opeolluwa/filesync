import DocumentComponent from "@/components/thumbnail/DocumentComponent";
import QuickAccessLayout from "@/components/layout/PageLayout";
import { AppData, AudioFile } from "@/types";
import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from "react";
import { shareFile } from "@/utils";
import LoaderCircle from "@/components/loaders/LoaderCircle";

const isClient = typeof window !== "undefined";

export default function Document() {
  const [data, setData] = useState(null);
  const [isLoading, setLoading] = useState(false);

  // get the data from the application core
  useEffect(() => {
    setLoading(true);
    invoke("fetch_documents").then((res) => {
      setData(res as any);
      console.log({ res });

      setLoading(false);
    });
    // setLoading(false);
  }, []);

  // typecast the response into AppData type
  const fetchedDocuments = data as unknown as AppData<Array<AudioFile>>;
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
      pageTitle={"Music"}
      includeSearchBar={true}
      searchBarText="search document"
    >
      <div>
        <div className="flex flex-wrap  flex-grow gap-4 justify-start mt-12">
          {fetchedDocuments?.data.map((file, index) => (
            <DocumentComponent
              key={index}
              fileName={file.fileName}
              fileSize={file.fileSize}
              fileFormat={file.fileFormat}
              filePath={file.filePath}
              onClick={() => shareFile(file.filePath)}
            />
          ))}
        </div>
      </div>
    </QuickAccessLayout>
  );
}
