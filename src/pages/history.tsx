import PageLayout from "@/components/layout/PageLayout";
import { useEffect, useState } from "react";
import LoaderCircle from "@/components/loaders/LoaderCircle";
import { CommandData } from "../../core/bindings/CommandData";
import { TransferHistory } from "../../core/bindings/TransferHistory";
import { invoke } from "@tauri-apps/api/tauri";
import Image from "next/image";
import { FileHistory } from "@/components/history/TransferHistory";

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
  const fetchedData = data as unknown as CommandData<
    Array<TransferHistory>
  >;
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
        <div className="w-full  px-2 ">
          <>
            {fetchedData?.data?.length == 0
              ? (
                <div className="flex flex-col items-center gap-4 mt-10">
                  <Image
                    src={"/icons/empty-state.svg"} // Route of the image file
                    height={100} // Desired size with correct aspect ratio
                    width={100} // Desired size with correct aspect ratio
                    alt="file card icon"
                    className="" // automatic height calculation
                  />
                  <h3 className="text-gray-400 mt-1 italic">no recent files</h3>
                </div>
              )
              : (
                fetchedData?.data?.map((history, index) => {
                  <FileHistory
                    key={index}
                    id={history.id}
                    fileName={history.fileName}
                    fileSize={history.fileSize}
                    date={history.date}
                    transactionType={history.transactionType}
                    recipient={history.recipient}
                  />;
                })
              )}
          </>
          {JSON.stringify(fetchedData.data)}
        </div>
      </PageLayout>
    </>
  );
}
