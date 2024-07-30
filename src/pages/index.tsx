import { useEffect, useState } from "react";
import { DesktopApp } from "./desktop";
import { MobileApp } from "./mobile";
import { platform } from "@tauri-apps/plugin-os";

enum DeviceType {
  Desktop = "desktop",
  Mobile ="mobile"
}

export default function FileSync() {
  const [deviceType, setDeviceType] = useState("");

  // useEffect(async () => {
  //   const currentPlatform = await platform();
  //   if (["android", "ios"].includes(currentPlatform)){
  //     setDeviceType(DeviceType.Mobile)
  //   }
  //   setDeviceType(DeviceType.Desktop)
  // }, []);

  if (true) {
    return <MobileApp />;
  } else {
    return <DesktopApp />;
  }
}
