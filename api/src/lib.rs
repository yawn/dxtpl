use dioxus::{logger::tracing::debug, prelude::*};
use shared::Session;

#[cfg(feature = "backend")]
use backend::prelude::*;

#[server]
pub async fn echo(req: String) -> Result<String, ServerFnError> {
    debug!("calling echo attempt");

    let session = Session {
        id: 123,
        username: "foo".to_string(),
    };

    let _ = session.do_something_in_database(123).unwrap(); // TODO: show error handling

    Ok(req)
}
