#[path = "app/mod.rs"]
pub mod app;

#[path = "hooks"]
pub mod hooks {
    mod theme;

    pub use theme::{use_init_theme, use_theme};

    mod locale;
    pub use locale::{use_init_locale, use_locale};
}

#[path = "pages"]
pub mod pages {
    mod index;
    pub use index::Index;

    mod not_found;
    pub use not_found::NotFound;
}

#[path = "components"]
pub mod components {
    mod navbar;
    pub use navbar::*;

    mod section;
    pub use section::*;

    mod profile;
    pub use profile::*;

    mod tech_stack;
    pub use tech_stack::*;

    mod on_the_web;
    pub use on_the_web::*;
}