use leptos::{view, CollectView};

#[derive(Debug)]
pub struct Route {
    pub label: String,
    pub route: String,
    pub icon_class: String,
}

impl Route {
    pub fn _action<F>(&self, _closure: F) -> ()
    where
        F: Fn(),
    {
    }

    /// Creates a new `Route`.
    pub fn new(label: &str, route: &str, icon_class: &str) -> Self {
        Self {
            label: label.to_string(),
            route: route.to_string(),
            icon_class: icon_class.to_string(),
        }
    }
}

#[leptos::component]

pub fn SideNavigation() -> impl leptos::IntoView {
    let routes: Vec<Route> = vec![
        Route::new("home", "/", "arrow-left-right-fill"),
        // Route::new("home", "/", "time-line"),
        Route::new("home", "/", "share-line"),
        // Route::new("home", "/", "folder-download-line"),
        Route::new("home", "/", "settings-5-line"),
        Route::new("home", "/", "home-line"),
    ];

    view! {
        <div class="flex flex-col relative items-center justify-center gap-y-4 px-2">
            {routes
                .into_iter()
                .map(|route| {
                    let Route { route, icon_class, label } = route;
                    let icon_class = format!("ri-{icon_class} ri-lg");
                    view! {
                        <a
                            href=route
                            class="text-gray-500 flex flex-col justify-center items-center dark:hover:bg-gray-700/40 hover:bg-app-50 hover:text-app p-3 w-full rounded-lg"
                        >
                            <i class=icon_class></i>
                            <span class="sr-only">{label}</span>
                        </a>
                    }
                })
                .collect_view()}

        </div>
    }
}
