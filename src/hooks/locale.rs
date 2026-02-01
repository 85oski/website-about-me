use std::str::FromStr;

use dioxus::prelude::*;
use dioxus_i18n::{
    prelude::{use_init_i18n, I18nConfig},
    unic_langid::{langid, LanguageIdentifier},
};
use dioxus_sdk::storage::{use_synced_storage, LocalStorage};

pub fn use_init_locale() -> Signal<String> {
    let fallback = langid!("pl-PL");

    let locale =
        use_synced_storage::<LocalStorage, String>("locale".to_string(), || fallback.to_string());

    let mut i18n = use_init_i18n(|| {
        I18nConfig::new(fallback)
            .with_locale((langid!("en-US"), include_str!("../locales/en-US.ftl")))
            .with_locale((langid!("pl-PL"), include_str!("../locales/pl-PL.ftl")))
    });

    use_effect(move || {
        let lang = locale.read();

        if lang.is_empty() {
            return
        }

        let Ok(lang_id) = LanguageIdentifier::from_str(&lang) else {
            return
        };

        let document_element = web_sys::window()
            .and_then(|window| window.document())
            .and_then(|document| document.document_element());

        document_element.inspect(|document| {
            let value = lang_id.language.as_str();
            let _ = document.set_attribute("lang", value);
        });

        i18n.set_language(lang_id);
    });

    use_context_provider(|| locale)
}

pub fn use_locale() -> Signal<String> {
    consume_context()
}
