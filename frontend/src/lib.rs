use dioxus::prelude::*;

pub mod prelude;

pub fn run(app: fn() -> Element) {
    // TODO: replace this with LaunchBuilder
    dioxus::fullstack::prelude::server_fn::client::set_server_url("http://127.0.0.1:8080");
    dioxus::launch(app);
}

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},
}

#[component]
fn Home() -> Element {
    rsx! {
        div {
            Echo { text: "hello world".to_string() }
        }
    }
}

#[component]
fn Echo(text: String) -> Element {
    let echo = use_resource(move || {
        let text = text.clone();
        async move { api::echo(text).await }
    });

    match &*echo.read_unchecked() {
        Some(Ok(res)) => rsx! {
            div { {res.clone()} }
        },
        Some(Err(_)) => rsx! {
            div { "Loading failed" }
        },
        None => rsx! {
            div { "Loading ..." }
        },
    }
}
