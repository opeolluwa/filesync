import Image from "next/image";
import Heading from "@/components/Heading";
import Text from "@/components/Text";
import React, { useEffect, useState } from "react";
import PageLayout from "@/components/layout/desktop/DesktopViewLayout";
import { getName, getVersion } from "@tauri-apps/api/app";

export default function AboutPage() {
  let [appName, setAppName] = useState("");
  let [appVersion, setAppVersion] = useState("");

  useEffect(() => {
    const fetchData = async () => {
      try {
        const name = await getName();
        const version = await getVersion();

        setAppName(name);
        setAppVersion(version);
      } catch (error: any) {
        console.log(error.message);
      }
    };
    fetchData();
  }, []);
  return (
    <PageLayout pageTitle={"About"} includeSearchBar={false}>
      <div className="p-4 rounded-lg dark:dark-900">
        <div className="block mb-4 ">
          <Image
            src="/icons/app-icon.png"
            alt="app icon"
            width={200}
            height={200}
            className="w-[50px] block mx-auto"
          />

          <div className="flex flex-col justify-center items-center rounded-lg full my-4 p-4 ">
            <Heading className="text-center text-base capitalize">
              {appName} {appVersion}
            </Heading>
            <a
              href="https://github.com/opeolluwa/filesync"
              className="small text-gray text-dark text-center"
            >
              https://github.com/opeolluwa/filesync
            </a>{" "}
          </div>
        </div>
      </div>
    </PageLayout>
  );
}
