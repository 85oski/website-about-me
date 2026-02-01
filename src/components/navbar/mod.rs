use dioxus::prelude::*;

mod component;
pub use component::Navbar;

const _: Asset = asset!("./style.css", CssAssetOptions::new().with_static_head(true));