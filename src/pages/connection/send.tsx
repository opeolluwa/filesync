import PageLayout from "@/components/layout/PageLayout";
import React from "react";
import { Steps } from "antd";

export default function HelpPage() {
  return (
    <>
      <PageLayout pageTitle={"Connect Device"} includeSearchBar={false}>
        <div>
          <button
            type="button"
            className="flex justify-center rounded-md   px-4 py-2 text-sm font-medium border border-mirage-500 bg-gray-600 mt-2 text-white my-4"
          >
            Step 01:
          </button> 
          form
          <StepOne />
          <button
            type="button"
            className="flex justify-center rounded-md   px-4 py-2 text-sm font-medium border border-mirage-500 bg-gray-600 mt-2 text-white my-4"
          >
            Step 02:
          </button>
          <StepTwo />

          <button
            type="button"
            className="flex justify-center rounded-md   px-4 py-2 text-sm font-medium border border-mirage-500 bg-gray-600 mt-2 text-white my-4"
          >
            Step 03:
          </button>
          <StepTree />
        </div>
      </PageLayout>
    </>
  );
}

function StepOne() {
  // tell use to create hotspot and add a button to do so
  return (
    <div className="my-8 fle flex-col pl-12 gap-2">
      <p className="leading-3 hidden">
        Create a WiFi hotspot on your device and connect to it from the other
        device
      </p>
      <button
        type="button"
        className="flex justify-center rounded-md   px-4 py-2 text-sm font-medium border border-mirage-500 bg-app mt-2 text-white hidden"
      >
        Create Hotspot
      </button>
    </div>
  );
}

function StepTwo() {
  // tell user to connect peer device via wifi
  return (
    <div className="my-8 pl-12">
      <p className="leading-3 text-gray-400 hidden">
        Connect to the hotspot you created on the other device
      </p>
    </div>
  );
}

function StepTree() {
  // tell user to open up connection in app
  return (
    <div className="my-8 pl-12">
      <div className="my-4 pl-4 text-gray hidden">
        Open up the application or web browser on the other device and go to
        this page
      </div>
    </div>
  );
}
