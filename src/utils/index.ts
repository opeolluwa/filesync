
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
    }
    else if (size > 1024 * 1024 * 1024) {
        return (size / (1024 * 1024 * 1024)).toFixed(1).toString() + " GB";
    } else if (size > 1024 * 1024) {
        return (size / (1024 * 1024)).toFixed(1).toString() + " MB";
    } else if (size > 1024) {
        return (size / 1024).toFixed(1).toString() + " KB";
    } else {
        return size.toString() + " B";
    }
}

// go bak to prev page
export function goToPrevPage() {
    window.history.back()

}
// navigate to the next page
export function goToNextPage() {
    window.history.forward()

}

/**
 * @function gotoPage - to to the specified page
 * @param routePath a string of the route path relative to the index route 
 */

export interface AppRouterInterface {
    routePath: string
}
export function goToPage({ routePath }: any) {
    if ( typeof window !== 'undefined') {
        // browser code
        window.location.href = routePath;
    }

}


// allowed file extension
export const allowedExtension: DialogFilter[] = [
  {
    name: "image",
    extensions: [
      "ai",
      "dxf",
      "odg",
      "fodg",
      "svg",
      "svgz",
      "bmp",
      "gif",
      "ico",
      "jpg",
      "jpeg",
      "png",
      "psd",
      "pdd",
      "tga",
      "tiff",
      "xcf",
      "xpm",
    ],
  },
  {
    name: "audio",
    extensions: [
      "au",
      "aif",
      "aifc",
      "aiff",
      "wav",
      "flac",
      "la",
      "pac",
      "m4a",
      "ape",
      "wv",
      "wma",
      "ast",
      "mp2",
      "mp3",
      "spx",
      "aac",
      "mpc",
      "ra",
      "ogg",
      "mid",
      "m3u",
      "pls",
    ],
  },
  { name: "pdf", extensions: ["pdf", "ps"] },
  {
    name: "video",
    extensions: [
      "webm",
      "mkv",
      "flv",
      "vob",
      "ogv",
      "drc",
      "avi",
      "mov",
      "qt",
      "wmv",
      "rm",
      "rmvb",
      "asf",
      "mp4",
      "m4p",
      "m4v",
      "mpg",
      "mpeg",
      "mpe",
      "mpv",
      "3gp",
      "3g2",
      "mxf",
      "aff",
      "m2ts",
      "mts",
    ],
  },
  {
    name: "powerpoint",
    extensions: [
      "ppt",
      "pot",
      "pps",
      "pptx",
      "pptm",
      "potx",
      "potm",
      "ppam",
      "ppsx",
      "ppsm",
      "sldx",
      "sldm",
      "odp",
      "fodp",
      "otp",
    ],
  },
  {
    name: "word",
    extensions: [
      "doc",
      "dot",
      "docx",
      "docm",
      "dotx",
      "dotm",
      "docb",
      "odt",
      "fodt",
      "ott",
    ],
  },
  {
    name: "excel",
    extensions: [
      "xls",
      "xlt",
      "xlm",
      "xlsx",
      "xlsm",
      "xltx",
      "xltm",
      "xla",
      "xlam",
      "ods",
      "fods",
      "ots",
    ],
  },
  { name: "xml", extensions: ["xml", "xslt", "html", "xhtml", "htm"] },
  {
    name: "delimited",
    extensions: ["csv"],
  },
  {
    name: "document",
    extensions: [
      "txt",
      "rtf",
      "c",
      "h",
      "cpp",
      "hpp",
      "cxx",
      "hxx",
      "java",
      "js",
      "rb",
      "py",
      "cs",
      "m",
      "sh",
      "php",
      "css",
      "go",
    ],
  },
];