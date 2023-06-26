import { SystemInformation } from "@/store/sys-info";
import SendFile from "./SendFile";
import { Dialog, Transition } from "@headlessui/react";
import React, { Fragment, ReactNode, useEffect, useState } from "react";
import ReceiveFile from "./ReceiveFile";
import { invoke } from "@tauri-apps/api";

export default function ConnectionModal({
  children,
  systemInformation,
  isModalOpen,
  openModal,
  closeModal,
}: {
  children: ReactNode;
  systemInformation: SystemInformation;
  isModalOpen: boolean;
  openModal: () => void;
  closeModal: () => void;
}) {
  // let [isModalOpen, setModalState] = useState(false);

  // const closeModal = () => setModalState(false);
  // const openModal = () => setModalState(true);

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
                    {showSendConfig && <SendFile />}
                    {showReceiveConfig && (
                      <ReceiveFile
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
      {children}
    </>
  );
}
