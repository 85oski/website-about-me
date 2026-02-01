use dioxus::prelude::*;
use super::Props;

#[component]
pub fn Profile(props: Props) -> Element {
    rsx! {
        section { class: "profile",
            section {
                p { {props.name} }
                p { {props.title} }
                span { {props.status} }
            }
            img { src: props.image, alt: props.image_alt }
        }
    }
}