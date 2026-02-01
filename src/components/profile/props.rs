use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct Props {
    pub name: &'static str,
    pub title: String,
    pub status: String,
    pub image: Asset,
    pub image_alt: String
}