use dioxus::prelude::*;

mod page;
pub use page::Index;

const _: Asset = asset!("./style.css", CssAssetOptions::new().with_static_head(true));
