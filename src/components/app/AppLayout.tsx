"use client";
import { useContext, useState } from "react";
import DesktopAppLayout from "../layout/desktop/DesktopAppLayout";
import MobileAppLayout from "../layout/mobile/MobileAppLayout";
import { DeviceInformationContext } from "@/store/device";

interface Props {
  children: React.ReactNode;
}

export default function Layout({ children }: Props) {
  const { osType } = useContext(DeviceInformationContext);
  const isMobile = osType === "Android";

  if (isMobile) {
    return <MobileAppLayout>{children}</MobileAppLayout>;
  }

  return <DesktopAppLayout> {children}</DesktopAppLayout>;
}
