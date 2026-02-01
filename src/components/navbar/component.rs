use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::ld_icons::{LdCode, LdLanguages, LdSunMoon}};

use crate::prelude::{app::Route, hooks::{use_locale, use_theme}};

#[derive(Clone, Copy, PartialEq)]
enum MenuState {
    None,
    Language,
}

#[component]
pub fn Navbar(title: &'static str) -> Element {
    let mut theme = use_theme();
    let mut locale = use_locale();

    let mut menu_state = use_signal(|| MenuState::None);

    let active_class = |state: MenuState| {
    if *menu_state.read() == state {
        "active"
    } else {
        "inactive"
    }
};

    let mut toggle_menu = move |state: MenuState| {
        if *menu_state.read() == state {
            menu_state.set(MenuState::None);
        } else {
            menu_state.set(state);
        }
    };

    rsx! {
        header {
            nav {
                section {
                    Link { to: Route::Index,
                        Icon { icon: LdCode }
                        {title}
                    }
                }
                section {
                    button { onclick: move |_| theme.toggle(),
                        Icon { icon: LdSunMoon }
                    }
                    button { onclick: move |_| toggle_menu(MenuState::Language),
                        Icon { icon: LdLanguages }
                        menu { class: active_class(MenuState::Language),
                            li {
                                button { onclick: move |_| locale.set("pl-PL".into()),
                                    "🇵🇱 Polski"
                                }
                            }
                            li {
                                button { onclick: move |_| locale.set("en-US".into()),
                                    "🇺🇸 English"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}