"use client";

import Heading from "@/components/Heading";
import Text from "@/components/Text";
import View from "@/components/View";
import PageLayout from "@/components/layout/PageLayout";
import { LinuxOutlined } from "@ant-design/icons";
import Link from "next/link";

export default function ConnectionPage() {
  return (
    <>
      <PageLayout pageTitle={"Connect Device"} includeSearchBar={false}>
        <View className=" text-center flex flex-col justify-center items-center h-[500px]">
          <View>
            <Heading className=" text-gray-700 text-2xl">Select Device</Heading>
            <Text className="mb-8 mt-1  leading-1">
              Select device type to begin
            </Text>
          </View>

          <View className="flex items-center gap-6 justify-center text-4xl font-thin">
            <Link
              href="/connection/desktop"
              className="  bg-gray-200 hover:bg-app-50 hover:text-app transition-all duration-200  p-8 box-border rounded-full shadow hover:shadow-none cursor-pointer w-[70px] h-[70px] flex justify-center items-center"
            >
              <LinuxOutlined />
            </Link>
            <Link
              href="/connection/mobile"
              className="ri-android-fill  bg-gray-200 hover:bg-app-50 hover:text-app transition-all duration-200  box-border p-8 rounded-full shadow hover:shadow-none cursor-pointer w-[70px] h-[70px] flex justify-center items-center"
            ></Link>
            <Link
              href="/connection/desktop"
              className="ri-windows-fill  bg-gray-200 hover:bg-app-50 hover:text-app transition-all duration-200  p-8 rounded-full shadow hover:shadow-none cursor-pointer w-[70px] h-[70px] flex justify-center items-center"
            ></Link>
            <Link
              href="/connection/desktop"
              className="ri-apple-fill  bg-gray-200 hover:bg-app-50 hover:text-app transition-all duration-200  p-8 rounded-full shadow hover:shadow-none cursor-pointer w-[70px] h-[70px] flex justify-center items-center"
            ></Link>
            <Link
              href="/connection/browser"
              className="ri-global-line  bg-gray-200 hover:bg-app-50 hover:text-app transition-all duration-200  p-8 rounded-full shadow hover:shadow-none cursor-pointer w-[70px] h-[70px] flex justify-center items-center"
            ></Link>
          </View>
        </View>
      </PageLayout>
    </>
  );
}
