use dioxus::prelude::*;
use dioxus_sdk::{
    storage::{use_synced_storage, LocalStorage},
    window::theme::{get_theme, use_system_theme, Theme as UserTheme},
};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, ops::Not};

const KEY: &str = "theme";

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum Theme {
    Light,
    Dark,
}
impl Theme {
    pub fn apply(&self) {
        let document_element = web_sys::window()
            .and_then(|window| window.document())
            .and_then(|document| document.document_element());

        if let Some(document_element) = document_element {
            let theme = format!("{self}");
            let _ = document_element.set_attribute("data-theme", &theme);
        }
    }
}
impl Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let theme = match self {
            Self::Light => "light",
            Self::Dark => "dark",
        };

        write!(f, "{theme}")
    }
}
impl Not for Theme {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Self::Light => Self::Dark,
            Self::Dark => Self::Light,
        }
    }
}
impl From<UserTheme> for Theme {
    fn from(value: UserTheme) -> Self {
        match value {
            UserTheme::Light => Self::Light,
            UserTheme::Dark => Self::Dark,
        }
    }
}
impl Default for Theme {
    fn default() -> Self {
        get_theme()
            .map(Self::from)
            .unwrap_or(Self::Light)
    }
}

pub fn use_init_theme() -> Signal<Theme> {
    let mut theme = use_synced_storage::<LocalStorage, Theme>(KEY.into(), Theme::default);

    let system_theme = use_system_theme();

    use_effect(move || theme.read().apply());

    use_effect({
        let mut mounted = use_signal(|| false);

        move || {
            let system_theme = system_theme.read();
            if !*mounted.peek() {
                return mounted.set(true);
            }

            if let Ok(system_theme) = *system_theme {
                theme.set(Theme::from(system_theme))
            }
        }
    });

    use_context_provider(|| theme)
}

pub fn use_theme() -> Signal<Theme> {
    consume_context()
}
