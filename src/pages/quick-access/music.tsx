import MusicFile from "@/components/thumbnail/MusicFile";
import QuickAccessLayout from "@/components/layout/PageLayout";
import { AppData, AudioFile } from "@/types";
import { convertFileSrc, invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from "react";
import { Skeleton } from "antd";
import PageTitle from "@/components/PageTitle";

export default function Music() {
  function playMusic(filePath: string) {
    const assetUrl = convertFileSrc(filePath);
    const audio = document.getElementById("testNode") as HTMLAudioElement;
    const source = document.createElement("source");
    source.type = "audio/mp3";
    source.src = assetUrl;
    audio?.appendChild(source);
    audio?.load();
    console.log("playing ", filePath);
  }

  const [data, setData] = useState(null);
  const [isLoading, setLoading] = useState(false);

  // get the data from the application core
  useEffect(() => {
    setLoading(true);
    invoke("fetch_audio_files").then((res) => {
      setData(res as any);
      setLoading(false);
    });
  }, []);
  // TODO(@opeolluwa): use Tauri Js API to render musicData
  // TODO(@opeolluwa) add modal to play audio file, audio and document using web APIs
  // typecast the response into AppData type
  const musicData = data as unknown as AppData<Array<AudioFile>>;
  if (isLoading) {
    
    return <Skeleton active />;
  }
  return (
    <QuickAccessLayout
      pageTitle={"Music"}
      includeSearchBar={true}
      searchBarText="search audio files"
    >
      <div>
        <div className="flex flex-wrap  flex-grow gap-4 justify-start mt-12">
          {musicData?.data.map((file, index) => (
            <MusicFile
              key={index}
              fileName={file.fileName}
              fileSize={file.fileSize}
              fileFormat={file.fileFormat}
              filePath={file.filePath}
              onClick={() => playMusic(file.filePath)}
            />
          ))}
        </div>
      </div>
    </QuickAccessLayout>
  );
}
