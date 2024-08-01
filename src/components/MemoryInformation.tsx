import { WifiStatusContext } from "@/store/wifi-status";
import { useContext, useEffect, useState } from "react";

import { memoryInfo, MemoryInfo } from "tauri-plugin-system-info-api";
// use this to display the available memory
export const MemoryInformation = ({
  usedDisk,
  availableDisk,
}: {
  usedDisk: string;
  availableDisk: string;
}) => {
  const [systemMemory, setsystemMemory] = useState("");
  const { data: isConnectedToWifi } = useContext(WifiStatusContext);
  useEffect(() => {


  }, []);


  const usedMemory = Number(usedDisk?.split(" ")[0]);
  const freeMemory = Number(availableDisk?.split(" ")[0]);
  const totalMemory = usedMemory + freeMemory;
  const memoryBarWidth = Math.round((freeMemory / totalMemory) * 100);
  return (
    <div
      style={{
        position: "absolute",
        width: "100%",
        left: 0,
        bottom: "45px",
      }}
    >
      <div className="flex justify-between mb-2 px-4">
        <span className=" font-medium text-blue-700 text-sm">
          {availableDisk} free space
        </span>
      </div>
      <div className="w-fill bg-gray-200 rounded-md mx-4 h-2">
        <div
          className={
            isConnectedToWifi
              ? "bg-app-400 h-2 rounded-full"
              : "bg-gray-400 h-2 rounded-full"
          }
          style={{ width: `${memoryBarWidth}%` }}
        ></div>
      </div>
    </div>
  );
};
