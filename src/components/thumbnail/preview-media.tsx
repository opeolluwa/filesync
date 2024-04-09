/// give preview for differet file type 
// import Viewer from "viewerjs";
// import Viewer from "viewerjs/dist/viewer.esm";
import "viewerjs/dist/viewer.min.css";

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
  fileType: FileType;
  filePath: string;
}

// export const ImageViewer = new Viewer(
//   document?.getElementById("image-viewer")!,
//   {
//     inline: false,
//     viewed() {
//       ImageViewer.zoomTo(2);
//     },
//   }
// );



// <Image
//   src={`file://${filePath}`}
//   className="flex items-center align-center justify-center p-4"
//   alt={fileName}
//   id="image-viewer"
//   width={400}
//   height={400}
// />;


export default function PreviewMedia({ fileType, filePath }: Props) {
  switch (fileType) {
    case FileType.Archive:
      return (
        <>
          <h1>archive file</h1>
        </>
      );
    case FileType.Audio:
      return (
        <>
          <h1>audio file</h1>
        </>
      );
    case FileType.CSV:
      return (
        <>
          <h1>csv file</h1>
        </>
      );
    case FileType.Document:
      return (
        <>
          <h1>document file</h1>
        </>
      );
    case FileType.Image:
      return (
        <>
          <h1>image file</h1>
        </>
      );
    case FileType.PDF:
      return (
        <>
          <h1>pdf file</h1>
        </>
      );
    case FileType.Presentation:
      return (
        <>
          <h1>presentation file</h1>
        </>
      );
    case FileType.SVG:
      return (
        <>
          <h1>svg file</h1>
        </>
      );
    case FileType.Text:
      return (
        <>
          <h1>text file</h1>
        </>
      );
    case FileType.Video:
      return (
        <>
          <h1>video file</h1>
        </>
      );
    default:
      return (
        <>
          <h1>default file</h1>
        </>
      );
  }

 
}
