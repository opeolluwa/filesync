/**
 *
 * @function computeFileSize - compute file size to human readable format
 * @param size - file size in byte
 * @returns file size and extension e.g 3.5 MB
 */

import { DialogFilter } from "@tauri-apps/api/dialog";

export function computeFileSize(size: number) {
  if (size > 1024 * 1024 * 1024 * 1024) {
    return (size / (1024 * 1024 * 1024 * 1024)).toFixed(1).toString() + " TB";
  } else if (size > 1024 * 1024 * 1024) {
    return (size / (1024 * 1024 * 1024)).toFixed(1).toString() + " GB";
  } else if (size > 1024 * 1024) {
    return (size / (1024 * 1024)).toFixed(1).toString() + " MB";
  } else if (size > 1024) {
    return (size / 1024).toFixed(1).toString() + " KB";
  } else {
    return size.toString() + " B";
  }
}
