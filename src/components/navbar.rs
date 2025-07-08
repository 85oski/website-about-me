use crate::prelude::{
    Route,
    hooks::{use_locale, use_scheme},
};
use dioxus::prelude::*;
use dioxus_free_icons::{
    Icon,
    icons::{md_action_icons::*, md_navigation_icons::*},
};
use dioxus_i18n::tid;

#[component]
pub fn Navbar() -> Element {
    let mut scheme = use_scheme();
    let mut locale = use_locale();

    let class_from = use_callback(move |signal: Signal<bool>| {
        return if *signal.read() { Some("active") } else { None };
    });

    let mut language_menu = use_signal(|| false);
    let mut categories_menu = use_signal(|| false);

    rsx! {
        header {
            nav {
                section {
                    Icon { icon: MdCode } "Oskar Baran"
                }
                section {
                    button {
                        onclick: move |_| scheme.toggle(),
                        Icon { icon: MdInvertColors }
                    }
                    button {
                        onclick: move |_| {
                            categories_menu.set(false);
                            language_menu.toggle()
                        },
                        Icon { icon: MdTranslate }
                        menu {
                            class: class_from(language_menu),
                            li {
                                button {
                                    onclick: move |_| locale.set("pl".to_string()),
                                    "Polski"
                                }
                            }
                            li {
                                button {
                                    onclick: move |_| locale.set("en".to_string()),
                                    "English"
                                }
                            }
                        }
                    }
                    button {
                        onclick: move |_| {
                            language_menu.set(false);
                            categories_menu.toggle()
                        },
                        Icon { icon: MdMenu }
                        menu {
                            class: class_from(categories_menu),
                            li {
                                Link {
                                    to: Route::Index {  },
                                    Icon { icon: MdHome }
                                    {tid!("categories.main-page")}
                                }
                            }
                            li {
                                Link {
                                    to: Route::AboutMe {},
                                    Icon { icon: MdAccountCircle }
                                    {tid!("categories.about-me")}
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
