"use client";

import Heading from "@/components/Heading";
import View from "@/components/View";
import PageLayout from "@/components/layout/PageLayout";
import { Text } from "@filesync/components";
import { ArrowDownIcon, ArrowUpIcon } from "@heroicons/react/24/outline";

export default function ConnectionPage() {
  return (
    <>
      <PageLayout pageTitle={"Connect Device"} includeSearchBar={false}>
        <View className=" text-center flex flex-col justify-center items-center h-[500px]">
          <View>
            <Heading className=" text-gray-700  text-2xl">
              What would you like to do?
            </Heading>
            <Text>some lorem text blablabla</Text>

            <View className="flex justify-between items-center mt-8">
              <a
                href="/connection/hotspot"
                className="flex flex-col items-center "
              >
                <ArrowUpIcon className=" bg-gray-200 hover:bg-app-50 hover:text-app transition-all duration-200  p-4 rounded-xl shadow hover:shadow-none cursor-pointer"></ArrowUpIcon>
                <Text className="mt-2">Send File</Text>
              </a>
              <a
                href="/connection/peer"
                className="flex flex-col items-center "
              >
                <ArrowDownIcon className="bg-gray-200 hover:bg-app-50 hover:text-app transition-all duration-200 p-4 rounded-xl shadow hover:shadow-none cursor-pointer w-[70px] h-[70px]"></ArrowDownIcon>
                <Text className="mt-2">Recieve File</Text>
              </a>
            </View>
          </View>
        </View>
      </PageLayout>
    </>
  );
}
