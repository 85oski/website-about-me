use dioxus::prelude::*;

use crate::prelude::{components::Navbar, Route};

#[component]
pub fn Layout() -> Element {
    rsx! {
        Navbar {}
        main {
            Outlet<Route> {}
        }
    }
}