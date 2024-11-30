
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
        Route::new("home", "/", "ri-home-line"),
        Route::new("home", "/", "ri-home-line"),
        Route::new("home", "/", "ri-home-line"),
    ];

    view! { <div class="flex flex-col">
        {
    routes.into_iter().map(|route| {
        let Route{route,icon_class, label}= route;
          let active_class =
    "flex items-left justify-start lg:items-start lg:my-6 my-4 rounded  ease-in-out  text-app  bg-app-50  py-3 px-1 lg:pl-2 first:mt-4 cursor-pointer";

  let preview_class =
    "flex items-left justify-start lg:items-start lg:my-6 my-4 rounded  ease-in-out  hover:text-app py-3 px-1 lg:pl-2 first:mt-4  text-gray-500 cursor-pointer";

        view!{
  <a href=route class="flex flex-col dark:text-gray-400">
       <i class="ri ri-home-line"></i>
            <span class="sr-only">{label}</span>
        </a>
    }}).collect_view()
        }
        </div> }
}
