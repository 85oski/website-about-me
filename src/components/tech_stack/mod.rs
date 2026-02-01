use dioxus::prelude::*;

mod component;
pub use component::TechStack;

mod props;
use props::Props;

const _: Asset = asset!("./style.css", CssAssetOptions::new().with_static_head(true));