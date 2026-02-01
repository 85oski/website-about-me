use dioxus::prelude::*;
use super::Props;

#[component]
pub fn Section(props: Props) -> Element {
    rsx! {
        section { class: props.class,
            h3 { {props.title} }
            {props.children}
        }
    }
}