import PageLayout from "@/components/layout/PageLayout";


export default function HelpPage() {
  return (
    <>
      <PageLayout pageTitle={"Help"} includeSearchBar={false}>
        <div>
          <h1>Help</h1>
          <div>some help content goes here</div>
        </div>
      </PageLayout>
    </>
  );
}
