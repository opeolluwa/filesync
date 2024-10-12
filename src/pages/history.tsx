import PageLayout from "@/components/layout/desktop/DesktopViewLayout";
import { useEffect, useState } from "react";
import LoaderCircle from "@/components/loaders/LoaderCircle";
import { invoke } from "@tauri-apps/api/core";
import { FileHistory } from "@/components/history/TransferHistory";
import { TransferHistory } from "tauri/bindings/TransferHistory";
import { CommandData } from "tauri/bindings/CommandData";

export default function HistoryPage() {
  // get the file transfer history data from the application backend
  const [data, setData] = useState(null);
  const [isLoading, setLoading] = useState(false);

  // get the data from the application core
  useEffect(() => {
    setLoading(true);
    invoke("get_transfer_history").then((res) => {
      setData(res as any);
      setLoading(false);
    });
  }, []);

  // typecast the response into AppData type
  const fetchedData = data as unknown as CommandData<Array<TransferHistory>>;
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
    <>
      <PageLayout pageTitle={"Transfer History"} includeSearchBar={false}>
        <div className="justify-start my-1 first:my-1 last:mb-8">
          {fetchedData?.data?.map((history, index) => (
            <FileHistory
              key={index}
              id={history.id}
              fileName={history.fileName}
              fileSize={history.fileSize}
              date={history.date}
              transactionType={history.transactionType}
              recipient={history.recipient}
            />
          ))}
        </div>
      </PageLayout>
    </>
  );
}
