import { WifiStatusContext } from "@/store/network";
import { computeFileSize } from "@/utils";
import { useContext, useEffect, useState } from "react";
import { allSysInfo, memoryInfo } from "tauri-plugin-system-info-api";

// use this to display the available memory
export const MemoryInformation = () => {
  const [totalMemory, setTotalMemory] = useState(0);
  const [usedMemory, setUsedMemory] = useState(0);

  const { data: isConnectedToWifi } = useContext(WifiStatusContext);

  useEffect(() => {
    const fetchData = async () => {
      const data = await memoryInfo();
      setTotalMemory(data.total_memory);
      setUsedMemory(data.used_memory);
    };
    fetchData();
  }, []);

  const freeMemory = totalMemory - usedMemory;
  const memoryBarWidth = Math.round((usedMemory / totalMemory) * 100);
  const availableDisk = computeFileSize(freeMemory);
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
