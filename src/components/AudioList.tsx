import React, { useEffect, useState } from "react";
import { FileEntry as TauriFile } from "@tauri-apps/api/fs";
const { readDir, BaseDirectory } = await import("@tauri-apps/api/fs");

const AudioList: React.FC = () => {
  const [audioFiles, setAudioFiles] = useState<TauriFile[]>([]);

  useEffect(() => {
    const fetchAudioFiles = async () => {
      try {
        // Reads the `$APPDATA/users` directory recursively
        const files = await readDir("/", {
          dir: BaseDirectory.Audio,
          // recursive: true,
        });

        /*  function processEntries(files: React.SetStateAction<TauriFile[]>) {
          for (const entry of files) {
            console.log(`Entry: ${entry.path}`);
            if (entry.children) {
              processEntries(entry.children);
            }
          }
        } */
        alert({ files });

        /*  setAudioFiles(
          // files
            files.filter((file: { name: string }) => file.name.endsWith(".mp3"))
        ); */
      } catch (error) {
        console.error("Error reading audio files:", error);
      }
    };

    fetchAudioFiles();
  }, []);

  
  return (
    <div>
      <h1>Audio List</h1>
      <ul>
        {audioFiles.map((file, index) => (
          <li key={index}>{file.name}</li>
        ))}
      </ul>
    </div>
  );
};

export default AudioList;
