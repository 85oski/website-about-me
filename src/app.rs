use crate::prelude::{
    hooks::{use_init_locale, use_init_scheme},
    layout::Layout,
    views::*,
};
use dioxus::prelude::*;

#[derive(Clone, Routable)]
pub enum Route {
    #[layout(Layout)]
    #[route("/")]
    Index {},
    #[route("/about-me")]
    AboutMe {},
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}

const FAVICON: Asset = asset!("assets/favicon.svg");
const STYLE: Asset = asset!("src/styles/main.scss");

#[component]
pub fn App() -> Element {
    use_init_scheme();
    use_init_locale();

    rsx! {
        document::Title { "Oskar Baran" }
        document::Stylesheet { href: STYLE }
        document::Link { rel: "icon", href: FAVICON }

        Router<Route> {}
    }
}
