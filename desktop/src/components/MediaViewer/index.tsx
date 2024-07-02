"use client";

/// give preview for different file type


import { SystemInformationContext } from "@/store/sys-info";
import { useContext } from "react";
import { FileWindow as MediaViewer } from "@opeolluwa/filewindow";

const isClient = typeof window !== "undefined";

export enum FileType {
  Image = "image",
  Audio = "audio",
  PDF = "pdf",
  CSV = "csv",
  Presentation = "presentation",
  Video = "video",
  Archive = "archive",
  Document = "document",
  Text = "text",
  SVG = "svg",
  Default = "default",
}

interface Props {
  fileType: string;
  filePath: string;
}

export default function PreviewMedia({ fileType, filePath }: Props) {
  const { serverBaseUrl } = useContext(SystemInformationContext);

  const fileUrl = isClient
    ? `${serverBaseUrl}/api/file?file_path=${filePath}`
    : "";

  return (
    <div className="w-full h-full flex justify-center items-center">
      <MediaViewer fileUrl={fileUrl} fileName={""} fileExtension={fileType} />
    </div>
  );
}
