"use client";

import PageTitle from "@/components/PageTitle";
import SearchBar from "@/components/Search";
import Text from "@/components/Text";

import LoaderCircle from "@/components/Progress/LoaderCircle";
import { WifiStatusContext } from "@/store/network";
import { computeFileSize } from "@/utils";
import { LoadingOutlined } from "@ant-design/icons";
import {
  ArchiveBoxIcon,
  CloudArrowDownIcon,
  DocumentDuplicateIcon,
  FilmIcon,
  MusicalNoteIcon,
  PhotoIcon,
} from "@heroicons/react/24/outline";
import { ask } from "@tauri-apps/plugin-dialog";
import { Spin } from "antd";
import Link from "next/link";
import { useContext, useState } from "react";
import Heading from "@/components/Heading";
import { CommandData } from "tauri/bindings/CommandData";
import { TransferHistory } from "tauri/bindings/TransferHistory";

export interface QuickAccessTab {
  name: string;
  icon: any;
}
const antIcon = <LoadingOutlined style={{ fontSize: 24 }} spin />;

const quickAccessTabs: QuickAccessTab[] = [
  {
    name: "Pictures",
    icon: (
      <PhotoIcon className="rounded-lg my-4 mx-2 flex w-[40px] text-gray-100 " />
    ),
  },
  {
    name: "Audio",
    icon: (
      <MusicalNoteIcon className="rounded-lg my-4 mx-2 flex w-[40px]   text-gray-100 " />
    ),
  },
  {
    name: "Documents",
    icon: (
      <DocumentDuplicateIcon className="rounded-lg my-4 mx-2 flex w-[40px]   text-gray-100 " />
    ),
  },
  {
    name: "Videos",
    icon: (
      <FilmIcon className="rounded-lg my-4 mx-2 flex w-[40px]   text-gray-100 " />
    ),
  },
  {
    name: "Zipped",
    icon: (
      <ArchiveBoxIcon className="rounded-sm my-4 mx-2 flex w-[40px]  text-gray-100 " />
    ),
  },
  {
    name: "Downloads",
    icon: (
      <CloudArrowDownIcon className="rounded-sm my-4 mx-2 flex w-[40px]  text-gray-100 " />
    ),
  },
];

export default function Main() {
  const [data, setData] = useState(null);
  const [isLoading, setLoading] = useState(false);
  const { data: isConnectedToWifi } = useContext(WifiStatusContext);

  // typecast the response into AppData type
  const transferHistory = data as unknown as CommandData<
    Array<TransferHistory>
  >;

  if (isLoading) {
    return (
      <>
        <Spin indicator={antIcon} />
        <Heading context="Loading..." className="font-xl font-bold" />
        <Text>
          Please wait while we load your documents. This might take a while.
        </Text>
      </>
    );
  }

  // if done loading and not connected to wifi
  if (!isLoading && !isConnectedToWifi) {
    return (
      <>
        <div className="">
          <div className="">
            <LoaderCircle />
          </div>
          <PageTitle title={" Waiting for WiFi Connection"} />
          <p className="mt-2  py-4 w-3/4 dark:text-gray-400 leading-2 font-medium">
            You should see your system files as soon as you are connected to a
            WiFi network
          </p>
          <div className="flex gap-5">
            <button className=" bg-app text-white px-4 py-1 rounded w-24 ">
              Refresh
            </button>
            <button className=" px-4 py-1 border-2 text-gray-400 border-gray-400 rounded w-24">
              Exit
            </button>
          </div>
        </div>
      </>
    );
  }
  return (
    <>
      <section className="hidden">
        <SearchBar
          onSearch={function (): void {
            throw new Error("Function not implemented.");
          }}
          placeholder={"search files"}
        />
      </section>

      <section className="my-12">
        <PageTitle styles="mb-12" title={"Quick Access"} />
        <ul className="grid grid-flow-col col-4 gap-24 items-center justify-start mt-4 px-6 ">
          {quickAccessTabs.map((tab, index) => (
            <li
              key={index}
              className="flex flex-col items-center justify-evenly w-6 h-12"
            >
              <Link
                href={"quick-access/" + tab.name.toLowerCase()}
                className="rounded-lg flex px-3 hover:bg-[#3074f5] bg-app-400"
              >
                <div className="hover:brightness-25 sepia-0">{tab.icon}</div>
              </Link>
              <span className="text-gray-600 block mt-2 text-small">
                {tab.name}
              </span>
            </li>
          ))}
        </ul>
      </section>

      <section className="my-16">
        <h2 className="flex justify-between mt-24 mb-4 ">
          <span className=" font-medium text-gray-400">
            <PageTitle styles="" title={"Recent Files"} />
          </span>
          <Link href="/history" className="text-gray-500 text-violet-600 ">
            view all
          </Link>
        </h2>
        <div className="relative overflow-x-auto bg-white rounded-[24px] shadow-sm px-4 py-8">
          <table className="w-full text-sm text-left">
            <thead className="text-gray-500">
              <tr>
                <th scope="col" className="px-6 py-3 rounded-l-lg">
                  Name
                </th>
                <th scope="col" className="px-6 py-3">
                  Size
                </th>
                <th scope="col" className="px-6 py-3 rounded-r-lg">
                  Last Modified
                </th>
              </tr>
            </thead>
            <tbody className="text-gray-500 transition-all delay-75 ease-in">
              {isLoading ? (
                <Spin indicator={antIcon} />
              ) : (
                transferHistory?.data
                  ?.sort(
                    (a, b) =>
                      new Date(a.date).getTime() - new Date(b.date).getTime()
                  )
                  .reverse()
                  .slice(0, 5)
                  .map((file, index) => (
                    <tr key={index}>
                      <td className="px-6 py-4">{file.fileName}</td>
                      <td className="px-6 py-4">
                        {computeFileSize(Number(file.fileSize))}
                      </td>
                      <td className="px-6 py-4">{file.date}</td>
                    </tr>
                  ))
              )}
            </tbody>
          </table>
        </div>
      </section>
      <section className="my-16 hidden">
        <h2 className="flex justify-between mt-20 mb-8 ">
          <span className=" font-medium dark:text-gray-400">Recent Files</span>
          <span className="text-gray-500 dark:text-violet">view all</span>
        </h2>
      </section>
    </>
  );
}
