import Link from "next/link";
import React, { useState } from "react";
import { useRouter } from "next/router";
import { Route } from "./routes";



export default function NavigationTab({
  icon,
  alternateIcon,
  action,
  path,
  name,
  disabled,
}: Route) {
  const [currentIcon, setIcon] = useState(icon);
  const router = useRouter();

  const activeClass =
    "flex items-left justify-start lg:items-start lg:my-6 my-4 rounded  ease-in-out  text-app  bg-app-50  py-3 px-1 lg:pl-2 first:mt-4 cursor-pointer";

  const previewClass =
    "flex items-left justify-start lg:items-start lg:my-6 my-4 rounded  ease-in-out  hover:text-app py-3 px-1 lg:pl-2 first:mt-4  text-gray-500 cursor-pointer"; 

  if (disabled) {
    return (
      <div onClick={action}>
        <div
          className={
            "flex items-left justify-start lg:items-start lg:my-6 my-4 rounded  ease-in-out  py-3 px-1 lg:pl-2 first:mt-4  text-gray-500 cursor-pointer"
          }
        >
          <span className="cursor-pointer">
            <span className="sr-only">{path}</span>
            <div className="gap-2 justify-left mx-4  flex capitalize">
              {router.pathname == path.trim() ? alternateIcon : currentIcon}
              <span className="">{name}</span>
            </div>
          </span>
        </div>
      </div>
    );
  }

  return (
    <div onClick={action}>
      <Link
        href={path}
        className={
          router.pathname == path.trim() 
            ? activeClass
            : previewClass
        }
        onBlur={() => setIcon(icon)}
        onMouseEnter={() => setIcon(alternateIcon)}
        onClick={() => setIcon(alternateIcon)}
        onMouseLeave={() => setIcon(icon)}
      >
        <span className="cursor-pointer">
          <span className="sr-only">{router.asPath.split("/")[1]} </span>
          <div className="gap-2 justify-left mx-4  flex capitalize">
            {router.pathname == path.trim() ? alternateIcon : currentIcon}
            <span className="">{name}</span>
          </div>
        </span>
      </Link>
    </div>
  );
}
