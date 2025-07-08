use std::str::FromStr;

use dioxus::prelude::*;
use dioxus_i18n::{prelude::{use_init_i18n, I18nConfig}, unic_langid::{langid, LanguageIdentifier}};
use dioxus_sdk::storage::{use_synced_storage, LocalStorage};


pub fn use_init_locale() -> Signal<String> {
    let locale = use_synced_storage::<LocalStorage, String>(
        "locale".to_string(),
        || "pl".to_string()
    );

    let mut i18n = use_init_i18n(
        || I18nConfig::new(langid!("en"))
        .with_locale((langid!("en"), include_str!("../locales/en.ftl")))
        .with_locale((langid!("pl"), include_str!("../locales/pl.ftl")))
    );

    use_effect(move || {
        let lang = locale.read();
        let Ok(lang_id) = LanguageIdentifier::from_str(lang.as_str()) else {
            return
        };

        i18n.set_language(lang_id);
    });

    use_context_provider(|| locale)
}

pub fn use_locale() -> Signal<String> {
    consume_context()
}