import AudioList from "@/components/AudioList";
import PageLayout from "@/components/layout/PageLayout";

export default function Desktop() {
  return (
    <>
      <PageLayout pageTitle={"Videos"} includeSearchBar={true}>
        <div>
          <h1>Help</h1>
          <AudioList />
          <div>some help content goes here</div>
        </div>
      </PageLayout>
    </>
  );
}
