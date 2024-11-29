"use client";

import { OsType } from "@tauri-apps/plugin-os";
import { type } from "os";
import { useEffect, useState } from "react";
import DesktopAppEntry from "./desktop";
import MobileAppEntry from "./mobile";

export default function AppEntry() {
  const [osType, setOsType] = useState<OsType>("" as OsType);
  const [isMobile, setIsMobile] = useState(false);
  useEffect(() => {
    const fetchData = () => {
      const os = type();
      setOsType(os);

      if (os == "android" || os == "ios") {
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
