#[path = "app.rs"]
mod app;
pub use app::{App, Route};

#[path = "layout.rs"]
mod layout;

#[path = "views"]
mod views {
    mod index;
    pub use index::Index;

    mod about_me;
    pub use about_me::AboutMe;

    mod not_found;
    pub use not_found::NotFound;
}

#[path = "components"]
mod components {
    mod navbar;
    pub use navbar::Navbar;
}

#[path = "hooks"]
mod hooks {
    mod scheme;
    pub use scheme::*;

    mod locale;
    pub use locale::*;
}
