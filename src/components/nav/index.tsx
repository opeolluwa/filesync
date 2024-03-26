"is client";
import {
  ClockIcon,
  Cog8ToothIcon,
  HomeIcon,
  InformationCircleIcon,
  ShareIcon,
  SignalIcon,
} from "@heroicons/react/24/outline";
import {
  ClockIcon as SolidClockIcon,
  Cog8ToothIcon as SolidCog8ToothIcon,
  FolderOpenIcon as SolidFolderIconOpen,
  HomeIcon as SolidHomeIcon,
  InformationCircleIcon as SolidInformationIcon,
  ShareIcon as SolidShareIcon,
  SignalIcon as SolidSignalIcon,
} from "@heroicons/react/24/solid";
import { goToPage as gotoPage } from "@/utils";
import NavigationTab, { Route } from "./NavigationTab";
import { useEffect, useState } from "react";
import { SystemInformation } from "@/store/sys-info";
import { invoke } from "@tauri-apps/api/tauri";
import { MemoryInformation } from "../MemoryInformation";
import { message } from "@tauri-apps/api/dialog";
import { open } from "@tauri-apps/api/dialog";
export default function Navigation() {
  /**
   * @function openFileManager - opens a file manager
   * @returns {Array<Files>} an array of selected files
   */
  const openFileManager = async () => /* : Array<File> */ {
    try {
      const selectedFilePath = await open({
        directory: false,
        multiple: true,
        // filters: allowedExtension,
        // defaultPath: await pictureDir(),
      });
      // upload select file with tauri upload plugin
    } catch (err) {
      message((err as Error).message, {
        title: "Access error",
        type: "error",
      });
    }
  };
  let [systemInformation, setSystemInformation] = useState(
    {} as SystemInformation
  );

  useEffect(() => {
    // fetch sys information from app core
    invoke("get_system_information").then((sysInfo) => {
      setSystemInformation((sysInfo as any).data);
    });
  }, []);

  const routes: Route[] = [
    {
      path: "/",
      icon: <HomeIcon className="w-6 h-6" />,
      name: "home",
      alternateIcon: <SolidHomeIcon className="w-6 h-6" />,
      action: () => gotoPage({ routePath: "/" }),
    },
    {
      icon: <SignalIcon className="w-6 h-6" />,
      name: "Connect Device",
      alternateIcon: <SolidSignalIcon className="w-6 h-6" />,
      action: () => gotoPage({ routePath: "/connection" }),
      path: "/connection",
    },
    {
      path: "/share",
      icon: <ShareIcon className="w-6 h-6" />,
      name: "Share files",
      alternateIcon: <SolidShareIcon className="w-6 h-6" />,
      action: () => gotoPage({ routePath: "share" }),
    },
    {
      path: "/history",
      icon: <ClockIcon className="w-6 h-6" />,
      name: "Transfer History",
      alternateIcon: <SolidClockIcon className="w-6 h-6" />,
      action: () => gotoPage({ routePath: "history" }),
    },

    {
      path: "/settings",
      icon: <Cog8ToothIcon className="w-6 h-6" />,
      alternateIcon: <SolidCog8ToothIcon className="w-6 h-6" />,
      action: () => gotoPage({ routePath: "settings" }),
      name: "settings",
    },
    {
      path: "/help",
      icon: <InformationCircleIcon className="w-6 h-6" />,
      alternateIcon: <SolidInformationIcon className="w-6 h-6" />,
      action: () => gotoPage({ routePath: "help" }),
      name: "help",
    },
  ];

  return (
    <>
      <nav
        className="col-span-3 lg:col-span-3 bg-[rgba(249,250,254,255)] dark:bg-dark-900  px-[1px]   text-gray-600  pt-10"
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
            />
          ))}
        </div>

        <MemoryInformation
          systemName={systemInformation.systemName}
          usedMemory={systemInformation.usedDisk}
          totalMemory={systemInformation.availableDisk}
        />
      </nav>
    </>
  );
}
