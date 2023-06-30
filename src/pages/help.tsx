import PageLayout from "@/components/layout/PageLayout";

// TODO: write the about page here 
// use Tauri API to get the app version from the application core and render here 
// a
/**
 * @function helpPage -  A page responsible for guiding users on various actions
 * @returns tsx
 */
export default function HelpPage() {
  return (
    <>
      <PageLayout pageTitle={"SendFile Help"} includeSearchBar={false}>
        <div>
          <h1>Help</h1>

          <div>some help content goes here</div>
        </div>
      </PageLayout>
    </>
  );
}
