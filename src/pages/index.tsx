"use client";

import Button from "@/components/Button";
import Card from "@/components/Card";
import Heading from "@/components/Heading";
import Loader from "@/components/Loaders/LoaderWifi";
import Text from "@/components/Text";
import View from "@/components/View";
import { DeviceInformationContext } from "@/store/device";
import { WifiStatusContext } from "@/store/network";
import { useContext, useEffect, useState } from "react";
import DesktopAppEntry from "./desktop";
import MobileAppEntry from "./mobile";
import { OsType } from "@tauri-apps/plugin-os";
import { type } from "os";

export default function Home() {
  // const { osType } = useContext(DeviceInformationContext);
  // const isMobile = osType === "Android";
  // const { data: isConnectedToWifi } = useContext(WifiStatusContext);

    const [osType, setOsType] = useState<OsType>("" as OsType);
    const [isMobile, setIsMobile] = useState(false);
    useEffect(() => {
      const fetchData = () => {
        const os = type();
        setOsType(os);

        if (os == "android" || os == "ios") {
          setIsMobile(true);
        }
      };

      fetchData();
    }, []);

    if (isMobile) {
      return <MobileAppEntry />;
    } else {
      return <DesktopAppEntry />;
    }


  return <>
  Lorem ipsum dolor sit amet consectetur adipisicing elit. Voluptatum nulla deserunt quaerat dolores ex laborum, esse voluptatem autem, consectetur reprehenderit delectus provident vitae earum vero, facere at expedita culpa recusandae.
  </>
  // if (!isConnectedToWifi) {
  //   return (
  //     <Card className="text-gray-600  h-[75%]  w-full my-auto flex justify-center items-center flex-col -mt-30">
  //       <Loader />
  //       <Heading>No network connected</Heading>
  //       <Text className="text-center leading-3 mt-2">
  //         Filesync couldn&apos;t detect local network.<br></br>please connect to
  //         a wifi and retry
  //       </Text>
  //       <View className="flex gap-x-4 items-center justify-center text-gray-400 mt-5 ">
  //         <Button className="bg-red-500 hover:bg-red-700  text-white py-[5px] px-6">
  //           Exit
  //         </Button>
  //         <Button className="bg-app-500 hover:bg-app-700 text-white py-[5px]">
  //           Reload
  //         </Button>
  //       </View>
  //     </Card>
  //   );
  // }
  // if (isMobile) {
  //   return <MobileApp />;
  // } else {
  //   return <DesktopApp />;
  // }
}
