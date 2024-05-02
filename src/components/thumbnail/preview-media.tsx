"use client";

import { SystemInformationContext } from "@/store/sys-info";
import { useContext } from "react";
/// give preview for differet file type
import FileViewer from "react-file-viewer";

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
    <div className="w-full h-full">
      <FileViewer fileType={fileType} filePath={fileUrl} />
    </div>
  );
}
