import React from "react";
import { Tabs } from "antd";
import type { TabsProps } from "antd";
import Button from "@/components/Button";
import { FloatButton } from "antd";
import { Bars3BottomRightIcon } from "@heroicons/react/24/outline";
import { ShareIcon } from "@heroicons/react/24/solid";

const onChange = (key: string) => {
  console.log(key);
};

const items: TabsProps["items"] = [
  {
    key: "History",
    label: "History",
    children: "Content of Tab Pane 1",
  },
  {
    key: "Download",
    label: "Download",
    children: "Content of Tab Pane 2",
  },
  {
    key: "Photo",
    label: "Photo",
    children: "Photo",
  },
  {
    key: "Music",
    label: "Music",
    children: "Music",
  },
  {
    key: "Video",
    label: "Video",
    children: "Video",
  },
];

export function MobileApp() {
  return (
    <>
      <header className="px-4 py-8">
        <nav className="flex items-center justify-between">
          <a href="">dd</a>
          <Bars3BottomRightIcon className="w-6 h-6" />
        </nav>
      </header>

      <main className="px-4">
        <Tabs
          defaultActiveKey="1"
          items={items}
          onChange={onChange}
          className=" p-0 m-0  bg-red "
        />
      </main>
      <FloatButton
        shape="square"
        type="primary"
        icon={<ShareIcon className="w-6 h-6" />}
        className="bg-app-400 text-white p-3"
        onClick={() => console.log("onClick")}
      />
    </>
  );
}
