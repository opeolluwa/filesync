import PageLayout from "@/components/layout/PageLayout";
import { AndroidFilled } from "@ant-design/icons";
import {
  ChartPieIcon,
  ClockIcon,
  CpuChipIcon,
  LanguageIcon,
  MoonIcon,
  SwatchIcon,
  UserCircleIcon,
} from "@heroicons/react/24/outline";

// import the solid icons as alternative
import {
  ChartPieIcon as ChartPieIconSolid,
  ClockIcon as ClockIconSolid,
  CpuChipIcon as CpuChipIconSolid,
  LanguageIcon as LanguageIconSolid,
  MoonIcon as MoonIconSolid,
  SwatchIcon as SwatchIconSolid,
  UserCircleIcon as UserCircleIconSolid,
} from "@heroicons/react/24/solid";

import { SettingsInterface, SettingsTab } from "@/components/settings";
import Heading from "@/components/Heading";
import { SystemInformation } from "@/store/sys-info";
import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from "react";
import { State } from "../../../core/bindings/State";
/**
 * @function helpPage -  A page responsible for guiding users on various actions
 * @returns tsx
 */

export default function HelpPage() {
  let [systemInformation, setSystemInformation] = useState(
    {} as SystemInformation
  );

  useEffect(() => {
    // fetch sys information from app core
    invoke("get_system_information").then((sysInfo) => {
      setSystemInformation((sysInfo as any).data);
    });
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
      <PageLayout
        pageTitle={"Settings"}
        includeSearchBar={false}
        // placeholder="search settings"
      >
        <div>
          <Heading context="Personalization" className="mt-12 mb-2" />
          <div className=" bg-white flex justify-between py-2 rounded-lg px-4 flex-col lg:px-4 lg:pl-6  capitalize">
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

          <Heading context="System Information" className="mt-12 mb-2" />
          <div className=" flex justify-between  bg-white py-2 rounded-lg px-4 flex-col lg:px-4 lg:pl-6 capitalize  mb-12 overflow-y-scroll">
            <SettingsTab
              icon={<UserCircleIcon className="w-6 h-6" />}
              alternateIcon={<UserCircleIconSolid className="w-6 h-6" />}
              text={systemInformation.systemName}
            />
            <SettingsTab
              icon={<AndroidFilled className="w-6 h-6" />}
              alternateIcon={<AndroidFilled className="w-6 h-6" />}
              text={String(systemInformation.port)}
            />

            <SettingsTab
              icon={<ChartPieIcon className="w-6 h-6" />}
              alternateIcon={<ChartPieIconSolid className="w-6 h-6" />}
              text={`${systemInformation.availableDisk} of ${systemInformation.usedDisk} free`}
            />

            <SettingsTab
              icon={<ClockIcon className="w-6 h-6" />}
              alternateIcon={<ClockIconSolid className="w-6 h-6" />}
              text={String(systemInformation?.remainingTime)}
            />

            <SettingsTab
              icon={<CpuChipIcon className="w-6 h-6" />}
              alternateIcon={<CpuChipIconSolid className="w-6 h-6" />}
              text={systemInformation.serverBaseUrl}
              withStyle="lowercase"
            />
          </div>
        </div>
      </PageLayout>
    </>
  );
}
