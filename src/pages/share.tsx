import PageLayout from "@/components/PageLayout";
import { useContext } from "react";
import type { UploadProps } from "antd";
import { message, Upload } from "antd";
import { FileContext, FileTransferStatus } from "@/store/context";
import { CloudArrowUpIcon } from "@heroicons/react/24/outline";
import { SystemInformationContext } from "@/store/sys-info";
import { database, DatabaseTableNames } from "@/utils/database";

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
        // TODO: added uploaded files to application transfer history
      }
      if (status === FileTransferStatus.COMPLETED) {
        message.success(`${info.file.name} file uploaded successfully.`);
        // save the file to transfer history
        await database.execute(
          "CREATE TABLE IF NOT EXIST 1? (id INTEGER PRIMARY KEY AUTOINCREMENT, fileName VARCHAR, fileSize VARCHAR, transferType VARCHAR, transferDate TEXT); ",
          [DatabaseTableNames.FILE_TRANSFER_HISTORY.toString()]
        );
        // insert the newly transferred file
        const fileName = info.file.name;
        const fileSize = info.file.size;
        const transferType = "sent";
        const transferDate = new Date().toLocaleDateString("en-us", {
          month: "short",
          year: "numeric",
          weekday: "long",
          day: "numeric",
        });
        await database.execute(
          "INSERT INTO 1? (fileName, fileSize, transferType, transferDate) VALUES (?,?,?,?)",
          [
            DatabaseTableNames.FILE_TRANSFER_HISTORY.toString(),
            fileName,
            fileSize,
            transferType,
            transferDate,
          ]
        );
        // setFileTransferStatus({ status });
      } else if (status === FileTransferStatus.ERROR) {
        message.error(`${info.file.name} file upload failed.`);
      }
    },
    onDrop(e) {
      console.log("Dropped files", e.dataTransfer.files);
    },
  };
  return (
    <>
      <PageLayout pageTitle={"Shared files"} includeSearchBar={false}>
        <Dragger className="lg:h-[500px] h-[400px] block" {...props}>
          <CloudArrowUpIcon className="text-app-300  text-center small w-20 mx-auto" />
          <p className="text-gray-400 leading-2">
            Drop files here to share or click to browse
          </p>
        </Dragger>
      </PageLayout>
    </>
  );
}
