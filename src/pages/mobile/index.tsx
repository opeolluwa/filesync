import React from "react";
import { Tabs } from "antd";
import type { TabsProps } from "antd";
import Button from "@/components/Button";
import { FloatButton } from "antd";

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
      <header className="px-4">
        <nav>heheh</nav>
      </header>

      <main className="px-4">
        <Tabs
          defaultActiveKey="1"
          items={items}
          onChange={onChange}
          className=" p-0 m-0 "
        />
      </main>
      <FloatButton className="bg-app-400 text-white" onClick={() => console.log("onClick")} />
    </>
  );
}
