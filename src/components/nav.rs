use super::super::components::navbar::{
    Navbar, NavbarContent, NavbarItem, NavbarNav, NavbarTrigger,
};
use super::super::route::Route;
use dioxus::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn Nav() -> Element {
    rsx! {

        div { class: "navbar",
            Navbar { aria_label: "Components",
                NavbarNav { index: 1usize,
                    margin: "auto",
                    NavbarTrigger { "Menu" }
                    NavbarContent {
                        NavbarItem {
                            index: 0usize,
                            value: "entrance".to_string(),
                            to: Route::Entrance {},
                            "Casino Entrance"
                        }
                        NavbarItem {
                            index: 1usize,
                            value: "info".to_string(),
                            to: Route::InfoElement {},
                            "Info"
                        }
                    }
                }
            }
        }
      // nav {
      // ul {
      // li {
      // Link {
      //   to: Route::Entrance {},
      // "Entrance"
      // }
      // }
      // li {
      // Link {
      //   to: Route::InfoElement {},
      // "info page"
      // }
      // }
      // }
      // }
    }
}
