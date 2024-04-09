"use client";

/// give preview for differet file type
import FileViewer from "react-file-viewer";

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
  return (
    <FileViewer
      fileType={fileType}
      filePath={
        "https://ik.imagekit.io/nethbooks/FunaabPayInvoice_80118121__4jRm5vp7L.pdf?updatedAt=1712070984568"
      }
    />
  );
}
