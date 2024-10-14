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
import React from "react";

export interface Route {
  icon: React.ReactNode; // the route icon
  name: string; // the route name
  alternateIcon?: React.ReactNode; // the icon to show on hover or active state
  action?: () => any; // action that will be executed when the route is clicked
  path: string; // the path string
  isActive?: any;
  disabled?: boolean;
}

export const desktopRoutes: Route[] = [
  {
    path: "/home",
    icon: <HomeIcon className="w-6 h-6" />,
    name: "home",
    alternateIcon: <SolidHomeIcon className="w-6 h-6" />,
  },
  {
    icon: <QrCodeIcon className="w-6 h-6" />,
    name: "Connect Device",
    alternateIcon: <QrCodeIcon className="w-6 h-6" />,
    path: "/",
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

// export const MobileRoutes = [
//   { name: "History", path: "/history" },
//   { name: "Home", path: "/home" },
//   { name: "Video", path: "/video" },
//   { name: "Audio", path: "/audio" },
//   { name: "Pictures", path: "/audio" },
//   { name: "Zipped", path: "/zip" },
//   { name: "Big Files", path: "/big-files" },
//   { name: "Storage", path: "/related" },
// ];

export const MobileRoutes = [
  { name: "Home", path: "/history" },
  { name: "Share", path: "/home" },
  { name: "History", path: "/video" },
  { name: "profile", path: "/audio" },
];
