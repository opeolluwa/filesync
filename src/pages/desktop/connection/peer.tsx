"use client";

import Heading from "@/components/Heading";
import Text from "@/components/Text";
import View from "@/components/View";
import PageLayout from "@/components/layout/PageLayout";
import { AndroidFilled, LinuxOutlined } from "@ant-design/icons";
import Link from "next/link";
import { Tooltip } from "antd";
export default function ConnectionPage() {
  return (
    <>
      <PageLayout pageTitle={"Connect Device"} includeSearchBar={false}>
        <View className=" text-center flex flex-col justify-center items-center h-[500px]">
          <View>
            <Heading className=" text-gray-700 text-2xl">
              Select Peer&apos;s Platform
            </Heading>
            <Text className="mb-8 mt-1  leading-1">
              Select device type to begin
            </Text>
          </View>

          <View className="flex items-center gap-6 justify-center text-4xl font-thin">
            <Tooltip title="Linux">
              <Link
                href="/connection/desktop"
                className="  bg-gray-200 hover:bg-app-50 hover:text-app transition-all duration-200  p-8 box-border rounded-full shadow hover:shadow-none cursor-pointer w-[70px] h-[70px] flex justify-center items-center"
              >
                <LinuxOutlined />
              </Link>
            </Tooltip>
            <Tooltip title="Android">
              <Link
                href="/connection/mobile"
                className="  bg-gray-200 hover:bg-app-50 hover:text-app transition-all duration-200  box-border p-8 rounded-full shadow hover:shadow-none cursor-pointer w-[70px] h-[70px] flex justify-center items-center"
              >
                <AndroidFilled />
              </Link>
            </Tooltip>
            <Tooltip title="macOS">
              <Link
                href="/connection/desktop"
                className="ri-apple-fill  bg-gray-200 hover:bg-app-50 hover:text-app transition-all duration-200  p-8 rounded-full shadow hover:shadow-none cursor-pointer w-[70px] h-[70px] flex justify-center items-center"
              ></Link>
            </Tooltip>
            <Tooltip title="Windows">
              <Link
                href="/connection/desktop"
                className="ri-windows-fill bg-gray-200 hover:bg-app-50 hover:text-app transition-all duration-200  p-8 rounded-full shadow hover:shadow-none cursor-pointer w-[70px] h-[70px] flex justify-center items-center"
              ></Link>
            </Tooltip>
            <Tooltip title="Browser">
              <Link
                href="/connection/browser"
                className="ri-window-2-line  bg-gray-200 hover:bg-app-50 hover:text-app transition-all duration-200  p-8 rounded-full shadow hover:shadow-none cursor-pointer w-[70px] h-[70px] flex justify-center items-center"
              ></Link>
            </Tooltip>
          </View>
        </View>
      </PageLayout>
    </>
  );
}
