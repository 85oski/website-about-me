use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::{bs_icons::*, fa_brands_icons::{FaCss3, FaHtml5, FaJs, FaPhp, FaPython, FaRust}, md_navigation_icons::MdArrowForward}, Icon
};
use dioxus_i18n::t;


const IMAGE: Asset = asset!("assets/image.png");

#[component]
pub fn Index() -> Element {
    rsx! {
        p {
            class: "welcome-message",
            {t!("welcome-message")}
        }
        section {
            class: "card",
            section {
                p { "Oskar Baran" }
                p { {t!("card.title")} }
            }
            img {
                src: IMAGE,
                alt: "Photo of me",
                loading: "lazy"
            }
        }
        section {
            class: "about-me",
            h3 { {t!("about-me")} }
            p { {t!("about-me.content")} }
        }
        section {
            class: "bio",
            h3 { {t!("biography")} }

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
            }
        }
        section {
            class: "what-i-love",
            h3 { {t!("what-i-love")} }
            p { {t!("what-i-love.content")} }
        }
        section {
            class: "languages-i-know",
            h3 { {t!("about-me.languages-i-know")} }
            ul {
                li {
                    Icon { icon: FaRust }
                    "Rust"
                }
                li {
                    Icon { icon: FaJs }
                    "JavaScript"
                }
                li {
                    Icon { icon: FaHtml5 }
                    "HTML"
                }
                li {
                    Icon { icon: FaCss3 }
                    "CSS"
                }
                li {
                    Icon { icon: FaPhp }
                    "PHP"
                }
                li {
                    Icon { icon: FaPython }
                    "Python"
                }
            }
        }
        section {
            class: "on-the-web",
            h3 { {t!("on-the-web")} },
            ul {
                li {
                    a {
                        href: "https://www.telegram.me/oskarbaran",
                        Icon { icon: BsTelegram } "Oskar Baran"
                    }
                }
                li {
                    a {
                        href: "https://github.com/85oski",
                        Icon { icon: BsGithub } "85oski"
                    }
                }
                li {
                    a {
                        href: "https://www.facebook.com/oskar.baran.01/",
                        Icon { icon: BsFacebook } "Oskar Baran"
                    }
                }
                li {
                    a {
                        href: "https://open.spotify.com/user/31rj4uuz6etfsiwo35izmfmamepe?si=785bfdbd008540cf",
                        Icon { icon: BsSpotify } "Oskar Baran"
                    }
                }
                li {
                    a {
                        href: "https://www.instagram.com/85oski/",
                        Icon { icon: BsInstagram } "85oski"
                    }
                }
            }
        }
    }
}
