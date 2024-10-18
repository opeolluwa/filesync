"use client";
import Card from "@/components/Card";
import Heading from "@/components/Heading";
import { FileTransferHistory } from "@/components/history/TransferHistory";
import MobileViewLayout from "@/components/layout/mobile/MobileViewLayout";
import SmallText from "@/components/SmallText";
import View from "@/components/View";
import {
  FolderArrowDownIcon,
  LinkIcon,
  PaperAirplaneIcon,
} from "@heroicons/react/24/solid";
import { Progress } from "antd";
import Link from "next/link";

export default function MobileAppEntryPoint() {
  const fileTransfers = [
    {
      id: "1",
      fileName: "test",
      fileSize: "300000635",
      date: "Wed, October 2 2024",
      transactionType: "sent",
      recipient: "MacBook Pro 2021",
    },
    {
      id: "2",
      fileName: "test",
      fileSize: "300000635",
      date: "Wed, October 2 2024",
      transactionType: "sent",
      recipient: "MacBook Pro 2021",
    },
    {
      id: "3",
      fileName: "test",
      fileSize: "300000635",
      date: "Wed, October 2 2024",
      transactionType: "sent",
      recipient: "MacBook Pro 2021",
    },
    {
      id: "4",
      fileName: "test",
      fileSize: "300000635",
      date: "Wed, October 2 2024",
      transactionType: "sent",
      recipient: "MacBook Pro 2021",
    },
    {
      id: "5",
      fileName: "test",
      fileSize: "300000635",
      date: "Wed, October 2 2024",
      transactionType: "sent",
      recipient: "MacBook Pro 2021",
    },
  ];

  return (
    <MobileViewLayout includeFooter={true} includeHeader={true}>
      <Card className="bg-gray-50  mb-4 flex gap-x-8 items-center rounded-lg  ">
        <Progress
          type={"circle"}
          percent={43}
          size={80}
          steps={10}
          trailColor="rgba(0, 0, 0, 0.06)"
          strokeWidth={15}
        />
        <View className="">
          <SmallText className="text-gray-500">Used memory</SmallText>
          <Heading className="text-black font-bold mt-1 mb-3  leading-3">
            100Mb of 345GB used
          </Heading>
          <SmallText className="text-app">Manage Storage </SmallText>
        </View>
      </Card>
      <View className="grid  grid-cols-3 justify-center gap-x-4 items-center">
        <Card className="flex flex-col items-center justify-center p-2 gap-2 bg-gray-50 rounded">
          <PaperAirplaneIcon className="text-app w-8 h-8 col-auto"></PaperAirplaneIcon>
          <SmallText>Send</SmallText>
        </Card>
        <Card className="flex flex-col items-center justify-center col-auto p-2 gap-2 bg-gray-50   rounded">
          <FolderArrowDownIcon className="text-app w-8 h-8"></FolderArrowDownIcon>
          <SmallText>Receive</SmallText>
        </Card>
        <Card className="flex flex-col justify-center col-auto items-center gap-2 p-2 rounded bg-gray-50  ">
          <LinkIcon className="h-8 w-8 p-2 rounded-full text-white bg-app"></LinkIcon>
          <SmallText>Web link</SmallText>
        </Card>
      </View>
      <View>
        <Card className="flex justify-between items-center mt-3">
          <Heading className="font-bold"> Recent files</Heading>
          <Link href={"/mobile/history"}>
            <SmallText className="text-app">View all </SmallText>
          </Link>
        </Card>
        <View>
          {fileTransfers.map((transfer) => (
            <FileTransferHistory
              key={transfer.id}
              id={transfer.id}
              fileName={transfer.fileName}
              fileSize={transfer.fileSize}
              date={transfer.date}
              transactionType={transfer.transactionType}
              recipient={transfer.recipient}
            />
          ))}
        </View>
      </View>
    </MobileViewLayout>
  );
}
