"use client";
import Card from "@/components/Card";
import MobileAppLayout from "@/components/layout/MobileAppLayout";
import SearchBar from "@/components/Search";
import SmallText from "@/components/SmallText";
import View from "@/components/View";
import {
  FolderArrowDownIcon,
  LinkIcon,
  PaperAirplaneIcon,
} from "@heroicons/react/24/solid";
import React from "react";

export default function MobileAppEntryPoint() {
  return (
    <View className="hidden">
      <SearchBar
        className="bg-gray-50 rounded-lg mb-8"
        onSearch={function (city: string): void {
          throw new Error("Function not implemented.");
        }}
        placeholder={""}
      ></SearchBar>

      <Card>hahha</Card>

      <ul>{}</ul>

      <View className="grid grid-cols-3 justify-center gap-x-4 items-center">
        <Card className="flex flex-col items-center justify-center p-2 gap-2 bg-gray-50 rounded">
          <PaperAirplaneIcon className="text-app w-8 h-8 col-auto"></PaperAirplaneIcon>
          <SmallText>Send</SmallText>
        </Card>
        <Card className="flex flex-col items-center justify-center col-auto p-2 gap-2 bg-gray-50 rounded">
          <FolderArrowDownIcon className="text-app w-8 h-8"></FolderArrowDownIcon>
          <SmallText>Receive</SmallText>
        </Card>
        <Card className="flex flex-col justify-center col-auto items-center gap-2 p-2 rounded bg-gray-50">
          <LinkIcon className="h-8 w-8 text-app-600"></LinkIcon>
          <SmallText>Web link</SmallText>
        </Card>
      </View>
    </View>
  );
}
