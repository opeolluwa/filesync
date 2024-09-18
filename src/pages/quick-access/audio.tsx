import FileCard, { FileInterface } from "@/components/Thumbnail";
import QuickAccessLayout from "@/components/layout/DesktopViewLayout";
import { AppData } from "@/types";
import { convertFileSrc, invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import LoaderCircle from "@/components/loaders/LoaderCircle";

export default function Music() {
  const [data, setData] = useState(null);
  const [isLoading, setLoading] = useState(false);

  // get the data from the application core
  useEffect(() => {
    setLoading(true);
    invoke("read_dir", { path: "audio" }).then((res) => {
      setData(res as any);
      setLoading(false);
    });
  }, []);
  // typecast the response into AppData type
  const musicData = data as unknown as AppData<Array<FileInterface>>;
  if (isLoading) {
    return (
      <>
        <LoaderCircle />
        <h2 className="font-xl font-bold mt-2">Loading...</h2>
        <p className="leading-5 text-gray-400">
          Please wait while we load your music files. This might take a while
        </p>
        <LoaderCircle />
      </>
    );
  }
  return (
    <QuickAccessLayout
      pageTitle={"Music"}
      includeSearchBar={true}
      searchBarText="search audio files"
    >
      <div>
        <div className="flex flex-wrap  flex-grow gap-4 justify-start mt-12">
          {musicData?.data.map((file, index) => (
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
