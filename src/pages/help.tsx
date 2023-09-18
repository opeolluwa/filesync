import PageLayout from "@/components/layout/PageLayout";
import LoaderCircle from "@/components/loaders/LoaderCircle";
import LoaderDevices from "@/components/loaders/LoaderDevices";

export default function HelpPage() {
  return (
    <>
      <PageLayout pageTitle={"Help"} includeSearchBar={false}>
        <div>
          <h1>Help</h1>
          <div>some help content goes here</div>
          <LoaderDevices />
        </div>
      </PageLayout>
    </>
  );
}
