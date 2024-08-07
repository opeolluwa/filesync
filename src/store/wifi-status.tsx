import { ReactNode, createContext, useEffect, useState } from "react";

import { invoke } from "@tauri-apps/api/core";
import { CommandData } from "tauri/bindings/CommandData";

type WifiStatusInterface = CommandData<boolean>;

export const WifiStatusContext = createContext({} as WifiStatusInterface);

export default function WifiStatus({ children }: { children: ReactNode }) {
  const [isConnectedToWifi, setConnectedToWifi] = useState(null);

  useEffect(() => {
    invoke("is_connected_to_wifi").then((res) => {
      setConnectedToWifi(res as any);
    });
  }, []);

  // typecast the wifi response too
  const wifiStatus = isConnectedToWifi as unknown as CommandData<boolean>;

  return (
    <WifiStatusContext.Provider
      value={{
        data: wifiStatus?.data,
        message: wifiStatus?.message,
        status: wifiStatus?.status,
      }}
    >
      {children}
    </WifiStatusContext.Provider>
  );
}
