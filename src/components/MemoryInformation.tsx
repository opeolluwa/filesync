// use this to display the available memory
export const MemoryInformation = ({
  systemName,
  freeMemory,
}: {
  systemName: string;
  freeMemory: string;
}) => {
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
          {freeMemory} of 100GB
        </span>
      </div>
      <div className="w-fill bg-gray-200 rounded-md mx-4 h-2">
        <div
          className="bg-app-400 h-1.5 rounded-full"
          style={{ width: "45%" }}
        ></div>
      </div>
    </div>
  );
};
