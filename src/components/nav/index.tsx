"is client";
import {
  Cog8ToothIcon,
  HomeIcon,
  FolderOpenIcon,
  InformationCircleIcon,
  ShareIcon,
  ClockIcon,
  SignalIcon,
} from "@heroicons/react/24/outline";
import {
  Cog8ToothIcon as SolidCog8ToothIcon,
  HomeIcon as SolidHomeIcon,
  FolderOpenIcon as SolidFolderIconOpen,
  InformationCircleIcon as SolidInformationIcon,
  ShareIcon as SolidShareIcon,
  SignalIcon as SolidSignalIcon,
  ClockIcon as SolidClockIcon,
} from "@heroicons/react/24/solid";
import { goToPage as gotoPage } from "@/utils";
import ConnectionModal from "./ConnectionModal";
import NavigationTab, { Route } from "./NavigationTab";
import { useEffect, useState } from "react";
import { SystemInformation } from "@/store/sys-info";
import { invoke } from "@tauri-apps/api/tauri";
import { MemoryInformation } from "../MemoryInformation";
import { DialogFilter, message, ask } from "@tauri-apps/api/dialog";
import { open } from "@tauri-apps/api/dialog";
import { pictureDir } from "@tauri-apps/api/path";
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

  let [isModalOpen, setModalState] = useState(false);

  const closeModal = () => setModalState(false);
  const openModal = () => setModalState(true);

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
      name: "Connection",
      alternateIcon: <SolidSignalIcon className="w-6 h-6" />,
      action: openModal,
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
      name: "History",
      alternateIcon: <SolidClockIcon className="w-6 h-6" />,
      action: () => gotoPage({ routePath: "history" }),
    },
    {
      //TODO: open saved files directory
      path: "/files",
      icon: <FolderOpenIcon className="w-6 h-6" />,
      action: openFileManager,
      alternateIcon: <SolidFolderIconOpen className="w-6 h-6" />,
      name: "File Manager",
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
      <ConnectionModal
        systemInformation={systemInformation}
        isModalOpen={isModalOpen}
        openModal={openModal}
        closeModal={closeModal}
      >
        <nav
          className="col-span-1 lg:col-span-2 bg-[rgba(249,250,254,255)]  px-[1px]   text-gray-600  pt-10"
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
            freeMemory={systemInformation.freeMemory}
          />
        </nav>
      </ConnectionModal>
    </>
  );
}
