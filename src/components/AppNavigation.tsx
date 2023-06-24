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
import { invoke } from "@tauri-apps/api/tauri";
import { goToPage as gotoPage } from "@/utils";
import { Fragment, useEffect, useState } from "react";
import { Dialog, Transition } from "@headlessui/react";
import QRCode from "react-qr-code";
import Image from "next/image";
import NavigationTab, { Route } from "./NavigationTab";
import { MemoryInformation } from "./MemoryInformation";
import { SystemInformation } from "@/store/sys-info";
/**
 * @function openFileManager - opens a file manager
 * @returns {Array<Files>} an array of selected files
 */
// const openFileManager = async () => /* : Array<File> */ {
//   try {
//     const selectedFilePath = await open({
//       directory: false,
//       multiple: true,
//       filters: allowedExtension,
//       // defaultPath: await appDir(),
//     });
//     // upload select file with tauri upload plugin
//   } catch (err) {
//     message((err as Error).message, {
//       title: "Access error",
//       type: "error",
//     });
//   }
// };
// the port on which th application urn for the sender PC
interface SenderProps {
  port: number;
}

/**
 * @function QRConnect - display QR code in which URL  for connection is embedded
 * @param url - the core application URl
 * @param serverId, the serverId the application
 * @return UI
 */
function QRConnect({ url }: { url: string }) {
  return (
    <>
      <div
        style={{ background: "white", padding: "4px", height: "260px" }}
        className="flex flex-col items-center rounded"
      >
        <QRCode
          fgColor={"#1e293b"}
          value={url.trim()}
          style={{ boxSizing: "border-box", maxWidth: "150px" }}
        />
      </div>
    </>
  );
}

function SendFileComponent() {
  return (
    <form action="" style={{ background: "", padding: "4px", height: "260px" }}>
      <div className="flex flex-col align-center justify-center">
        <label htmlFor="connectionID " className="text-gray-600 sr-only">
          connection ID
        </label>
        <div className="flex items-center my-4 justify-center mx-auto">
          <Image
            src="/icons/Computer-Tablet-and-Phone-Vectors---1.0.0.svg"
            alt="recieve files"
            className="w-[150px]"
            width={400}
            height={400}
          />
        </div>
        <input
          type="text"
          maxLength={6}
          name="connectionID"
          placeholder="enter connection ID"
          id="connectionID"
          className="border-2 placeholder:text-small my-0 w-2/3 mt-10 mx-auto border-gray-300 rounded-md p-2"
        />
      </div>
    </form>
  );
}

//
function ReceiveConfig({
  serverId,
  ipAddress,
}: {
  serverId: number;
  ipAddress: string;
}) {
  return (
    <div>
      <div>
        <QRConnect url={`http://${ipAddress.trim()}:${serverId}/`} />
      </div>
      <div className=" font-mono my-2 text-center text-gray-600">
        <strong>ID:</strong>
        {serverId}
      </div>
    </div>
  );
}

export default function AppNavigation() {
  let [isModalOpen, setModalState] = useState(false);
  let [systemInformation, setSystemInformation] = useState(
    {} as SystemInformation
  );

  useEffect(() => {
    // fetch sys information from app core
    invoke("get_system_information").then((sysInfo) => {
      setSystemInformation((sysInfo as any).data);
    });
  }, []);

  const closeModal = () => setModalState(false);
  const openModal = () => setModalState(true);

  let [showSendConfig, setSendConfig] = useState(false);
  let [showReceiveConfig, setReceiveConfig] = useState(true);

  const showSendComponent = () => {
    setSendConfig(true);
    setReceiveConfig(false);
  };
  const showReceiveComponent = () => {
    setReceiveConfig(true);
    setSendConfig(false);
  };

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
      // action: openFileManager,
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
      <Transition appear show={isModalOpen} as={Fragment}>
        <Dialog as="div" className="relative z-10" onClose={closeModal}>
          <Transition.Child
            as={Fragment}
            enter="ease-out duration-300"
            enterFrom="opacity-0"
            enterTo="opacity-100"
            leave="ease-in duration-200"
            leaveFrom="opacity-100"
            leaveTo="opacity-0"
          >
            <div className="fixed inset-0 bg-black bg-opacity-50" />
          </Transition.Child>

          <div className="fixed inset-0 overflow-y-auto py-10">
            <div className="flex min-h-full items-center justify-center p-4 text-center">
              <Transition.Child
                as={Fragment}
                enter="ease-out duration-300"
                enterFrom="opacity-0 scale-95"
                enterTo="opacity-100 scale-100"
                leave="ease-in duration-200"
                leaveFrom="opacity-100 scale-100"
                leaveTo="opacity-0 scale-95"
              >
                <Dialog.Panel className="w-full dark:bg-gray-200  max-w-md transform overflow-hidden rounded-2xl bg-white p-6 text-left align-middle shadow-xl transition-all mb-8">
                  <Dialog.Title
                    as="h3"
                    className="text-sm text-gray-500 text-center"
                  >
                    {showSendConfig &&
                      "Input the connection ID below in the recipient computer"}
                    {showReceiveConfig &&
                      "Scan QR Code or provide Connection ID"}
                  </Dialog.Title>
                  <div className="mt-6 ">
                    {showSendConfig && <SendFileComponent />}
                    {showReceiveConfig && (
                      <ReceiveConfig
                        serverId={systemInformation.port}
                        ipAddress={systemInformation.ipAddress}
                      />
                    )}
                    <div className="text-sm flex justify-center gap-5 text-gray-500 mt-6">
                      <button
                        type="button"
                        className={
                          showReceiveConfig
                            ? "inline-flex justify-center rounded-md   px-4 py-2 text-sm font-medium border bg-gray-600 text-gray-100 border-mirage-500"
                            : "inline-flex justify-center rounded-md   px-4 py-2 text-sm font-medium border border-mirage-500"
                        }
                        onClick={showReceiveComponent}
                      >
                        Send files
                      </button>

                      <button
                        type="button"
                        className={
                          showSendConfig
                            ? "inline-flex justify-center rounded-md   px-4 py-2 text-sm font-medium border bg-gray-600 text-gray-100 border-mirage-500"
                            : "inline-flex justify-center rounded-md   px-4 py-2 text-sm font-medium border border-mirage-500"
                        }
                        onClick={showSendComponent}
                      >
                        Recieve files
                      </button>
                    </div>
                  </div>
                </Dialog.Panel>
              </Transition.Child>
            </div>
          </div>
        </Dialog>
      </Transition>

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
            />
          ))}
        </div>

        <MemoryInformation
          systemName={systemInformation.systemName}
          freeMemory={systemInformation.freeMemory}
        />
      </nav>
    </>
  );
}
