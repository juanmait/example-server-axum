use axum::{
    http::header::{HeaderMap, SET_COOKIE},
    response::{Html, IntoResponse},
};

use tracing::{debug, instrument};

use crate::session::UserIdFromSession;

const HTML: &str = include_str!("../public/index.html");

// https://docs.rs/axum/latest/axum/handler/index.html
// In axum a “handler” is an async function that accepts zero or more “extractors” as arguments and
// returns something that can be converted into a response.
// For a function to be used as a handler it must implement the Handler trait.
// https://docs.rs/axum/latest/axum/handler/trait.Handler.html
// axum provide blanket implementations of for functions that
// - Are async fns.
// - Take no more than 16 arguments that all implement FromRequest.
// - Returns something that implements IntoResponse.
// - If a closure is used it must implement Clone + Send and be 'static.
// - Returns a future that is Send. The most common way to accidentally make a future !Send is to
// hold a !Send type across an await.
#[instrument]
pub async fn handler(user_id: UserIdFromSession) -> impl IntoResponse {
    let (headers, _user_id, _is_cookie_created) = match user_id {
        UserIdFromSession::FoundUserId(user_id) => (HeaderMap::new(), user_id, false),
        UserIdFromSession::CreatedFreshUserId(new_user) => {
            let mut headers = HeaderMap::new();
            headers.insert(SET_COOKIE, new_user.cookie);
            (headers, new_user.user_id, true)
        }
    };

    debug!("sending html");

    other_fn();

    (headers, Html(HTML))
}

#[instrument]
fn other_fn() {
    let foo = 32;
    debug!(foo, "I'm foo");
}
