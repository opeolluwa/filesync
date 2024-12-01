use leptos::{view, CollectView};

#[derive(Debug)]
pub struct Route {
    pub label: String,
    pub route: String,
    pub icon_class: String,
    pub is_active: bool,
}

impl Route {
    pub fn new(label: &str, route: &str, icon_class: &str) -> Self {
        Self {
            label: label.to_string(),
            route: route.to_string(),
            icon_class: icon_class.to_string(),
            is_active: false,
        }
    }
}

#[leptos::component]

pub fn SideNavigation() -> impl leptos::IntoView {
    let routes: Vec<Route> = vec![
        Route::new("transfer", "/", "arrow-left-right-fill"),
        Route::new("share", "/share", "share-line"),
        Route::new("settings", "/settings", "settings-5-line"),
        Route::new("about", "/about", "information-line"),
    ];

    view! {
        <div class="flex flex-col relative items-center justify-center gap-y-4 px-2">
            {routes
                .into_iter()
                .map(|route| {
                    let Route { route: path, icon_class, label, .. } = route;
                    let icon_class = format!("ri-{icon_class} ri-lg");
                    // TODO: update the current route
                    view! {
                        <a
                            href=path
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
