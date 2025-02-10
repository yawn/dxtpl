use dioxus::prelude::*;
use frontend::Route;

#[cfg(feature = "server")]
fn main() {
    backend::run(App);
}

#[cfg(not(feature = "server"))]
fn main() {
    frontend::run(App);
}

#[component]
fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
