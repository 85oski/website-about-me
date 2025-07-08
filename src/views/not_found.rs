use dioxus::prelude::*;
use dioxus_free_icons::{icons::md_navigation_icons::MdArrowForward, Icon};
use dioxus_i18n::t;
use crate::prelude::Route;

#[component]
pub fn NotFound(segments: Vec<String>) -> Element {
    let path = segments.join("/");
    rsx! {
        section {
            class: "not-found",
            p {
                {t!("not-found", path: path)}
            }
            Link {
                to: Route::Index {  },
                {t!("not-found.button")},
                Icon {
                    icon: MdArrowForward
                }
            }
        }
    }
}