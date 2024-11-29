"use client";

import { OsType, type } from "@tauri-apps/plugin-os";
import { useEffect, useState } from "react";
import DesktopAppEntry from "./desktop";
import MobileAppEntry from "./mobile";

export default function AppEntry() {
  const [osType, setOsType] = useState<OsType>();
  const [isMobile, setIsMobile] = useState(false);
  useEffect(() => {
    const fetchData = () => {
      setOsType(type());

      if (osType == "android" || osType == "ios") {
        setIsMobile(true);
      }
    };

    fetchData();
  }, []);

  if (isMobile) {
    return <MobileAppEntry />;
  } else {
    return <DesktopAppEntry />;
  }
}
