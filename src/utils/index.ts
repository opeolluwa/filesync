import { getStoredData, storeData } from "./store";
export { computeFileSize } from "./file-size";
export { goToNextPage, goToPage, goToPrevPage } from "./navigation";
export const isClient = typeof window !== "undefined";
