import PageTitle from "@/components/PageTitle";
import PageLayout from "@/components/layout/PageLayout";
import {
  ArrowDownCircleIcon,
  ArrowUpCircleIcon,
} from "@heroicons/react/24/outline";
import Image from "next/image";
import Link from "next/link";
export default function HelpPage() {
  return (
    <>
      <PageLayout pageTitle={"Connect Device"} includeSearchBar={false}>
        <div className="flex flex-col items-center justify-center gap-10 my-auto">
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
          <div className="flex items-center  justify-start gap-10">
            <Link
              href={"/connection/send"}
              className=" capitalize text-center flex flex-col justify-center hover:text-white hover:bg-app  rounded px-4 py-2 text-gray-400 w-20"
            >
              <ArrowUpCircleIcon className="w-10 h-10"></ArrowUpCircleIcon>
              send
            </Link>
            <Link
              href={"/connection/receive"}
              className=" capitalize text-center flex flex-col justify-center hover:text-white hover:bg-app  rounded px-4 py-2 text-gray-400 w-20"
            >
              <ArrowDownCircleIcon className="w-10 h-10 "></ArrowDownCircleIcon>
              receive
            </Link>
          </div>
        </div>
      </PageLayout>
    </>
  );
}
