import {
  ArrowDownLeftIcon,
  ArrowUpRightIcon,
  ClockIcon,
  Cog6ToothIcon,
} from "@heroicons/react/24/outline";
import Button from "../Button";
import Text from "../Text";
import SmallText from "../SmallText";
interface Props {
  children: React.ReactNode;
}

export default function MobileAppLayout({ children }: Props) {
  return (
    <div className="relative h-[100vh] overflow-y-scroll">
      <div className="px-6 pt-3">{children}</div>

      <footer className="flex  align-center justify-between bg-white fixed px-4  mb-0 pb-0 rounded-t-md z-50 w-full bottom-0 left-0 shadow-sm shadow-gray-200">
        <Button className=" text-gray-400 hover:bg-app-100 transition-all  rounded-lg p-4 cursor-pointer flex flex-col items-center justify-center">
          <ArrowUpRightIcon className="w-6 h-6 mb-1" />
          <SmallText>Send</SmallText>
        </Button>

        <Button className="text-gray-400 hover:text-app-400 transition-all  rounded-lg p-4 cursor-pointer flex  flex-col items-center justify-center">
          <ArrowDownLeftIcon className="w-6 h-6 mb-1" />
          <SmallText>Recieve</SmallText>
        </Button>

        <Button className="text-gray-400 hover:text-app-400 transition-all  rounded-lg p-4 cursor-pointer flex  flex-col items-center justify-center">
          <ClockIcon className="w-6 h-6 mb-1" />
          <SmallText>History</SmallText>
        </Button>

        <Button className="text-gray-400 hover:bg-app-100 transition-all  rounded-lg p-4 cursor-pointer flex flex-col items-center justify-center">
          <Cog6ToothIcon className="w-6 h-6 mb-1" />
          <SmallText>Settings</SmallText>
        </Button>
      </footer>
    </div>
  );
}
