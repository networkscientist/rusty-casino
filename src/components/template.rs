use super::super::route::Route;
use super::nav::Nav;
use dioxus::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn Template() -> Element {
    const STYLESHEET_TAILWIND: Asset = asset!("../../assets/main.css");
    const STYLESHEET_COMPONENT: Asset = asset!("../../assets/dx-components-theme.css");

    rsx! {

      document::Link {
        href: STYLESHEET_TAILWIND,
        rel: "stylesheet",
      }
      document::Link {
        href: STYLESHEET_COMPONENT,
        rel: "stylesheet",
      }
      Nav { }
      Outlet::<Route> {}
    }
}
