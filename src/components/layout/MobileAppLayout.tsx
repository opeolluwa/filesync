import {
  BarsArrowDownIcon,
  BarsArrowUpIcon,
  ClockIcon,
  Cog6ToothIcon
} from "@heroicons/react/24/outline";
import { QrCodeIcon } from "@heroicons/react/24/solid";
import Button from "../Button";
import SmallText from "../SmallText";
import View from "../View";
import { MobileRoutes } from "../nav";
interface Props {
  children: React.ReactNode;
}

export default function MobileAppLayout({ children }: Props) {
  return (
    <div className="relative min-h-screen overflow-y-scroll">
      <header className="bg-app-500 min-h-24 pt-6 px-4 ">
        <View className="flex items-center justify-end">
          <QrCodeIcon className="w-6 h-6 text-white"></QrCodeIcon>
        </View>
        <ul className="flex overflow-x-scroll justify-between gap-x-4 mt-6 text-white">
          {MobileRoutes.map((route) => (
            <li>{route.name}</li>
          ))}
        </ul>
      </header>
      <div className="px-4 pt-3">{children}</div>

      <footer className="flex border-t border-t-gray-500 align-center justify-between fixed px-4  mb-0 pb-0 rounded-t-md z-50 w-full bottom-0 left-0 shadow-sm shadow-gray-200">
        <Button className=" text-gray-400  transition-all  rounded-lg p-4 cursor-pointer flex flex-col items-center justify-center">
          <BarsArrowUpIcon className="w-6 h-6 mb-1" />
          <SmallText>Send</SmallText>
        </Button>

        <Button className="text-gray-400 hover:text-app-400 transition-all  rounded-lg p-4 cursor-pointer flex  flex-col items-center justify-center">
          <BarsArrowDownIcon className="w-6 h-6 mb-1" />
          <SmallText>Receive</SmallText>
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
