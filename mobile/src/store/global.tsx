import { ReactNode, createContext, useEffect, useState } from "react";
import { SystemInformation as CoreSysInfo } from "@filesync/types/SystemInformation";
import { CapacitorHttp, HttpResponse } from "@capacitor/core";

export interface SystemInformation extends CoreSysInfo {
  usedDisk: string;

  availableDisk: string;
  /// the current user name eg - drizzle
  systemName: string;
  /// available store
  freeMemory: string;
  /// the port on which the core server runs
  port: bigint;
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

  const BASE_URL = "http://192.168.0.170:18005/api/sys-info";
  // request options =
  const options = {
    url: BASE_URL,
    headers: { "X-Fake-Header": "Fake-Value" }, //TODO: impl client without header
  };

  useEffect(() => {
    // fetch sys information from app core
    CapacitorHttp.get(options).then((response: HttpResponse) => {
      const data = response.data as unknown as SystemInformation;
      setSystemInformation(data);
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
        disk: systemInformation.disk,
        remainingTime: systemInformation.remainingTime,
      }}
    >
      {children}
    </SystemInformationContext.Provider>
  );
}
