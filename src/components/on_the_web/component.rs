use dioxus::prelude::*;
use crate::prelude::components::Section;

use super::Props;

#[component]
pub fn OnTheWeb(props: Props) -> Element {
    rsx! {
        Section {
            title: props.title,
            class: "on-the-web",

            ul {
                for (url, icon) in props.items {
                    li {
                        a {
                            target: "_blank",
                            href: url,
                            {icon}
                        }
                    }
                }
            }
        }
    }
}