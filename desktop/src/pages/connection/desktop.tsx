"use client";

import Heading from "@/components/Heading";
import Text from "@/components/Text";
import View from "@/components/View";
import PageLayout from "@/components/layout/PageLayout";
import { SystemInformationContext } from "@/store/sys-info";
import { useContext } from "react";
import { writeText } from "@tauri-apps/api/clipboard";
import { message } from "@tauri-apps/api/dialog";

export default function ConnectionPage() {
  const { port } = useContext(SystemInformationContext);

  const copyToClipboard = async () => {
    writeText(port.toString())
      .then(async () => {
        message("Text Copied", { title: "Tauri", type: "info" });
      })
      .then(() => {
        console.log("here goes nothing");
      });
  };
  return (
    <>
      <PageLayout pageTitle={"Connect Device"} includeSearchBar={false}>
        <View
          className=" text-center flex flex-col justify-center items-center"
          style={{ height: "500px" }}
        >
          <View>
            <Heading className="mt-8 text-3xl text-gray-700">
              Connect Device
            </Heading>
            <Text className="mb-8 leading-5">
              Provide the client id in the peer device
            </Text>

            <View className="flex items-center justify-center">
              <span className="text-5xl" onClick={copyToClipboard}>
                #{port}
              </span>
              <i className="ri-file-copy-line text-4xl ml-3 cursor-pointer hover:text-app"></i>
            </View>
          </View>
        </View>
      </PageLayout>
    </>
  );
}
