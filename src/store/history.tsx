// file transfer history
import { DatabaseTableNames, database } from "@/utils/database";
import { invoke } from "@tauri-apps/api/tauri";
import { ReactNode, createContext, useEffect, useState } from "react";
export interface SharedFile {
  id: number;
  fileName: string;
  fileSize: string;
  transferType: "sent" | "received";
  transferDate: string;
}

interface SharedFileHistory {
  sharedFilesList: SharedFile[];
}

export const SharedFileContext = createContext([] as SharedFile[]);
export default function SystemInfoStore({ children }: { children: ReactNode }) {
  let [sharedFiles, setSharedFileInfo] = useState<SharedFile[]>([]);

  useEffect(() => {
    database
      .select<Array<SharedFile>>("SELECT * FROM $1", [
        DatabaseTableNames.FILE_TRANSFER_HISTORY,
      ])
      .then((rows: SharedFile[]) => {
        return setSharedFileInfo(rows as SharedFile[]);
      });
  }, []);

  return (
    <SharedFileContext.Provider
      value={{
        ...sharedFiles,
      }}
    >
      {children}
    </SharedFileContext.Provider>
  );
}
