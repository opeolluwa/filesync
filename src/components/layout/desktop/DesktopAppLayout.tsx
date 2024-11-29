"use client";
import React, { useContext, useState } from "react";
import Navigation from "../../app/AppNavigation";
import NavigationTab from "@/components/Navigation/NavItem";
import { desktopRoutes } from "@/components/Navigation/routes";
import { WifiStatusContext } from "@/store/network";
import { ChevronLeftIcon } from "@heroicons/react/24/outline";

interface Props {
  children: React.ReactNode;
}
export default function DesktopAppLayout({ children }: Props) {
  const { data: isConnectedToWifi } = useContext(WifiStatusContext);
  const [drawerIsOpen, setDrawerIsOpen] = useState(false);

  const toggleDrawer = () => {
    if (drawerIsOpen == true) {
      setDrawerIsOpen(false);
    }
    if (drawerIsOpen == false) {
      setDrawerIsOpen(true);
    }
  };
  return (
    <div
      className="grid grid-cols-12 mb-0 pb-0 w-[100vw]"
      style={{
        height: "100vh",
        overflowY: "hidden",
        marginBottom: 0,
      }}
    >
      <nav
        className={
          drawerIsOpen
            ? "col-span-3 bg-[rgba(249,250,254,255)]   px-[1px]   text-gray-600  pt-10"
            : "col-span-1 bg-[rgba(249,250,254,255)]   px-[1px]   text-gray-600  pt-10 "
        }
        style={{
          height: "calc(100vh-200px)",
          overflowY: "hidden",
          position: "relative",
        }}
      >
        <div className="flex flex-col px-2 ">
          {desktopRoutes.map((route, index) => (
            <NavigationTab
              key={index}
              icon={route.icon}
              name={route.name}
              action={route.action}
              alternateIcon={route.alternateIcon}
              path={route.path}
              disabled={Boolean(isConnectedToWifi) === false}
              drawerIsOpen={drawerIsOpen}
            />
          ))}
        </div>

        <button
          className="flex items-left justify-start lg:items-start lg:my-6 my-4 rounded  ease-in-out  text-app  bg-app-50  py-3 px-1 lg:pl-2 first:mt-4 cursor-pointer mx-2 absolute bottom-0"
          onClick={toggleDrawer}
        >
          <div className="gap-2 justify-left mx-4  flex capitalize">
            <ChevronLeftIcon className="size-5" />
          </div>
        </button>
      </nav>

      <main
        className={
          drawerIsOpen
            ? "col-span-9 lg:col-span-9 pt-10 px-10   overflow-y-scroll"
            : "col-span-11 lg:col-span-9 pt-10 px-10   overflow-y-scroll"
        }
      >
        {children}
      </main>
    </div>
  );
}
