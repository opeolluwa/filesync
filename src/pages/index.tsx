"use client";

import PageTitle from "@/components/PageTitle";
import SearchBar from "@/components/SearchBar";
import LoaderCircle from "@/components/loaders/LoaderCircle";
import {
  Bars3BottomLeftIcon,
  MusicalNoteIcon,
  PhotoIcon,
  PlayIcon,
} from "@heroicons/react/24/solid";
import { invoke } from "@tauri-apps/api/tauri";
import Link from "next/link";
import { useEffect, useState } from "react";
import { CommandData } from "../../core/bindings/CommandData";
import { TransferHistory } from "../../core/bindings/TransferHistory";
import { AppData } from "@/types";
import { computeFileSize } from "@/utils";

interface QuickAccessTab {
  name: string;
  icon: any;
}

const quickAccessTabs: QuickAccessTab[] = [
  {
    name: "Images",
    icon: (
      <PhotoIcon className="rounded-lg my-4 mx-2 flex w-[47.5px] text-gray-100 dark:text-dark-900" />
    ),
  },
  {
    name: "Music",
    icon: (
      <MusicalNoteIcon className="rounded-lg my-4 mx-2 flex w-[47.5px]   text-gray-100 dark:text-dark-900" />
    ),
  },
  {
    name: "Videos",
    icon: (
      <PlayIcon className="rounded-lg my-4 mx-2 flex w-[47.5px]   text-gray-100 dark:text-dark-900" />
    ),
  },
  {
    name: "Documents",
    icon: (
      <Bars3BottomLeftIcon className="rounded-lg my-4 mx-2 flex w-[47.5px]   text-gray-100 dark:text-dark-900" />
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
        <ul className="flex flex-wrap gap-24 items-center justify-start mt-4 px-8">
          {quickAccessTabs.map((tab, index) => (
            <li
              key={index}
              className="flex flex-col items-center justify-center w-10 h-10 lg:w-20 lg:h-20"
            >
              <Link
                href={"quick-access/" + tab.name.toLowerCase()}
                className="rounded-[12px] shadow shadow-gray-500 px-3 dark:shadow-none"
                style={{
                  // backgroundColor: "#3074f5",
                  backgroundColor: "#578EF7",
                }}
              >
                <div className="hover:brightness-25 sepia-0">{tab.icon}</div>
              </Link>
              <span className="text-gray-600 dark:text-dark-500 block mt-2 text-small">
                {tab.name}
              </span>
            </li>
          ))}
        </ul>
      </section>

      <section className="my-16">
        <h2 className="flex justify-between mt-24 mb-4 ">
          <span className=" font-medium dark:text-dark-400 text-gray-400">
            Recent Files
          </span>
          <Link
            href="/history"
            className="text-gray-500 text-violet-600 dark:text-dark"
          >
            view all
          </Link>
        </h2>
        <div className="relative overflow-x-auto bg-white rounded-[24px] shadow-lg px-4 py-8  dark:bg-dark-900 dark:shadow-none">
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
            <tbody className="text-gray-500">
              {isLoading
                ? "Loading..."
                : transferHistory?.data?.slice(0, 5).map((file, index) => (
                    <tr key={index}>
                      <td className="px-6 py-4">{file.fileName}</td>
                      <td className="px-6 py-4">
                        {computeFileSize(Number(file.fileSize))}
                      </td>
                      <td className="px-6 py-4">{file.date}</td>
                    </tr>
                  ))}
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
