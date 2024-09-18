import { useContext, useState } from "react";
import DesktopAppLayout from "../layout/DesktopAppLayout";
import MobileAppLayout from "../layout/MobileAppLayout";
import { DeviceInformationContext } from "@/store/device";

interface Props {
  children: React.ReactNode;
}

export default function Layout({ children }: Props) {
  const { osType } = useContext(DeviceInformationContext);
  const isMobile = osType === "Android";

  if (isMobile) {
    return <MobileAppLayout children={children} />;
  }

  return <DesktopAppLayout children={children} />;
}
