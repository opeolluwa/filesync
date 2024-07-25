import PageLayout from "@/components/layout/PageLayout";
import { useContext } from "react";
import type { UploadProps } from "antd";
import { message, Upload } from "antd";
import { FileContext, FileTransferStatus } from "@/store/context";
import { CloudArrowUpIcon } from "@heroicons/react/24/outline";
import { SystemInformationContext } from "@/store/sys-info";
import { TransferHistoryBuilder } from "../../core/bindings/TransferHistoryBuilder";
import { invoke } from "@tauri-apps/api/core";

/**
 * @function sharePage -  A page responsible for guiding users on various actions
 * @returns tsx
 */
export default function ShareFiles() {
  const { Dragger } = Upload;
  const { onUpdate } = useContext(FileContext);
  const { serverBaseUrl } = useContext(SystemInformationContext);
  const serverAddress = serverBaseUrl?.trim() + "/upload";

  const props: UploadProps = {
    name: "file",
    multiple: true,
    action: serverAddress,
    async onChange(info) {
      const { status } = info.file;
      if (status !== FileTransferStatus.UPLOADING) {
        onUpdate(info.fileList);
      }
      if (status === FileTransferStatus.COMPLETED) {
        message.success(`${info.file.name} file uploaded successfully.`);
        // save the file to transfer history

        const fileName = info.file.name;
        const fileSize = String(info.file.size);
        const transactionType = "sent";

        // add file to transfer history
        const transferHistory: TransferHistoryBuilder = {
          fileName,
          fileSize,
          transactionType,
          recipient: "",
        };

        const data = await invoke("persist_transfer_history", {
          file: transferHistory,
        });
        console.log(JSON.stringify(data));
      } else if (status === FileTransferStatus.ERROR) {
        message.error(
          `${info.file.name} file upload failed. due to ${info.file.error}`
        );
      }
    },
    onDrop(e) {
      console.log("Dropped files", e.dataTransfer.files);
    },
  };
  return (
    <>
      <PageLayout pageTitle={"Shared files"} includeSearchBar={false}>
        <Dragger className="lg:h-[500px] h-[400px]   block" {...props}>
          <CloudArrowUpIcon className="text-app-300  text-center small w-20 mx-auto" />
          <p className="text-gray-400 leading-2">
            Drop files here to share or click to browse
          </p>
        </Dragger>
      </PageLayout>
    </>
  );
}
