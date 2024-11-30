use leptos::{view, IntoView};

#[leptos::component]
pub fn HomeView() -> impl IntoView {
    view! {
        <div class=" text-center flex flex-col align-center justify-center items-center h-[500px]">
            <div>
                <h1 class="font-medium leading-2 text-2xl text-gray-700 dark:text-gray-400 ">
                    What would you like to do?
                </h1>
                <p class="text-base">Do you want to send or receive files?</p>

                <div class="flex justify-center gap-x-5 items-center mt-8">
                    <button class="flex flex-col items-center ">
                        <div class="dark:bg-gray-700 dark:hover:bg-gray-900/50 bg-gray-200 text-gray-400 hover:bg-app-50 hover:text-app transition-all duration-200  p-4 rounded-xl shadow hover:shadow-none cursor-pointer">
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke-width="1.5"
                                stroke="currentColor"
                                class="size-6"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    d="M8.25 6.75 12 3m0 0 3.75 3.75M12 3v18"
                                />
                            </svg>

                        </div>
                        <p class="mt-2">Send File</p>
                    </button>
                    <div class="flex flex-col items-center ">
                        <button class=" dark:bg-gray-700 dark:hover:bg-gray-900/50 bg-gray-200 text-gray-400 hover:bg-app-50 hover:text-app transition-all duration-200 p-4 rounded-xl shadow hover:shadow-none cursor-pointer">
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke-width="1.5"
                                stroke="currentColor"
                                class="size-6"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    d="M15.75 17.25 12 21m0 0-3.75-3.75M12 21V3"
                                />
                            </svg>
                        </button>
                        <p class="mt-2">Receive File</p>
                    </div>
                </div>
            </div>
        </div>
    }
}
