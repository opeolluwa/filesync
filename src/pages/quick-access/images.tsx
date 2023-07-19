import QuickAccessLayout from "@/components/layout/PageLayout";
import { BaseDirectory, FileEntry, readDir } from "@tauri-apps/api/fs";
// import { pictureDir } from '@tauri-apps/api/path';
import { useEffect, useState } from "react";
import dynamic from "next/dynamic";
import { Skeleton } from "antd";

// const pictureDir = dynamic(() => import('@tauri-apps/api/path').then((mod) => mod.pictureDir), { ssr: false });
//TODO(@opeolluwa): use Tauri Js API to fetch image files and render
export default function Images() {
  const [data, setData] = useState("");
  const [isLoading, setLoading] = useState(false);

  // get the data from the application core
  useEffect(() => {
    setLoading(true);
    // get the picture directory path
    const pictureDirPath = async () =>
      (await import("@tauri-apps/api/path")).pictureDir;
    // console.log(pictureDirPath, " the picture dir path");

    // read the pictures
    readDir("users", { dir: BaseDirectory.Picture, recursive: true }).then(
      (result) => {
        console.log(result, " the read files are");
        processEntries(result);
      }
    );
    // console.log(entries, " the read files are");

    function processEntries(entries: FileEntry[]) {
      for (const entry of entries) {
        console.log(`Entry: ${entry.path}`);
        if (entry.children) {
          processEntries(entry.children);
        }
      }
    }

    // setData(entries.toString());
    setLoading(false);
  }, []);

  if (isLoading) {
    return <h2>fetch your image files</h2>;
  }

  return (
    <QuickAccessLayout pageTitle={"Images"} includeSearchBar={false}>
      <Skeleton active />;
    </QuickAccessLayout>
  );
}
