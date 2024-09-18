import Image from "next/image";
import PageLayout from "@/components/layout/DesktopViewLayout";
import Text from "@/components/Text";
import { getName, getVersion } from "@tauri-apps/api/app";
import { useEffect, useState } from "react";

export default function HelpPage() {
  const [appName, setAppName] = useState("");
  const [appVersion, setAppVersion] = useState("");

  useEffect(() => {
    getVersion().then((version) => setAppVersion(version));
    getName().then((name) => setAppName(name));
  }, []);

  return (
    <>
      <PageLayout pageTitle={"About"} includeSearchBar={false}>
        <div className="p-4 rounded-lg">
          <div className="block mb-4 ">
            <Image
              src="icons/app-icon.png"
              alt="app icon"
              width={200}
              height={200}
              className="w-[50px] block mx-auto"
            />

            <div className="flex flex-col rounded-lg items-center my-4 p-4">
              <Text context={`${appName} v${appVersion}`} />
              <a
                href="https://github.com/opeolluwa/filesync"
                className="small text-gray text-dark"
              >
                https://github.com/opeolluwa/filesync
              </a>{" "}
              {""}
              <a
                href="https://twitter.com/_opeolluwa"
                className="text-gray-400 block mt-1 "
              >
                Adeoye Adefemi
              </a>
            </div>
          </div>
        </div>
      </PageLayout>
    </>
  );
}
