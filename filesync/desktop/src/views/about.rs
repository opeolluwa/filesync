use leptos::view;

#[leptos::component]
pub fn AboutView() -> impl leptos::IntoView {
  view! {
      <div className="p-4 rounded-lg dark:dark-900">
          <div className="block mb-4 ">
              <img
                  src="/icons/app-icon.png"
                  alt="app icon"
                  width=200
                  height=200
                  className="w-[50px] block mx-auto"
              />

              <div className="flex flex-col justify-center items-center rounded-lg full my-4 p-4 ">
                  <h1 className="text-center text-medium font-2xl text-base capitalize">
                    //   {appName} {appVersion}
                  </h1>
                  <a
                      href="https://github.com/opeolluwa/filesync"
                      className="small text-gray text-dark text-center"
                  >
                      
                      "https://github.com/opeolluwa/filesync"
                  </a>
                  {" "}
              </div>
          </div>
      </div>
  }
}