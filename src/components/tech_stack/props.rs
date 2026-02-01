use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct Props {
    pub title: String,
    pub items: &'static [&'static str]
}