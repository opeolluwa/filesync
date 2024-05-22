"is client";
import {
  ClockIcon,
  Cog8ToothIcon,
  FolderArrowDownIcon,
  HomeIcon,
  InformationCircleIcon,
  QrCodeIcon,
  ShareIcon,
} from "@heroicons/react/24/outline";
import {
  ClockIcon as SolidClockIcon,
  Cog8ToothIcon as SolidCog8ToothIcon,
  HomeIcon as SolidHomeIcon,
  InformationCircleIcon as SolidInformationIcon,
  ShareIcon as SolidShareIcon, // QrCodeIcon as SolidQrCodeIcon,
  FolderArrowDownIcon as SolidFolderArrowDownIcon,
} from "@heroicons/react/24/solid";
import NavigationTab, { Route } from "./NavItem";
import { useContext } from "react";
import { SystemInformationContext } from "@/store/sys-info";
import { MemoryInformation } from "../MemoryInformation";
import { WifiStatusContext } from "@/store/wifi-status";

export default function Navigation() {
  const { data: isConnectedToWifi } = useContext(WifiStatusContext);
  const { availableDisk, usedDisk, systemName } = useContext(
    SystemInformationContext
  );

  const routes: Route[] = [
    {
      path: "/",
      icon: <HomeIcon className="w-6 h-6" />,
      name: "home",
      alternateIcon: <SolidHomeIcon className="w-6 h-6" />,
    },
    {
      icon: <QrCodeIcon className="w-6 h-6" />,
      name: "Connect Device",
      alternateIcon: <QrCodeIcon className="w-6 h-6" />,
      path: "/connection",
    },
    {
      path: "/share",
      icon: <ShareIcon className="w-6 h-6" />,
      name: "Share files",
      alternateIcon: <SolidShareIcon className="w-6 h-6" />,
    },

    {
      path: "/received",
      icon: <FolderArrowDownIcon className="w-6 h-6" />,
      name: "Received files",
      alternateIcon: <SolidFolderArrowDownIcon className="w-6 h-6" />,
    },
    {
      path: "/history",
      icon: <ClockIcon className="w-6 h-6" />,
      name: "Transfer History",
      alternateIcon: <SolidClockIcon className="w-6 h-6" />,
    },
    {
      path: "/settings",
      icon: <Cog8ToothIcon className="w-6 h-6" />,
      alternateIcon: <SolidCog8ToothIcon className="w-6 h-6" />,
      name: "settings",
    },
    {
      path: "/about",
      icon: <InformationCircleIcon className="w-6 h-6" />,
      alternateIcon: <SolidInformationIcon className="w-6 h-6" />,
      name: "About",
    },
  ];

  return (
    <>
      <nav
        className="col-span-3 lg:col-span-3 bg-[rgba(249,250,254,255)]   px-[1px]   text-gray-600  pt-10"
        style={{
          height: "calc(100vh-200px)",
          overflowY: "hidden",
          position: "relative",
        }}
      >
        <div className="flex flex-col px-2 lg:px-4 lg:pl-6">
          {routes.map((route, index) => (
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

        <MemoryInformation usedDisk={usedDisk} availableDisk={availableDisk} />
      </nav>
    </>
  );
}
