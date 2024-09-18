"use client";

import { useContext, useState } from "react";
import MobileApp from "./home/MobileApp";
import DesktopApp from "./home/DesktopApp";
import { DeviceInformationContext } from "@/store/device";

export default function Home() {
  const { osType } = useContext(DeviceInformationContext);
  const isMobile = osType === "Android";

  if (isMobile) {
    return <MobileApp />;
  }

  return <DesktopApp />;
}
