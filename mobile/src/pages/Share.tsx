import { IonContent, IonPage } from "@ionic/react";
import Thumbnail from "../components/thumbnail";
import { File } from "@filesync/types/File";
import { Card, Heading, View } from "@filesync/components";
import { CloudArrowUpIcon } from "@heroicons/react/24/outline";
import { useContext } from "react";
import type { UploadProps } from "antd";
import { message, Upload } from "antd";
import { BASE_URL, SystemInformationContext } from "../store/global";


export enum FileTransferStatus {
  DOWNLOADING = "downloading",
  PAUSED = "paused",
  PENDING = "pending",
  UPLOADING = "uploading",
  COMPLETED = "done",
  ERROR = "error",
}

const Share: React.FC = () => {
  const { Dragger } = Upload;
//   const { onUpdate } = useContext(FileContext);
  const { serverBaseUrl } = useContext(SystemInformationContext);
  const serverAddress = BASE_URL + "/upload";

  const props: UploadProps = {
    name: "file",
    multiple: true,
    action: serverAddress,
    async onChange(info) {
      const { status } = info.file;
      if (status !== FileTransferStatus.UPLOADING) {
        // onUpdate(info.fileList);
      }
      if (status === FileTransferStatus.COMPLETED) {
        message.success(`${info.file.name} file uploaded successfully.`);
        // save the file to transfer history

        const fileName = info.file.name;
        const fileSize = String(info.file.size);
        const transactionType = "sent";

        // add file to transfer history
        // const transferHistory: TransferHistoryBuilder = {
        //   fileName,
        //   fileSize,
        //   transactionType,
        //   recipient: "",
        // };

        // const data = await invoke("persist_transfer_history", {
        //   file: transferHistory,
        // });
        // console.log(JSON.stringify(data));
      } else if (status === FileTransferStatus.ERROR) {
        message.error(
          `${info.file.name} file upload failed. due to ${JSON.stringify(info.file.error)}`
        );
      }
    },
    onDrop(e) {
      console.log("Dropped files", e.dataTransfer.files);
    },
  };

  return (
    <>
      <IonPage id="main-content" className="bg-accent">
        <IonContent className="ion-padding">
          <Dragger className="h-[300px]   block" {...props}>
            <CloudArrowUpIcon className="text-app-300  text-center small w-20 mx-auto" />
            <p className="text-gray-400 leading-2">
              Drop files here to share or click to browse
            </p>
          </Dragger>



          <Card className="flex gap-4 mt-6 hidden">
            <Card className="w-1/2 rounded-lg bg-app h-[50px] text-white flex gap-2 items-center justify-start">
              <i className="ri-send-plane-line text-2xl text-white bg-app p-2 rounded-full"></i>
              <Heading context="Send" className="text-white" />
            </Card>

            <Card className="w-1/2 rounded-lg bg-app h-[50px] text-white flex gap-2 items-center justify-start">
              <i className="ri-download-cloud-line text-2xl text-white bg-app p-2 rounded-full"></i>
              <Heading context="Receive" className="text-white" />
            </Card>
          </Card>
        </IonContent>
      </IonPage>
    </>
  );
};

export default Share;
