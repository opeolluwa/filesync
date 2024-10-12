import React from "react";
import SettingsPage from "./../settings/index";
import MobileViewLayout from "@/components/layout/mobile/MobileViewLayout";
export default function settings() {
  return (
    <MobileViewLayout>
      <SettingsPage />
    </MobileViewLayout>
  );
}
