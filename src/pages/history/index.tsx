import PageLayout from "@/components/layout/desktop/DesktopViewLayout";
import LoaderCircle from "@/components/Loaders/LoaderCircle";
import type { TabsProps } from "antd";
import { Tabs } from "antd";
import React, { useState } from "react";
import { CommandData } from "tauri/bindings/CommandData";
import { TransferHistory } from "tauri/bindings/TransferHistory";

type Align = "start" | "center" | "end";

const items: TabsProps["items"] = [
  { key: "1", label: "Sent files", children: "sent files" },
  { key: "2", label: "Received files", children: "received files" },
];


export default function HistoryPage() {
  // get the file transfer history data from the application backend
  const [data, setData] = useState(null);
  const [isLoading, setLoading] = useState(false);
  const [alignValue, setAlignValue] = React.useState<Align>("center");

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
      <PageLayout
        pageTitle={"Transfer History"}
        includeSearchBar={false}
        includePageTitle={false}
      >
        <Tabs
          defaultActiveKey="1"
          items={items}
          indicator={{ size: (origin) => origin - 20, align: alignValue }}
          className="-mt-4"
        />
      </PageLayout>
    </>
  );
}
