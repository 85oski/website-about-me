mod layout;
mod route;

use dioxus::prelude::*;
pub use layout::Layout;
pub use route::Route;

use crate::prelude::hooks::{use_init_locale, use_init_theme};

const _: Asset = asset!("/src/styles/base/animations.css", CssAssetOptions::new().with_static_head(true));
const _: Asset = asset!("/src/styles/base/variables.css", CssAssetOptions::new().with_static_head(true));
const _: Asset = asset!("/src/styles/main.css", CssAssetOptions::new().with_static_head(true));
const _: Asset = asset!("/src/styles/theme.css", CssAssetOptions::new().with_static_head(true));
const _: Asset = asset!("/src/styles/base/modern-normalize.css", CssAssetOptions::new().with_static_head(true));

const FAVICON: Asset = asset!("./favicon.ico");

#[component]
pub fn App() -> Element {
    use_init_theme();
    use_init_locale();

    rsx! {
        document::Title { "Oskar Baran" }
        document::Link { rel: "icon", href: FAVICON }
        document::Meta { name: "author", content: "Oskar Baran" }

        Router::<Route> {}
    }
}