import PageLayout from "@/components/layout/PageLayout";
import { AndroidFilled } from "@ant-design/icons";
import { Switch } from "@headlessui/react";
import { Radio } from "antd";
import {
  LanguageIcon,
  MoonIcon,
  SwatchIcon,
} from "@heroicons/react/24/outline";
import { SettingsInterface, SettingsTab } from "@/components/settings";

/**
 * @function helpPage -  A page responsible for guiding users on various actions
 * @returns tsx
 */

export default function HelpPage() {
  const settings: SettingsInterface[] = [
    {
      text: "toggle mode",
      icon: <MoonIcon className="w-6 h-6" />,
    },
    {
      text: "change language",
      icon: <LanguageIcon className="w-6, h-6" />,
    },
    {
      text: "change theme",
      icon: <SwatchIcon className="w-6 h-6" />,
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
          <div className="flex flex-col px-2 lg:px-4 lg:pl-6">
            {settings.map((control, index) => (
              <SettingsTab
                key={index}
                icon={control.icon}
                text={control.text}
                action={control.action}
              />
            ))}
          </div>
        </div>
      </PageLayout>
    </>
  );
}
