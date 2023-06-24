import DocumentComponent from "@/components/file/DocumentComponent";
import QuickAccessLayout from "@/components/layout/PageLayout";
import { AppData, AudioFile } from "@/types";
import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from "react";

const isClient = typeof window !== "undefined";
// send file to server
export async function shareMusicFile(filePath: string) {
  console.log("sharing music files");
  const ipAddr =
    isClient &&
    (await invoke("get_ip_address").catch((err) => {
      console.log("error getting ip addr due to ", (err as Error).message);
    }));
  const uploadPath = `${ipAddr}/upload`;
  console.log("the upload path is ", uploadPath);

  /* isClient && upload(
      uploadPath,
      filePath, // the path to the file to upload
      (progress, total) => console.log(`Downloaded ${progress} of ${total} bytes`) // a callback that will be called with the upload progress
      // { 'ContentType': 'text/plain' } // optional headers to send with the request
    ) */
}
export default function Music() {
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
    return <h2>fetching files </h2>;
  }
  return (
    <QuickAccessLayout pageTitle={"Music"} includeSearchBar={true}>
      <div>
        <div className="flex flex-wrap  flex-grow gap-4 justify-start mt-12">
          {fetchedDocuments?.data.map((file, index) => (
            <DocumentComponent
              key={index}
              fileName={file.fileName}
              fileSize={file.fileSize}
              fileFormat={file.fileFormat}
              filePath={file.filePath}
              onClick={() => shareMusicFile(file.filePath)}
            />
          ))}
        </div>
      </div>
    </QuickAccessLayout>
  );
}
