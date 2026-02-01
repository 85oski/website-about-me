use dioxus::prelude::*;

use dioxus_free_icons::{
    Icon, icons::ld_icons::{LdSearchX, LdUndo2}
};
use dioxus_i18n::t;

use crate::prelude::app::Route;

#[component]
pub fn NotFound(segments: Vec<String>) -> Element {
    rsx! {
        section { class: "not-found",
            p {
                Icon { icon: LdSearchX }
                "404"
            }
            aside {
                p { {t!("not-found")} }
                Link { to: Route::Index,
                    Icon { icon: LdUndo2 }
                }
            }
        }
    }
}
