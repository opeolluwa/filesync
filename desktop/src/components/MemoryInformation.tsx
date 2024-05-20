import { WifiStatusContext } from "@/store/wifi-status";
import { useContext } from "react";

// use this to display the available memory
export const MemoryInformation = ({
  systemName,
  usedMemory,
  totalMemory,
}: {
  systemName: string;
  usedMemory: string;
  totalMemory: string;
}) => {
  const { data: isConnectedToWifi } = useContext(WifiStatusContext);

  const freeMemory =
    Number(totalMemory?.split(" ")[0]) - Number(usedMemory?.split(" ")[0]);
  const memBarWidth = "56%";
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
         {totalMemory} free space
        </span>
      </div>
      <div className="w-fill bg-gray-200 rounded-md mx-4 h-2">
        <div
          className={
            isConnectedToWifi
              ? "bg-app-400 h-2 rounded-full"
              : "bg-gray-400 h-2 rounded-full"
          }
          style={{ width: memBarWidth }}
        ></div>
      </div>
    </div>
  );
};
