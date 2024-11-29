"use client";

import Heading from "@/components/Heading";
import PageLayout from "@/components/layout/desktop/DesktopViewLayout";
import Text from "@/components/Text";
import View from "@/components/View";
import { WifiIcon } from "@heroicons/react/24/outline";

export default function ConnectionPage() {
  return (
    <>
      <PageLayout pageTitle={"Connect Device"} includeSearchBar={false}>
        <View className=" text-center flex flex-col justify-center items-center h-[500px]">
          <View>
            <WifiIcon className="w-6 h-6 mb-4 text-center" />
            <Heading className=" text-gray-700 text-2xl">
              Network Configuration
            </Heading>
            <Text> Connect to the network on peer device</Text>
            <View className="flex flex-col justify-between items-center mt-8"></View>
          </View>
        </View>
      </PageLayout>
    </>
  );
}
