"use client";

import Text from "@/components/Text";
import View from "@/components/View";
import PageLayout from "@/components/layout/PageLayout";
import QRCode from "react-qr-code";
import { useContext } from "react";
import { SystemInformationContext } from "@/store/sys-info";
import Heading from "@/components/Heading";

export default function ConnectionPage() {
  const { serverBaseUrl } = useContext(SystemInformationContext);

  return (
    <>
      <PageLayout pageTitle={"Connect Device"} includeSearchBar={false}>
        <View
          className=" text-center flex flex-col justify-center items-center"
          style={{ height: "600px" }}
        >
          
          <QRCode
            value={serverBaseUrl? serverBaseUrl : " "}
            size={256}
            fgColor="#111827"
          />

          <View>
            <Heading className="mt-8 text-gray-700">Connect Device</Heading>
            <Text className="mb-8 leading-5">
              Scan the QR code below to connect your device
            </Text>
          </View>
        </View>
      </PageLayout>
    </>
  );
}
