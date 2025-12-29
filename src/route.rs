use super::components::entrance::Entrance;
use super::components::info_page::InfoElement;
use super::components::template::Template;
use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq, Routable)]
pub enum Route {
    #[layout(Template)]
    #[route("/")]
    Entrance {},
    #[route("/info_page")]
    InfoElement {},
}
