import Image from "next/image";
import PageLayout from "@/components/layout/PageLayout";
import Heading from "@/components/app/Heading";
import Text from "@/components/app/Text";

export default function HelpPage() {
  return (
    <>
      <PageLayout pageTitle={"Help"} includeSearchBar={false}>
        <div className="p-4 rounded-lg dark:dark-900">
          <div className="block w-[200px] mb-4 ">
            <Image
              src="/app-icon.png"
              alt="app icon"
              width={200}
              height={200}
            />

            <div className="flex flex-col items-center my-4 p-4 dark:bg-dark-900">
              <Heading context="File Sync" />
              <Text context="Securely share file without internet " />
            </div>

            <div className="flex flex-col items-center my-4 p-4 dark:bg-dark-900">
              <Text context=" give a star on GitHub" />
              <a href="https://github.com/opeolluwa/filesync">
                https://github.com/opeolluwa/filesync
              </a>{" "}
              {""}
              <small className="text-gray-400  dark:text-dark-400">
                &copy; Adeoye Adefemi {new Date().getFullYear()}
              </small>
            </div>
          </div>
        </div>
      </PageLayout>
    </>
  );
}
