import PageLayout from "@/components/layout/desktop/DesktopViewLayout";
import View from "@/components/View";
import React from "react";

export default function AboutPage() {
  return (
    <PageLayout pageTitle={"About"} includeSearchBar={false}>
      <View>
        Lorem ipsum dolor sit amet consectetur adipisicing elit. Nisi assumenda,
        delectus dignissimos ducimus saepe temporibus quae id doloribus, magnam
        necessitatibus sapiente minus facere sint enim! Minus praesentium iste
        illum id.
      </View>
    </PageLayout>
  );
}
