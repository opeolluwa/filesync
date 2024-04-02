"use client";

import PageTitle from "@/components/PageTitle";
import SearchBar from "@/components/Search";
import LoaderCircle from "@/components/loaders/LoaderCircle";
import {
  ArchiveBoxIcon,
  Bars3BottomLeftIcon,
  DocumentDuplicateIcon,
  MusicalNoteIcon,
  PhotoIcon,
  PlayIcon,
} from "@heroicons/react/24/solid";
import { invoke } from "@tauri-apps/api/tauri";
import Link from "next/link";
import { useEffect, useState } from "react";
import { CommandData } from "../../core/bindings/CommandData";
import { TransferHistory } from "../../core/bindings/TransferHistory";
import { computeFileSize } from "@/utils";
import Text from "@/components/Text";
import Heading from "@/components/Heading";
import { LoadingOutlined } from "@ant-design/icons";
import { Spin } from "antd";
import { FilmIcon } from "@heroicons/react/24/outline";
interface QuickAccessTab {
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
    name: "Document",
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
];

export default function Main() {
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
  const transferHistory = data as unknown as CommandData<
    Array<TransferHistory>
  >;
  if (isLoading) {
    return (
      <>
        <Spin indicator={antIcon} />
        <Heading context="Loading..." withStyle="font-xl font-bold" />
        <Text context="Please wait while we load your documents. This might take a while." />
      </>
    );
  }
  return (
    <>
      <section>
        <SearchBar
          onSearch={function (city: string): void {
            throw new Error("Function not implemented.");
          }}
          placeholder={"search files"}
        />
      </section>

      <section className="my-12">
        <PageTitle styles="mb-12" title={"Quick Access"} />
        <ul className="grid grid-flow-col col-4 gap-24 items-center justify-start mt-4 px-8">
          {quickAccessTabs.map((tab, index) => (
            <li
              key={index}
              className="flex flex-col items-center justify-evenly w-6 h-6 lg:w-20 lg:h-20"
            >
              <Link
                href={"quick-access/" + tab.name.toLowerCase()}
                className="rounded-[12px] px-3 hover:bg-[#3074f5]"
                style={{
                  // backgroundColor: "#3074f5",
                  backgroundColor: "#578EF7",
                }}
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
