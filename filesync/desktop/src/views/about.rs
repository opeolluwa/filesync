use leptos::view;

use crate::desktop_application::invoke_without_args;

#[leptos::component]
pub fn AboutUI() -> impl leptos::IntoView {
    let app_config =  invoke_without_args("get_app_config" ).await;
  view! {
      <div class="flex justify-center items-center">
          <div className="p-4 rounded-lg dark:dark-900">
              <div className="block mb-4 ">
                  <img
                      src="/images/app-icon.png"
                      alt="app icon"

                      className="w-[50px] block mx-auto"
                  />

                  <div className="flex flex-col justify-center items-center rounded-lg full my-4 p-4 ">
                      <h1 className="text-center text-medium font-2xl text-base capitalize">// {appName} {appVersion}
                      </h1>
                      <a
                          href="https://github.com/opeolluwa/filesync"
                          className="small text-gray text-dark text-center flex gap-x-2"
                      >

                          "https://github.com/opeolluwa/filesync"

                          <svg
                              xmlns="http://www.w3.org/2000/svg"
                              viewBox="0 0 20 20"
                              fill="currentColor"
                              class="size-5"
                          >
                              <path d="M12.232 4.232a2.5 2.5 0 0 1 3.536 3.536l-1.225 1.224a.75.75 0 0 0 1.061 1.06l1.224-1.224a4 4 0 0 0-5.656-5.656l-3 3a4 4 0 0 0 .225 5.865.75.75 0 0 0 .977-1.138 2.5 2.5 0 0 1-.142-3.667l3-3Z" />
                              <path d="M11.603 7.963a.75.75 0 0 0-.977 1.138 2.5 2.5 0 0 1 .142 3.667l-3 3a2.5 2.5 0 0 1-3.536-3.536l1.225-1.224a.75.75 0 0 0-1.061-1.06l-1.224 1.224a4 4 0 1 0 5.656 5.656l3-3a4 4 0 0 0-.225-5.865Z" />
                          </svg>

                      </a>

                  </div>
              </div>
          </div>
      </div>
  }
}