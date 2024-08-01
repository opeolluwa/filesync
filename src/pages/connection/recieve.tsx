"use client";

import Button from "@/components/Button";
import Heading from "@/components/Heading";
import Text from "@/components/Text";
import View from "@/components/View";
import PageLayout from "@/components/layout/PageLayout";
import Platform, {
  DevicePlatformContext,
  DevicePlatformInterface,
} from "@/store/platform";
import { CameraIcon } from "@heroicons/react/24/solid";
import { useContext } from "react";
export default function ConnectionPage() {
  const platform = useContext(DevicePlatformContext);
  if (
    platform.device === DevicePlatformInterface.ANDROID ||
    platform.device === DevicePlatformInterface.IOS
  ) {
    return (
      <>
        <PageLayout pageTitle={"Connect Device"} includeSearchBar={false}>
          <View className=" text-center flex flex-col justify-center items-center h-[500px]">
            <View>
              <Heading className=" text-gray-700 text-2xl">
                Scan QR code 
              </Heading>
              <Text className="mb-5 mt-1  leading-1">
                Scan QR code on peer&apos;s debvice to continue
              </Text>
            </View>
            <Button className="bg-app text-white flex items-center gap-2">
              Open Camera <CameraIcon className="w-6 h-6"/>
            </Button>
          </View>
        </PageLayout>
      </>
    );
  }
  else {
    return (
      <>
        <PageLayout pageTitle={"Connect Device"} includeSearchBar={false}>
          <View className=" text-center flex flex-col justify-center items-center h-[500px]">
            <View>
              <Heading className=" text-gray-700 text-2xl">
                Provide Client ID
              </Heading>
              <Text className="mb-8 mt-1  leading-1">
                What is the client id displayed on the peer device?
              </Text>
            </View>

            <View>
              <form action="">
                <View className="w-full">
                  <label htmlFor="clientId" className="block sr-only">
                    Client ID
                  </label>
                  <input
                    type="text"
                    name="clinetId"
                    id=""
                    className="py-3 px-4 rounded-lg  bg-gray-50 text-center form-input w-[300px]"
                    autoFocus
                  />
                </View>
              </form>
            </View>
          </View>
        </PageLayout>
      </>
    );
  }
}

export function DesktopClient() {
  return (
    <>
     
    </>
  );
}
