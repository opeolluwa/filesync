"use client";

import Card from "@/components/Card";
import Heading from "@/components/Heading";
import SearchBar from "@/components/Search";
import picturesIcon from "@/assets/extra/pictures.png";
import documentIcon from "@/assets/extra/google-docs.png";
import audioIcon from "@/assets/extra/audio.png";
import videoIcon from "@/assets/extra/video.png";
import MobileAppLayout from "@/components/layout/MobileAppLayout";
import SmallText from "@/components/SmallText";
import Image from "next/image";
export function MobileApp() {
  return (
    <MobileAppLayout>
      <header className=""></header>

      <main className="">
        <SearchBar
          onSearch={function (city: string): void {
            throw new Error("Function not implemented.");
          }}
          placeholder={"Search files"}
          className="mt-8"
        />

        <div className="my-12 grid grid-cols-2 grid-rows-2 gap-4">
          <Card className="bg-white px-4 py-6 rounded-xl shadow-md shadow-gray-200 cursor-pointer">
            <Image
              src={audioIcon}
              height={144}
              width={144}
              alt="file card icon"
              className="w-[50px]  mb-3 "
            />
            <Heading className="mt-4 mb-1">Audio</Heading>
            <SmallText>563 files - 2.45 GB</SmallText>
          </Card>

          <Card className="bg-white px-4 py-6 rounded-xl shadow-md shadow-gray-200 cursor-pointer">
            <Image
              src={videoIcon}
              height={144}
              width={144}
              alt="file card icon"
              className="w-[50px]  mb-3 "
            />
            <Heading className="mt-4 mb-1">Video</Heading>
            <SmallText>563 files - 2.45 GB</SmallText>
          </Card>

          <Card className="bg-white px-4 py-6 rounded-xl shadow-md shadow-gray-200 cursor-pointer">
            <Image
              src={picturesIcon}
              height={144}
              width={144}
              alt="file card icon"
              className="w-[50px]  mb-3 "
            />
            <Heading className="mt-4 mb-1">Pictures </Heading>
            <SmallText>563 files - 2.45 GB</SmallText>
          </Card>

          <Card className="bg-white px-4 py-6 rounded-xl shadow-md shadow-gray-200 cursor-pointer">
            <Image
              src={documentIcon}
              height={144}
              width={144}
              alt="file card icon"
              className="w-[50px]  mb-3 "
            />
            <Heading className="mt-4 mb-1">Documents</Heading>
            <SmallText>563 files - 2.45 GB</SmallText>
          </Card>
        </div>
      </main>
    </MobileAppLayout>
  );
}
