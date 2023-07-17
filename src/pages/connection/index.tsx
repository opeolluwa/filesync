import PageTitle from "@/components/PageTitle";
import PageLayout from "@/components/layout/PageLayout";

import Image from "next/image";
import Link from "next/link";
export default function HelpPage() {
  return (
    <>
      <PageLayout pageTitle={"Connect Device"} includeSearchBar={false}>
        <div className="flex flex-col items-center justify-center gap-4 my-auto">
          <div className="flex items-center  justify-center mx-auto">
            <Image
              src="/icons/Computer-Tablet-and-Phone-Vectors---1.0.0.svg"
              alt="recieve files"
              className="w-[30%]"
              width={400}
              height={400}
            />
          </div>
          <div className="mt-8">
            <PageTitle
              styles="mb-2"
              title={"Select Connection mode"}
            ></PageTitle>
            <p className="text-gray-400 l">Choose transfer mode to begin.</p>
          </div>
          <div className="flex gap-10">
            <Link
              href={"/connection/send"}
              className=" capitalize text-center flex flex-col justify-center text-white bg-gray-400 hover:bg-app  rounded px-4 py-2  w-20"
            >
              send
            </Link>
            <Link
              href={"/connection/receive"}
              className=" capitalize text-center flex flex-col justify-center text-white bg-gray-400 hover:bg-app   rounded  w-20"
            >
              receive
            </Link>
          </div>
        </div>
      </PageLayout>
    </>
  );
}
