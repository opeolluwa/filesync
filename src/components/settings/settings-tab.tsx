import { SettingsInterface } from ".";

export default function SettingsTab({
  icon,
  text,
  action,
  withStyle,
}: SettingsInterface) {
  return (
    <div
      onClick={action}
      className={
        "flex items-center gap-4 ease-in-out  hover:text-app capitalie py-4 px-1 lg:pl-2 first:mt-4 last:border-none last:mb-4 text-gray-500 cursor-pointer dark:border-b dark:border-1 dark:border-dark-800 " +
        withStyle
      }
    >
      {icon} {text}
    </div>
  );
}
