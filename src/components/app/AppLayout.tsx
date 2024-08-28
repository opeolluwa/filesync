import { useState } from "react";
import DesktopAppLayout from "../layout/DesktopAppLayout";
import MobileAppLayout from "../layout/MobileAppLayout";

interface Props {
  children: React.ReactNode;
}

export default function Layout({ children }: Props) {
  const [isMobile, setIsMobile] = useState(true);

  if (isMobile) {
    return <MobileAppLayout children={children} />;
  }

  return <DesktopAppLayout children={children} />;
}
