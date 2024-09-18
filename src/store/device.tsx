// manage all information about the device on which the application runs

import { invoke } from "@tauri-apps/api/core";
import { createContext, ReactNode, useEffect, useState } from "react";
import { Device } from "tauri/device/bindings/Device";

export interface DeviceInformationInterface extends Device {}

export const DeviceInformationContext = createContext(
  {} as DeviceInformationInterface
);

export default function DeviceInformationStore({
  children,
}: {
  children: ReactNode;
}) {
  const [deviceInformation, setDeviceInformation] = useState(
    {} as DeviceInformationInterface
  );

  useEffect(() => {
    invoke("get_device_information").then((result) => {
      setDeviceInformation((result as any).data);
    });
  }, []);

  return (
    <DeviceInformationContext.Provider
      value={{
        osType: deviceInformation.osType,
        memory: deviceInformation.memory,
      }}
    >
      {children}
    </DeviceInformationContext.Provider>
  );
}
