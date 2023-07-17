import {
  ArrowUpTrayIcon,
  CameraIcon,
  DocumentTextIcon,
  FolderOpenIcon,
  MusicalNoteIcon,
  PlayIcon,
} from "@heroicons/react/24/outline";
import { MemoryInformation } from "./components/MemoryInformation.tsx";
import PageTitle from "./components/PageTitle.tsx";
import SearchBar from "./components/SearchBar.tsx";
import { FloatButton, Modal, message, Upload } from "antd";
import { useState } from "react";
import type { UploadProps } from "antd";

const { Dragger } = Upload;

const props: UploadProps = {
  name: "file",
  multiple: true,
  action: "/upload",
  onChange(info) {
    const { status } = info.file;
    if (status !== "uploading") {
      console.log(info.file, info.fileList);
    }
    if (status === "done") {
      message.success(`${info.file.name} file uploaded successfully.`);
    } else if (status === "error") {
      message.error(`${info.file.name} file upload failed.`);
    }
  },
  onDrop(e) {
    console.log("Dropped files", e.dataTransfer.files);
  },
};

interface Tab {
  name: string;
  path: string;
  icon: any;
  color: string;
  files?: number; // 200 Files
  size?: string; // 56 MB
}

function App() {
  const [isModalOpen, setIsModalOpen] = useState(false);

  const showModal = () => {
    setIsModalOpen(true);
  };

  const handleOk = () => {
    setIsModalOpen(false);
  };

  const handleCancel = () => {
    setIsModalOpen(false);
  };

  const routes: Array<Tab> = [
    {
      name: "document",
      path: "/document",
      color: "bg-yellow-50",
      icon: <DocumentTextIcon className="text-yellow-500 w-6 h-6" />,
    },
    {
      name: "images",
      path: "/images",
      color: "bg-sky-50",
      icon: <CameraIcon className="text-sky-500 w-6 h-6" />,
    },
    {
      name: "videos",
      path: "/videos",
      icon: <PlayIcon className="text-green-500 w-6 h-6" />,
      color: "bg-green-50",
    },
    {
      name: "audios",
      path: "/audios",
      color: "bg-red-50",
      icon: <MusicalNoteIcon className="text-red-500 w-6 h-6" />,
    },
    {
      name: "folders",
      path: "/folder",
      color: "bg-purple-50",
      icon: <FolderOpenIcon className="text-purple-500 w-6 h-6" />,
    },
  ];
  return (
    <>
      <div className="bg-app-700  text-white rounded-b-[24px] px-8 py-8 mb-12">
        <PageTitle styles="mb-4 pb-2" title={"Overview"} />
        <SearchBar
          onSearch={function (): void {
            throw new Error("Function not implemented.");
          }}
          placeholder={"search files"}
        />
      </div>

      <div className="px-4 my-8">
        {/* <p className="leading-3 capitalize text-gray-400">Space used </p> */}
        <MemoryInformation systemName={"Sillicone"} freeMemory={"100 GB"} />
      </div>

      <div className="px-8 my-4">
        {routes.map((route, index) => {
          return (
            <div
              key={index}
              className="flex items-center justify-between py-4 cursor-pointer hover:bg-gray-50"
            >
              <div className="flex items-center">
                <div
                  className={
                    "w-10 h-10 rounded flex items-center justify-center mr-4 bg-red-50 " +
                    route.color
                  }
                >
                  {route.icon}
                </div>
                <div className="flex flex-col">
                  <p className="text-sm capitalize">{route.name}</p>
                  <p className="text-xs text-gray-400">
                    {route.files ? route.files : "0"} files
                  </p>
                </div>
              </div>
              <div className="text-sm text-gray-400">
                {route.size ? route.size : "0"} MB
              </div>
            </div>
          );
        })}
        <FloatButton
          shape="circle"
          type="primary"
          icon={<ArrowUpTrayIcon />}
          onClick={showModal}
        />
        <Modal
          title="Upload files"
          centered
          open={isModalOpen}
          onOk={handleOk}
          okText=""
          onCancel={handleCancel}
        >
          <Dragger {...props} className="min-h-[200px] block">
            <p className="text-sm gray-400">
              Click or drag file to this area to upload
            </p>
          </Dragger>
        </Modal>
      </div>
    </>
  );
}

export default App;
