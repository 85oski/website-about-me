use dioxus::prelude::*;
use dioxus_sdk::{
    storage::{LocalStorage, use_synced_storage},
    theme::{SystemTheme, get_system_theme, use_system_theme},
};
use serde::{Deserialize, Serialize};
use std::ops::Not;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum Scheme {
    Light,
    Dark,
}
impl Scheme {
    pub fn apply(&self) {
        let scheme = format!("{self:?}").to_lowercase();
        let js = format!("document.body.setAttribute('data-scheme', '{scheme}')");

        document::eval(js.as_str());
    }
}
impl Not for Scheme {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Self::Light => Self::Dark,
            Self::Dark => Self::Light,
        }
    }
}
impl From<SystemTheme> for Scheme {
    fn from(value: SystemTheme) -> Self {
        match value {
            SystemTheme::Light => Self::Light,
            SystemTheme::Dark => Self::Dark,
        }
    }
}
impl Default for Scheme {
    fn default() -> Self {
        get_system_theme()
            .map(Self::from)
            .unwrap_or(Self::Light)
    }
}

pub fn use_init_scheme() -> Signal<Scheme> {
    let mut scheme = use_synced_storage::<LocalStorage, Scheme>(
        "data-scheme".to_string(),
        Scheme::default
    );

    let system_theme = use_system_theme();

    use_effect(move || scheme.read().apply());

    use_effect({
        let mut mounted = use_signal(|| false);
        
        move || {
            let system_theme = system_theme.read();
            if !*mounted.peek() {
                return mounted.set(true);
            }

            if let Ok(theme) = *system_theme {
                scheme.set(Scheme::from(theme))
            }
        }
    });

    use_context_provider(|| scheme)
}

pub fn use_scheme() -> Signal<Scheme> {
    consume_context()
}
