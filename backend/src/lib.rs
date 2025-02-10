use anyhow::Result;
use axum::{
    extract::Request,
    http::StatusCode,
    middleware::{from_fn, Next},
    response::{Html, Response},
    routing::get,
    Router,
};
use dioxus::{logger::tracing::debug, prelude::*};
use tokio::net::TcpListener;

pub mod prelude;
mod session;

pub fn run(app: fn() -> Element) {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            dioxus::logger::initialize_default();

            debug!("running backend configuration");

            let builder = ServeConfigBuilder::new();

            let service = Router::new()
                .serve_dioxus_application(builder, app)
                .route("/test", get(handler))
                .layer(from_fn(middleware))
                .into_make_service();

            let address = dioxus::cli_config::fullstack_address_or_localhost();
            let listener = TcpListener::bind(address)
                .await
                .unwrap_or_else(|err| panic!("failed to bind to address {}: {}", address, err));

            axum::serve(listener, service)
                .await
                .expect("failed to start server");
        })
}

async fn middleware(req: Request, next: Next) -> Result<Response, StatusCode> {
    debug!("doing something in an axum middleware");

    let next = next.run(req).await;

    Ok(next)
}

async fn handler(_req: Request) -> axum::response::Html<String> {
    Html("hello".into())
}
