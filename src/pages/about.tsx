import Image from "next/image";
import PageLayout from "@/components/layout/PageLayout";
import Heading from "@/components/Heading";
import Text from "@/components/Text";

export default function HelpPage() {
  return (
    <>
      <PageLayout pageTitle={"Help"} includeSearchBar={false}>
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
              <Text context="Give a star on GitHub" />
              <a
                href="https://github.com/opeolluwa/filesync"
                className="small text-gray text-dark"
              >
                https://github.com/opeolluwa/filesync
              </a>{" "}
              {""}
              <small className="text-gray-400 block mt-4 ">
                &copy; Adeoye Adefemi {new Date().getFullYear()}
              </small>
            </div>
          </div>
        </div>
      </PageLayout>
    </>
  );
}
