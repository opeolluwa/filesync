import SettingsTab from "./settings-tab";
interface SettingsInterface {
  icon: any;
  text: string;
  action?: () => void;
  withStyle?: string;
  alternateIcon:any,
}
export { SettingsTab };
export type { SettingsInterface };
