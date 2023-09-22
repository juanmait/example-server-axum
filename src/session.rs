use async_session::{async_trait, MemoryStore, Session, SessionStore as _};
use axum::{
    extract::{FromRequest, RequestParts},
    http::{HeaderValue, StatusCode},
    Extension,
};
use serde::{Deserialize, Serialize};
use tracing::debug;
use uuid::Uuid;

const AXUM_SESSION_COOKIE_NAME: &str = "axum_session";

#[derive(Serialize, Deserialize, Debug)]
pub struct UserId(pub Uuid);

impl UserId {
    fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Debug)]
pub struct FreshUserId {
    pub user_id: UserId,
    pub cookie: HeaderValue,
}

#[derive(Debug)]
pub enum UserIdFromSession {
    FoundUserId(UserId),
    CreatedFreshUserId(FreshUserId),
}
// Convert UserIdFromSession into an extractor by implementing `FromRequest`.
// See https://docs.rs/axum/latest/axum/extract/index.html#defining-custom-extractors
// `FromRequest` is an extractor. Implementing `FromRequest` for `UserIdFromSession`
// will allow us to use it as the parameter to a handler.
// Remainder: In axum a “handler” is an async function that accepts zero or more “extractors”
// as arguments and returns something that can be converted into a response. A handler must implement
// the Handler trait
#[async_trait]
impl<B> FromRequest<B> for UserIdFromSession
where
    B: Send,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        // extract the Extension MemoryStore from the request
        // see https://docs.rs/axum/latest/axum/extract/index.html#accessing-other-extractors-in-fromrequest-implementations
        let Extension(store) = Extension::<MemoryStore>::from_request(req)
            .await
            .expect("missing `MemoryStore` extension");

        // Option implements FromRequest
        // https://docs.rs/axum-core/latest/src/axum_core/extract/mod.rs.html#258-260
        // extract the cookie from the request headers.
        // See optional extractors at https://docs.rs/axum/latest/axum/extract/index.html#optional-extractors
        let cookie = Option::<axum::TypedHeader<axum::headers::Cookie>>::from_request(req)
            .await
            .unwrap();

        let session_cookie = cookie
            .as_ref()
            .and_then(|cookie| cookie.get(AXUM_SESSION_COOKIE_NAME));

        // return the new created session cookie for client
        if session_cookie.is_none() {
            let user_id = UserId::new();
            let mut session = Session::new();
            session.insert("user_id", &user_id).unwrap();
            let cookie = store.store_session(session).await.unwrap().unwrap();
            return Ok(Self::CreatedFreshUserId(FreshUserId {
                user_id,
                cookie: HeaderValue::from_str(
                    format!("{}={}", AXUM_SESSION_COOKIE_NAME, cookie).as_str(),
                )
                .unwrap(),
            }));
        }

        debug!(
            "Got session cookie from user agent, {}={}",
            AXUM_SESSION_COOKIE_NAME,
            session_cookie.unwrap()
        );

        // continue to decode the session cookie
        let user_id = if let Some(session) = store
            .load_session(session_cookie.unwrap().to_owned())
            .await
            .unwrap()
        {
            if let Some(user_id) = session.get::<UserId>("user_id") {
                debug!("Session decoded success, user_id={:?}", user_id);
                user_id
            } else {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "No `user_id` found in session",
                ));
            }
        } else {
            debug!(
                "Error session not exists in store, {}={}",
                AXUM_SESSION_COOKIE_NAME,
                session_cookie.unwrap()
            );
            return Err((StatusCode::BAD_REQUEST, "No session found for cookie"));
        };

        Ok(Self::FoundUserId(user_id))
    }
}
