// render the folder icon
import archiveIcon from "./assets/common/archived.png";
import audioIcon from "./assets/common/audio.png";
import csvIcon from "./assets/common/csv.png";
import defaultIcon from "./assets/common/default.png";
import documentIcon from "./assets/common/document.png";
import imageIcon from "./assets/common/image.png";
import pdfIcon from "./assets/common/pdf.png";
import presentationIcon from "./assets/common/presentation.png";
import svgIcon from "./assets/common/svg.png";
import textIcon from "./assets/common/text.png";
import videoIcon from "./assets/common/video.png";
import { FileType } from "./file-type";

export function getFileIcon(fileExtension: string) {
  const imageExtensions = [
    "jpg",
    "jpeg",
    "png",
    "gif",
    "bmp",
    "tiff",
    "raw",
    "svg",
    "ai",
    "eps",
    "psd",
    "xcf",
    "ico",
    "webp",
    "jxr",
    "hdr",
    "tif",
    "exif",
    "pgm",
    "ppm",
    "pbm",
    "pnm",
    "heic",
    "heif",
    "dng",
    "cr2",
    "nef",
    "arw",
    "orf",
    "rw2",
    "sr2",
    "raf",
    "mrw",
    "pef",
    "x3f",
    "3fr",
    "kdc",
    "srw",
    "nrw",
    "rwz",
    "rwl",
    "iiq",
    "rw1",
    "r3d",
    "fff",
    "yuv",
    "cin",
    "dpx",
    "jp2",
    "j2k",
    "jpf",
    "jpx",
    "jpm",
    "mj2",
    "wdp",
    "hdp",
    "dds",
    "pvr",
    "tga",
    "cur",
    "icl",
    "thm",
    "sai",
    "ora",
    "pdn",
    "kra",
    "cpt",
    "pdd",
    "mng",
    "apng",
    "svgz",
    "emf",
    "wmf",
  ];
  const documentExtensions = [
    "doc",
    "docx",
    "rtf",
    "odt",
    "ods",
    "odp",
    "odg",
    "odp",
    "fodp",
    "otp",
    "doc",
    "dot",
    "docx",
    "docm",
    "dotx",
    "dotm",
    "docb",
    "odt",
    "fodt",
  ];
  const svgExtensions = ["svg"];
  const textExtensions = ["txt"];
  const audioExtensions = [
    "3gp",
    "aa",
    "aac",
    "aax",
    "act",
    "aiff",
    "alac",
    "amr",
    "ape",
    "au",
    "awb",
    "dss",
    "dvf",
    "flac",
    "gsm",
    "iklax",
    "ivs",
    "m4a",
    "m4b",
    "m4p",
    "mmf",
    "movpkg",
    "mp3",
    "mpc",
    "msv",
    "nmf",
    "ogg",
    "oga",
    "mogg",
    "opus",
    "ra",
    "rm",
    "raw",
    "rf64",
    "sln",
    "tta",
    "voc",
    "vox",
    "wav",
    "wma",
    "wv",
    "webm",
    "8svx",
    "cda",
  ];
  const pdfExtensions = ["pdf"];
  const csvExtensions = ["csv"];
  const presentationExtensions = [
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
    "thmx",
  ];
  const videoExtensions = [
    "mp4",
    "mkv",
    "webm",
    "flv",
    "vob",
    "ogv",
    "ogg",
    "drc",
    "gif",
    "gifv",
    "mng",
    "avi",
    "MTS",
    "MT2S",
    "TS",
    "mov",
    "qt",
    "wmv",
    "yuv",
    "rm",
    "rmvb",
    "viv",
    "asf",
    "amv",
    "m4p",
    "m4v",
    "mpg",
    "mp2",
    "mpeg",
    "mpe",
    "mpv",
    "m2v",
    "svi",
    "3gp",
    "3g2",
    "mxf",
    "roq",
    "nsv",
    "f4v",
    "f4p",
    "f4a",
    "f4b",
  ];
  const archiveExtensions = ["zip", "rar", "tar", "gz"];
  const extension = fileExtension.toLocaleLowerCase().trim();

  if (imageExtensions.includes(extension)) {
    return { type: FileType.Image, icon: imageIcon };
  } else if (audioExtensions.includes(extension)) {
    return { type: FileType.Audio, icon: audioIcon };
  } else if (pdfExtensions.includes(extension)) {
    return { type: FileType.PDF, icon: pdfIcon };
  } else if (csvExtensions.includes(extension)) {
    return { type: FileType.CSV, icon: csvIcon };
  } else if (presentationExtensions.includes(extension)) {
    return { type: FileType.Presentation, icon: presentationIcon };
  } else if (videoExtensions.includes(extension)) {
    return { type: FileType.Video, icon: videoIcon };
  } else if (archiveExtensions.includes(extension)) {
    return { type: FileType.Archive, icon: archiveIcon };
  } else if (documentExtensions.includes(extension)) {
    return { type: FileType.Document, icon: documentIcon };
  } else if (textExtensions.includes(extension)) {
    return { type: FileType.Text, icon: textIcon };
  } else if (svgExtensions.includes(extension)) {
    return { type: FileType.SVG, icon: svgIcon };
  } else {
    return { type: FileType.Default, icon: defaultIcon };
  }
}
