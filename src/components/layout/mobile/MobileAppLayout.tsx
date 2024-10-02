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
}

export default function MobileAppLayout({ children }: Props) {
  return (
    <div className="relative min-h-screen overflow-y-scroll">{children}</div>
  );
}
