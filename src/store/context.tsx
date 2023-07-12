import { ReactNode, createContext, useState } from "react";

interface TFileData  {
  size: number;
  type: string;
  name: string;
  status: FileTransferStatus;
};

export enum FileTransferStatus {
  DOWNLOADING = "downloading",
  PAUSED = "paused",
  PENDING = "pending",
  UPLOADING = "uploading",
  COMPLETED = "done",
  ERROR = "error",
}
type TFileContext = {
  fileList: TFileData[];
  onUpdate: Function;
};
export const FileContext = createContext({} as TFileContext);

const FileStore = ({ children }: { children: ReactNode }) => {
  const [fileList, setFileList] = useState<any[]>([]);

  const updateFileList = (data: any) => {
    setFileList(data);
  };

  return (
    <FileContext.Provider
      value={{
        fileList,
        onUpdate: updateFileList,
      }}
    >
      {children}
    </FileContext.Provider>
  );
};

export default FileStore;
