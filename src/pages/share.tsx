import PageLayout from "@/components/PageLayout";
import {  useContext,  } from "react";
import type { UploadProps } from "antd";
import { message, Upload } from "antd";
import { FileContext, FileTransferStatus } from "@/store/context";
import { CloudArrowUpIcon } from "@heroicons/react/24/outline";


/**
 * @function sharePage -  A page responsible for guiding users on various actions
 * @returns tsx
 */
export default function ShareFiles() {
  const { Dragger } = Upload;
  const { onUpdate } = useContext(FileContext);

  const props: UploadProps = {
    name: "file",
    multiple: true,
	//TODO: use actual API fetch from the peer device information
    action: "https://www.mocky.io/v2/5cc8019d300000980a055e76",
    onChange(info) {
      const { status } = info.file;
      if (status !== FileTransferStatus.UPLOADING) {
        onUpdate(info.fileList);
        console.log(info.file, info.fileList);
        // TODO: added uploaded files to application transfer history
      }
      if (status === FileTransferStatus.COMPLETED) {
        message.success(`${info.file.name} file uploaded successfully.`);
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
        <Dragger className="h-[500px] block" {...props}>
          <CloudArrowUpIcon className="text-app-300  text-center small w-20 mx-auto" />
          <p className="text-gray-400 leading-2">
            Drop files here to share or click to browse
          </p>
        </Dragger>
      </PageLayout>
    </>
  );
}
