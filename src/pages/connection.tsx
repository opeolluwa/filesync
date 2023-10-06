import PageTitle from "@/components/PageTitle";
import Heading from "@/components/Heading";
import Text from "@/components/Text";
import PageLayout from "@/components/layout/PageLayout";
import { Switch, Modal, Button } from "antd";
import { useState } from "react";
export default function ConnectionPage() {
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [networkName, setNetworkName] = useState("");
  const [passkey, setPasskey] = useState("");

  const handleOk = () => {
    setIsModalOpen(false);
  };

  const handleCancel = () => {
    setIsModalOpen(false);
  };

  const availableNetwork: Array<string> = [
    "network 1",
    "network 2",
    "network 3",
    "network 4",
    "network 5",
  ];
  return (
    <>
      <PageLayout pageTitle={"Connect Device"} includeSearchBar={false}>
        <>
          <div className="dark:bg-dark-900 flex justify-between items-center py-4 rounded-lg px-4">
            <div className="flex flex-col gap-4">
              <Heading context={"Turn on hotspot"} />
              <Text
                context={
                  " Creating hotspot allow you to share files with peer devices!"
                }
                withStyle="-mt-3 leading-6"
              />
            </div>
            <Switch onChange={() => setIsModalOpen(true)} />
          </div>

          <Heading context="Available Network" withStyle="mt-12 mb-2" />
          <div className="dark:bg-dark-900 p-4 rounded-lg">
            {availableNetwork.map((network, index) => (
              <div
                key={index}
                className="flex justify-between items-center py-4 rounded-lg px-4"
              >
                <div className="flex flex-col gap-4">
                  <Text context={network} withStyle="font-medium" />
                </div>
                <button className="bg-app rounded-md px-4 py-1 text-gray-100 dark:text-dark-100 border-none hover:text-dark-600">
                  Connect
                </button>
              </div>
            ))}
          </div>
        </>
      </PageLayout>

      <Modal
        title="Create Wi-Fi HotSpot?"
        centered
        width={400}
        open={isModalOpen}
        onOk={handleOk}
        onCancel={handleCancel}
        footer={[
          <Button
            key="ok"
            onClick={handleCancel}
            className="bg-app text-gray-100 dark:text-dark-100"
          >
            Ok
          </Button>,
          <Button
            key="Cancel"
            onClick={handleCancel}
            className="bg-gray-100 text-gray-500"
          >
            Cancel
          </Button>,
        ]}
      >
        <Text
          context={
            "WiFi HotSpot makes your device discoverable to peer devices and allows the transfer of files"
          }
        />
        <div className="flex flex-col gap-8 my-8 ">
          <div>
            <Heading context={"Network Name"} />
            <input
              className="px-4 py-3 rounded-md block w-full border-gray-400 dark:border-transparent bg-gray-100 dark:bg-dark-900 hover:border-none  border-none dark:text-gray-400 small "
              type="text"
              value={networkName}
              onChange={(e) => setNetworkName(e.target.value)}
              placeholder="ssid"
            />
          </div>
          <div>
            <Heading context={"Password"} />
            <input
              className="px-4 py-3 rounded-md block w-full border-gray-400 dark:border-transparent bg-gray-100 dark:bg-dark-900 hover:border-none  border-none dark:text-gray-400 small text-gray-200 "
              type="text"
              value={passkey}
              onChange={(e) => setPasskey(e.target.value)}
              placeholder="password"
            />
          </div>
        </div>
      </Modal>
    </>
  );
}
