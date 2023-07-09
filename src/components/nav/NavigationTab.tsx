import Link from "next/link";
import React, { useState } from "react";
import { useRouter } from "next/router";
export interface Route {
  icon: any; // the route icon
  name: string; // the route name
  alternateIcon: any; // the icon to show on hover or active state
  action?: () => any; // action that will be executed when the route is clicked
  path: string; // the path string
  isActive?: any;
}

export default function NavigationTab({
  icon,
  alternateIcon,
  action,
  path,
  name,
}: Route) {
  const [currentIcon, setIcon] = useState(icon);
  const router = useRouter();
  return (
    <div onClick={action}>
      <Link
        href={path}
        className={
          router.pathname == path.trim()
            ? "flex items-center justify-center lg:justify-start lg:items-start lg:my-6 my-4 rounded  ease-in-out  hover:text-app  hover:bg-app-50 text-app  bg-app-50 py-3 px-1 lg:pl-2 first:mt-4 cursor-pointer"
            : "flex items-center justify-center lg:justify-start lg:items-start lg:my-6 my-4 rounded  ease-in-out  hover:text-app hover:bg-app-50 py-3 px-1 lg:pl-2 first:mt-4  text-gray-500 cursor-pointer"
        }
        onBlur={() => setIcon(icon)}
        onMouseEnter={() => setIcon(alternateIcon)}
        onClick={() => setIcon(alternateIcon)}
        onMouseLeave={() => setIcon(icon)}
      >
        <span className="cursor-pointer">
          <span className="sr-only">{path}</span>
          <div className="gap-2 justify-center align-center flex capitalize">
            {router.pathname == path.trim() ? alternateIcon : currentIcon}
            <span className="hidden lg:block" id="route__name">
              {name}
            </span>
          </div>
        </span>
      </Link>
    </div>
  );
}
