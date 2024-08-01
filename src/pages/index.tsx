"use client";

import Heading from "@/components/Heading";
import View from "@/components/View";
import PageLayout from "@/components/layout/PageLayout";
import Text from "@/components/Text";
import { ArrowDownIcon, ArrowUpIcon } from "@heroicons/react/24/outline";
import Link from "next/link";


export default function DesktopApp() {
  return (
    <>
      <PageLayout pageTitle={"Connect Device"} includeSearchBar={false}>
        <View className=" text-center flex flex-col align-center justify-center items-center h-[500px]">
          <View>
            <Heading className=" text-gray-700 ">
              Select Operational Mode
            </Heading>
            <Text>What would you like to do?</Text>

            <View className="flex justify-between items-center mt-8">
              <Link
                href="/connection/peer"
                className="flex flex-col items-center "
              >
                <ArrowUpIcon className=" bg-gray-200 text-gray-400 hover:bg-app-50 hover:text-app transition-all duration-200  p-4 rounded-xl shadow hover:shadow-none cursor-pointer"></ArrowUpIcon>
                <Text className="mt-2">Send File</Text>
              </Link>
              <Link
                href="/connection/recieve"
                className="flex flex-col items-center "
              >

                <ArrowDownIcon className="bg-gray-200 text-gray-400 hover:bg-app-50 hover:text-app transition-all duration-200 p-4 rounded-xl shadow hover:shadow-none cursor-pointer w-[70px] h-[70px]"></ArrowDownIcon>
                <Text className="mt-2">Recieve File</Text>
              </Link>
            </View>
          </View>
        </View>
      </PageLayout>
    </>
  );
}
