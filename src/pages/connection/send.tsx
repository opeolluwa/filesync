import PageLayout from "@/components/layout/PageLayout";
import React from "react";
import { Steps } from "antd";

const description = "This is a description.";
const steps = [
  {
    title: "First",
    content: "First-content",
  },
  {
    title: "Second",
    content: "Second-content",
  },
  {
    title: "Last",
    content: "Last-content",
  },
];

export default function HelpPage() {
  return (
    <>
      <PageLayout pageTitle={"Connect Device"} includeSearchBar={false}>
        <>
          <Steps
            direction="vertical"
            current={1}
            items={[
              {
                title: "Finished",
                description,
              },
              {
                title: "In Progress",
                description,
              },
              {
                title: "Waiting",
                description,
              },
            ]}
          />
        </>
      </PageLayout>
    </>
  );
}
