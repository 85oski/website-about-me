pub mod prelude;

fn main() {
    use prelude::app::App;

    dioxus::launch(App);
}