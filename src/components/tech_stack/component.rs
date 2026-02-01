use dioxus::prelude::*;
use crate::prelude::components::Section;

use super::Props;

#[component]
pub fn TechStack(props: Props) -> Element {
    rsx! {
        Section {
            title: props.title,
            class: "tech-stack",
            ul {
                for item in props.items {
                    li { {*item} }
                }
            }
        }
    }
}