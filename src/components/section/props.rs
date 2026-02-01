use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct Props {
    pub title: String,
    pub children: Element,
    pub class: Option<&'static str>
}