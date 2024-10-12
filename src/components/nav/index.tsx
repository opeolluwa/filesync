"is client";
import { SystemInformationContext } from "@/store/system_information";
import { WifiStatusContext } from "@/store/wifi-status";
import { useContext } from "react";
import { MemoryInformation } from "../MemoryInformation";
import NavigationTab from "./NavItem";
import { desktopRoutes } from "./routes";
import React from "react";

export default function Navigation() {
  const { data: isConnectedToWifi } = useContext(WifiStatusContext);
  const { availableDisk, usedDisk, systemName } = useContext(
    SystemInformationContext
  );

  return (
    <>
      <nav
        className="hidden sm:block sm:col-span-3 lg:col-span-3 bg-[rgba(249,250,254,255)]   px-[1px]   text-gray-600  pt-10"
        style={{
          height: "calc(100vh-200px)",
          overflowY: "hidden",
          position: "relative",
        }}
      >
        <div className="flex flex-col px-2 lg:px-4 lg:pl-6">
          {desktopRoutes.map((route, index) => (
            <NavigationTab
              key={index}
              icon={route.icon}
              name={route.name}
              action={route.action}
              alternateIcon={route.alternateIcon}
              path={route.path}
              disabled={Boolean(isConnectedToWifi) === false}
            />
          ))}
        </div>

        <MemoryInformation />
      </nav>
    </>
  );
}

export { MobileRoutes, desktopRoutes as DesktopRoutes } from "./routes";
