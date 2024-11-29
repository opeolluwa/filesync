import PageLayout from "@/components/layout/desktop/DesktopViewLayout";
import {
  ChartPieIcon,
  CpuChipIcon,
  LanguageIcon,
  MoonIcon,
  SwatchIcon,
  UserIcon,
} from "@heroicons/react/24/outline";

// import the solid icons as alternative
import {
  ChartPieIcon as ChartPieIconSolid,
  CpuChipIcon as CpuChipIconSolid,
  LanguageIcon as LanguageIconSolid,
  MoonIcon as MoonIconSolid,
  SwatchIcon as SwatchIconSolid,
  UserIcon as UserCircleIconSolid,
} from "@heroicons/react/24/solid";

import Card from "@/components/Card";
import Heading from "@/components/Heading";
import { SettingsInterface, SettingsTab } from "@/components/Settings";
import { SystemInformation } from "@/store/system_information";
import { invoke } from "@tauri-apps/api/core";
import { hostname, locale } from "@tauri-apps/plugin-os";
import { useEffect, useState } from "react";



export default function HelpPage() {
  let [systemInformation, setSystemInformation] = useState(
    {} as SystemInformation
  );

  let [deviceHostname, setDeviceHostname] = useState("");
  let [deviceLanguage, setDeviceLanguage] = useState("");

  useEffect(() => {
    const fetchData = async () => {
      try {
        const hostnameData = await hostname();
        setDeviceHostname(hostnameData as string);

        const sysInfo = await invoke("get_system_information");
        setSystemInformation((sysInfo as any).data);

        const language = await locale();
        if (language) {
          setDeviceLanguage(language);
        }
      } catch (error: any) {
        console.log(error.message);
      }
    };

    fetchData();
  }, []);

  const settings: SettingsInterface[] = [
    {
      text: "toggle mode",
      icon: <MoonIcon className="w-6 h-6" />,
      alternateIcon: <MoonIconSolid className="w-6 h-6" />,
    },
    {
      text: "change language",
      icon: <LanguageIcon className="w-6, h-6" />,
      alternateIcon: <LanguageIconSolid className="w-6 h-6" />,
    },
    {
      text: "change theme",
      icon: <SwatchIcon className="w-6 h-6" />,
      alternateIcon: <SwatchIconSolid className="w-6 h-6" />,
    },
  ];
  return (
    <>
      <PageLayout pageTitle={"Settings"} includeSearchBar={false}>
        <Card className="mb-6">
          <Heading context="Personalization" className=" mb-3 font-bold" />
          <div className=" bg-card flex justify-between py-2 rounded-lg px-4 flex-col lg:px-4 lg:pl-6  capitalize">
            {settings.map((control, index) => (
              <SettingsTab
                key={index}
                icon={control.icon}
                alternateIcon={control.alternateIcon}
                text={control.text}
                action={control.action}
              />
            ))}
          </div>
        </Card>

        <Card>
          <Heading context="System Information" className=" mb-3 font-bold" />
          <div className=" flex justify-between  bg-card py-2 rounded-lg px-4 flex-col lg:px-4 lg:pl-6 capitalize  mb-12 overflow-y-scroll">
            <SettingsTab
              icon={<UserIcon className="w-6 h-6" />}
              alternateIcon={<UserCircleIconSolid className="w-6 h-6" />}
              text={deviceHostname}
            />
            <SettingsTab
              icon={<LanguageIcon className="w-6 h-6" />}
              alternateIcon={<LanguageIconSolid className="w-6 h-6" />}
              text={deviceLanguage}
            />

            <SettingsTab
              icon={<ChartPieIcon className="w-6 h-6" />}
              alternateIcon={<ChartPieIconSolid className="w-6 h-6" />}
              text={`${systemInformation.availableDisk} free space`}
            />

            <SettingsTab
              icon={<CpuChipIcon className="w-6 h-6" />}
              alternateIcon={<CpuChipIconSolid className="w-6 h-6" />}
              text={systemInformation.serverBaseUrl}
              withStyle="lowercase"
            />
          </div>
        </Card>
      </PageLayout>
    </>
  );
}
