import PageLayout from "@/components/layout/PageLayout";

export default function Desktop() {
  return (
    <>
      <PageLayout pageTitle={"Downloads"} includeSearchBar={true}>
        <div>
          <h1>Help</h1>

          <div>some help content goes here</div>
        </div>
      </PageLayout>
    </>
  );
}
