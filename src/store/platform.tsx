import { ReactNode, createContext, useEffect, useState } from "react";

import { invoke } from "@tauri-apps/api/core";
import { CommandData } from "tauri/bindings/CommandData";
import { platform } from "os";

export enum DevicePlatformInterface {
  ANDROID = "android",
  LINUX = "linux",
  IOS = "ios",
  WINDOWS = "windows",
  MAC = "mac"
}

export interface DevicePlatform {
    device: DevicePlatformInterface
}

export const DevicePlatformContext = createContext({} as DevicePlatform);

export default function Platform({ children }: { children: ReactNode }) {
// const devicePlatform = DevicePlatformInterface.MAC;
const devicePlatform = DevicePlatformInterface.ANDROID;


  return (
    <DevicePlatformContext.Provider
      value={{
        device: devicePlatform
      }}
    >
      {children}
    </DevicePlatformContext.Provider>
  );
}
