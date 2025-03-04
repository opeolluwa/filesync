use leptos::{prelude::RwSignal, view};

use thaw::{Tab, TabList};

#[leptos::component]
pub fn HomeScreen() -> impl leptos::IntoView {
  let selected_value = RwSignal::new(String::new());
  
   view! {
       <TabList selected_value class="flex justify-between px-3">
           <Tab value="apple">"Apple"</Tab>
           <Tab value="pear">"Pear"</Tab>
           <Tab value="item1">"Item 1"</Tab>
           <Tab value="item2">"Item 2"</Tab>
       </TabList>
   }
}
