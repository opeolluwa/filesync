import Image from "next/image";
import PageLayout from "@/components/layout/PageLayout";
import Heading from "@/components/app/Heading";
import Text from "@/components/app/Text";

export default function HelpPage() {
  return (
    <>
      <PageLayout pageTitle={"Help"} includeSearchBar={false}>
        <div className="p-4 rounded-lg dark:dark-900">
          <div className="block mb-4 ">
            <Image
              src="icons/app-icon.png"
              alt="app icon"
              width={200}
              height={200}
              className="w-[50px] block mx-auto"
            />

            <div className="flex flex-col rounded-lg full my-4 p-4 dark:bg-dark-900">
              <Heading context="File Sync" />
              <Text context="Securely share file without internet  " withStyle="mb-4" />
            </div>

            <div className="flex flex-col rounded-lg items-center my-4 p-4 dark:bg-dark-900">
              <Text context=" give a star on GitHub" />
              <a href="https://github.com/opeolluwa/filesync" className="small text-gray text-dark">
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
