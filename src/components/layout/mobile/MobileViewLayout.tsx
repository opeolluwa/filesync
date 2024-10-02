"use client";
import {
  Bars3Icon,
  ClockIcon,
  Cog6ToothIcon,
  FolderIcon,
  HomeIcon,
} from "@heroicons/react/24/outline";
import { QrCodeIcon } from "@heroicons/react/24/solid";
import { Format, scan } from "@tauri-apps/plugin-barcode-scanner";
import Link from "next/link";
import Button from "../../Button";
import SmallText from "../../SmallText";

interface Props {
  children: React.ReactNode;
  includeHeader?: boolean;
  includeFooter?: boolean;
}

export default function MobileViewLayout({
  children,
  includeFooter,
  includeHeader,
}: Props) {
  return (
    <div className="relative min-h-screen overflow-y-scroll">
      {includeHeader && (
        <header className=" mb-4 min-h-12 pt-6 px-4 flex items-center justify-between text-app">
          <Bars3Icon className="w-6 h-6"></Bars3Icon>
          <Link href={"/mobile/scan"}>
            <QrCodeIcon
              className="w-6 h-6"
              onClick={() => scan({ windowed: true, formats: [Format.QRCode] })}
            ></QrCodeIcon>
          </Link>
        </header>
      )}
      <div className="px-4 pt-3">{children}</div>

      {includeFooter && (
        <footer className="flex bg-app rounded-t-3xl border-t border-t-gray-200 align-center justify-between fixed px-4  mb-0 pb-0 z-50 w-full bottom-0 left-0 shadow-sm shadow-gray-200">
          <Button
            href="/mobile"
            className=" text-gray-200 hover:text-white  transition-all  rounded-lg p-4 cursor-pointer flex flex-col items-center justify-center"
          >
            <HomeIcon className="w-6 h-6 mb-1" />
            <SmallText>Home</SmallText>
          </Button>

          <Button
            href="/mobile/files"
            className="text-gray-200 hover:text-white  transition-all  rounded-lg p-4 cursor-pointer flex  flex-col items-center justify-center"
          >
            <FolderIcon className="w-6 h-6 mb-1" />
            <SmallText>Files</SmallText>
          </Button>

          <Button
            href="/mobile/history"
            className="text-gray-200 hover:text-white  transition-all  rounded-lg p-4 cursor-pointer flex  flex-col items-center justify-center"
          >
            <ClockIcon className="w-6 h-6 mb-1" />
            <SmallText>History</SmallText>
          </Button>

          {/* <Button className="text-gray-200 hover:text-white  transition-all  rounded-lg p-4 cursor-pointer flex  flex-col items-center justify-center">
          <QrCodeIcon
            className="w-6 h-6 mb-1"
            onClick={() => scan({ windowed: true, formats: [Format.QRCode] })}
          ></QrCodeIcon>
          <SmallText>Scan</SmallText>
        </Button> */}
          <Button
            href="/mobile/settings/"
            className="text-gray-200 hover:text-white   transition-all  rounded-lg p-4 cursor-pointer flex flex-col items-center justify-center"
          >
            <Cog6ToothIcon className="w-6 h-6 mb-1" />
            <SmallText>Settings</SmallText>
          </Button>
        </footer>
      )}
    </div>
  );
}
