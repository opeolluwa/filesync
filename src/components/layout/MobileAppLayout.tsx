import {
  BarsArrowDownIcon,
  ClockIcon,
  Cog6ToothIcon,
  HomeIcon,
} from "@heroicons/react/24/outline";
import { QrCodeIcon } from "@heroicons/react/24/solid";
import Button from "../Button";
import SmallText from "../SmallText";
import View from "../View";
import { scan, Format } from "@tauri-apps/plugin-barcode-scanner";

interface Props {
  children: React.ReactNode;
}

export default function MobileAppLayout({ children }: Props) {
  return (
    <div className="relative min-h-screen overflow-y-scroll">
      <header className=" min-h-12 pt-6 px-4 ">
        <View className="flex items-center justify-between">
          <img
            src="/img/avatar.png"
            alt="test-image"
            className="w-[28px] bg-gray-50 rounded"
          />
          <QrCodeIcon
            className="w-8 h-8 text-gray-400"
            onClick={() => scan({ windowed: true, formats: [Format.QRCode] })}
          ></QrCodeIcon>
        </View>
      </header>
      <div className="px-4 pt-3">{children}</div>

      <footer className="flex bg-app rounded-t-2xl border-t border-t-gray-200 align-center justify-between fixed px-4  mb-0 pb-0 z-50 w-full bottom-0 left-0 shadow-sm shadow-gray-200">
        <Button className=" text-white  transition-all  rounded-lg p-4 cursor-pointer flex flex-col items-center justify-center">
          <HomeIcon className="w-6 h-6 mb-1" />
          <SmallText>Send</SmallText>
        </Button>

        <Button className="text-white hover:text-app-400 transition-all  rounded-lg p-4 cursor-pointer flex  flex-col items-center justify-center">
          <BarsArrowDownIcon className="w-6 h-6 mb-1" />
          <SmallText>Received</SmallText>
        </Button>

        <Button className="text-white hover:text-app-400 transition-all  rounded-lg p-4 cursor-pointer flex  flex-col items-center justify-center">
          <ClockIcon className="w-6 h-6 mb-1" />
          <SmallText>History</SmallText>
        </Button>

        <Button className="text-white hover:bg-app-100 transition-all  rounded-lg p-4 cursor-pointer flex flex-col items-center justify-center">
          <Cog6ToothIcon className="w-6 h-6 mb-1" />
          <SmallText>Settings</SmallText>
        </Button>
      </footer>
    </div>
  );
}
