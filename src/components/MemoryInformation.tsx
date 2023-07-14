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
  let _freeMem = usedMemory?.split(" ")[0].trim() as unknown as number; // [10, GB]
  let _totMem = totalMemory?.split(" ")[0].trim() as unknown as number; // [106, GB]

  const usedMem = Math.round((1 - _totMem / _freeMem) * 100); //
  const memBarWidth = usedMem + "%";

  return (
    <div
      style={{
        position: "absolute",
        width: "100%",
        left: 0,
        bottom: "45px",
      }}
      className="hidden lg:block"
    >
      <div className="flex justify-between mb-2 px-4">
        {
          <span className=" font-medium text-blue-700 text-sm capitalize ">
            {systemName}
          </span>
        }

        <span className=" font-medium text-blue-700 text-sm">
          {totalMemory} of {usedMemory}
        </span>
      </div>
      <div className="w-fill bg-gray-200 rounded-md mx-4 h-2">
        <div
          className="bg-app-400 h-1.5 rounded-full"
          style={{ width: memBarWidth }}
        ></div>
      </div>
    </div>
  );
};
