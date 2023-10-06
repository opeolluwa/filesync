import SettingsTab from "./settings-tab";
interface SettingsInterface {
  icon: any;
  text: string;
  action?: () => void;
  withStyle?: string;
}
export { SettingsTab };
export type { SettingsInterface };
