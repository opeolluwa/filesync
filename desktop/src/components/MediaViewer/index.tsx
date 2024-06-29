import React from "react";
import { FileWindow } from "@opeolluwa/filewindow";
export default function MediaViewer() {
  return (
    <FileWindow
      fileName={"some file name "}
      fileType={"some file type"}
    ></FileWindow>
  );
}
