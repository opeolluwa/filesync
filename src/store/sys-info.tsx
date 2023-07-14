import { invoke } from "@tauri-apps/api/tauri";
import { ReactNode, createContext, useEffect, useState } from "react";
export interface SystemInformation {
  usedDisk: string;
  availableDisk: string;
  /// the current user name eg - drizzle
  systemName: string;
  /// available store
  freeMemory: string;
  /// the port on which the core server runs
  port: number;
  /// the system ip address,ex:  192.168.213.230
  ipAddress: string;
  /// the uptime e.g 2 hours
  uptime: string;
  /// the core server base url
  serverBaseUrl: string;
}

export const SystemInformationContext = createContext({} as SystemInformation);
export default function SystemInfoStore({ children }: { children: ReactNode }) {
  let [systemInformation, setSystemInformation] = useState(
    {} as SystemInformation
  );

  useEffect(() => {
    // fetch sys information from app core
    invoke("get_system_information").then((sysInfo) => {
      setSystemInformation((sysInfo as any).data);
    });
  }, []);

  return (
    <SystemInformationContext.Provider
      value={{
        systemName: systemInformation.systemName,
        freeMemory: systemInformation.freeMemory,
        port: systemInformation.port,
        ipAddress: systemInformation.ipAddress,
        uptime: systemInformation.uptime,
        serverBaseUrl: systemInformation.serverBaseUrl,
        availableDisk: systemInformation.availableDisk,
        usedDisk: systemInformation.usedDisk,
      }}
    >
      {children}
    </SystemInformationContext.Provider>
  );
}
