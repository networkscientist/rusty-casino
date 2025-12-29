use crate::components::lucky_numbers::LuckyNumbersTable;
use dioxus::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn Entrance() -> Element {
    rsx! {
      div { id: "title",
            h1 { "Rusty Casino" }
        }
        div { id: "subtitle",
            h2 { "Entrance" }
        LuckyNumbersTable {}
            }
    }
}
