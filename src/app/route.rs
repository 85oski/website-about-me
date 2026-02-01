use dioxus::prelude::*;

use crate::prelude::{
    app::layout::Layout,
    pages::{Index, NotFound},
};

#[derive(Clone, Routable)]
pub enum Route {
    #[layout(Layout)]
    #[route("/")]
    Index,
    #[route("/:..segments")]
    NotFound { segments: Vec<String> },
}
