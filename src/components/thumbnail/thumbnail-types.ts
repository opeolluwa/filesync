
// to interface with audio files coming from the application core

import { FileTransferStatus } from "@/store/context";

// the type extends the AppData type
export interface FileInterface extends File {}

// the required data to render the file card component
export interface FileCardInterface extends FileInterface {
  action?: () => void; // the action to perform when the file is clicked, for example it can be used to play an audio file
}

// the required data to render the file card component
// the data will be passed dynamically

export type TFileType = {
  fileType: string;
  fileName: string;
  fileSize: number;
  status: FileTransferStatus;
};


export interface FileTransferInterface {
  fileType: string;
  fileName: string;
  fileSize: number;
  status: FileTransferStatus;
}

export interface Props extends FileCardInterface {}