import { computeFileSize } from "@/utils";

import {
  ArrowDownCircleIcon,
  ArrowUpCircleIcon,
} from "@heroicons/react/24/outline";
import Identicon from "react-identicons";
import { TransferHistory } from "../../../core/bindings/TransferHistory";

// the required data to render the file card component
// the data will be passed dynamically

enum FileTransferType {
  SENT = "sent",
  RECEIVED = "received",
}

interface Props extends TransferHistory {
}
// the component
export function FileHistory({
  fileName,
  fileSize,
  transactionType,
  date,
  recipient,
  id,
}: Props) {
  return (
    <div className="flex justify-between items-center my-8 flex-wrap bg-[#edeffb]  border-gray-900  p-3 rounded-lg shadow-md shadow-gray-300 cursor-pointer dark:shadow-none hover:shadow-sm hover:shadow-gray-400 transition-shadow ease-in-out">
      <div className="pr-[10px] rounded-full">
        <Identicon
          string={id}
          size={50}
          padding={20}
        />
      </div>
      <div className="flex flex-col text-ellipsis">
        <h5
          className="font-medium text-gray-500 truncate overflow-clip text-ellipsis"
          style={{ width: "180px" }}
        >
          {fileName}
        </h5>
        <div className="flex gap-3 mt[1.5px] text-gray-400  italic text-xs height={30} 
                width={30} ">
          <span>{computeFileSize(Number(fileSize))}</span>{" "}
          <span>{`  ${recipient}`}</span>
        </div>
      </div>

      <div className="hidden lg:block ml-4">
        {transactionType == FileTransferType.SENT
          ? <ArrowUpCircleIcon className="w-8 h-8 text-gray-400 " />
          : <ArrowDownCircleIcon className="w-8 h-8 text-gray-400 " />}
      </div>
    </div>
  );
}
