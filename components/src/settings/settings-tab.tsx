import { useState } from "react";
import { SettingsInterface } from ".";

export default function SettingsTab({
  icon,
  text,
  action,
  withStyle,
  alternateIcon
}: SettingsInterface) {
  const [currentIcon, setIcon] = useState(icon);

  
  return (
    <div
      onClick={action}
      onBlur={() => setIcon(icon)}
      onMouseEnter={() => setIcon(alternateIcon)}
      // onClick={() => setIcon(alternateIcon)}
      onMouseLeave={() => setIcon(icon)}
      className={
        "flex items-center gap-4 ease-in-out py-4 first:mt-4 last:border-none last:mb-4 text-gray-500 cursor-pointer  pl-4  hover:text-app  hover:bg-app-50  rounded-xl" + " "+
        withStyle
      }
    >
      {currentIcon} {text}
    </div>
  );
}
