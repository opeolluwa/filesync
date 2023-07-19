import { invoke } from "@tauri-apps/api/tauri";
import { getStoredData, storeData } from "./store";
export { computeFileSize } from "./file-size";
export { goToNextPage, goToPage, goToPrevPage } from "./navigation";
export const isClient = typeof window !== "undefined";

// send file to server
export async function shareFile(filePath: string) {
  console.log("sharing music files");
  const ipAddr =
    isClient &&
    (await invoke("get_ip_address").catch((err) => {
      console.log("error getting ip addr due to ", (err as Error).message);
    }));
  const uploadPath = `${ipAddr}/upload`;
  console.log("the upload path is ", uploadPath);
}
