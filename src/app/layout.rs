use dioxus::prelude::*;
use crate::prelude::{app::Route, components::Navbar};

#[component]
pub fn Layout() -> Element {
    rsx! {
        Navbar { title: "Oskar Baran" }
        main { Outlet::<Route> {} }
    }
}
