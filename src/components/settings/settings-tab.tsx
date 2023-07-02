import { SettingsInterface } from ".";

export default function SettingsTab({ icon, text, action }: SettingsInterface) {
  return (
    <div
      onClick={action}
      className="flex items-center gap-4 my-4 rounded  ease-in-out  hover:text-app capitalize py-3 px-1 lg:pl-2 first:mt-4  text-gray-500 cursor-pointer "
    >
      {icon} {text}
    </div>
  );
}
