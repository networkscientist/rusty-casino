use dioxus::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn InfoElement() -> Element {
    rsx! {
        h1 { "Rusty Casino" }

        p {
          "Here goes some info.",
        }
    }
}
