use dioxus_free_icons::{Icon, icons::{fa_brands_icons::{FaFacebook, FaGithub, FaInstagram, FaSpotify, FaTelegram}}};
use dioxus_i18n::t;
use dioxus::prelude::*;

use crate::prelude::{components::{OnTheWeb, Profile, Section, TechStack}};

#[component]
pub fn Index() -> Element {
    rsx! {
        p { class: "greeting", {t!("greeting")} }

        Profile {
            name: "Oskar Baran",
            title: t!("profile.title"),
            status: t!("profile.status"),
            image: asset!("assets/image.png"),
            image_alt: "Photo of me",
        }

        Section { title: t!("about-me"),
            p { {t!("about-me.content")} }
        }

        OnTheWeb {
            title: t!("on-the-web"),
            items: vec![
                (
                    "https://github.com/85oski",
                    rsx! {
                        Icon { icon : FaGithub }
                    },
                ),
                (
                    "https://www.facebook.com/oskar.baran.01",
                    rsx! {
                        Icon { icon : FaFacebook }
                    },
                ),
                (
                    "https://www.telegram.me/oskarbaran",
                    rsx! {
                        Icon { icon : FaTelegram }
                    },
                ),
                (
                    "https://open.spotify.com/user/31rj4uuz6etfsiwo35izmfmamepe?si=785bfdbd008540cf",
                    rsx! {
                        Icon { icon : FaSpotify }
                    },
                ),
                (
                    "https://www.instagram.com/85oski/",
                    rsx! {
                        Icon { icon : FaInstagram }
                    },
                ),
            ],
        }

        TechStack {
            title: t!("tech-stack"),
            items: &[
                "NextJS", "NuxtJS", "TypeScript",
                "CSS", "SCSS", "HTML", "Rust",
                "Dioxus", "Vue", "JavaScript",
                "SQL",
            ],
        }

        Section { title: t!("what-i-love"),
            p { {t!("what-i-love.content")} }
        }

        Section { title: t!("biography"), class: "biography",
            table {
                tr {
                    th { "2006" }
                    td { {t!("biography.at-2006")} }
                }
                tr {
                    th { "2020" }
                    td { {t!("biography.at-2020")} }
                }
                tr {
                    th { "2024" }
                    td { {t!("biography.at-2024")} }
                }
                tr {
                    th { "2025" }
                    td { {t!("biography.at-2025")} }
                }
            }
        }
    }
}