"use client";

import { useState } from "react";
import MobileApp from "./home/MobileApp";
import DesktopApp from "./home/DesktopApp";

export default function Home() {
  const [isMobile, setIsMobile] = useState(true);

  if (isMobile) {
    return <MobileApp />;
  }

  return <DesktopApp />;
}
