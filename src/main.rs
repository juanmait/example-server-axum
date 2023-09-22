use crate::handler::handler;
use async_session::MemoryStore;
use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, get_service},
    Extension, Router,
};
use axum_server::tls_rustls::RustlsConfig;
use std::{io, net::SocketAddr};
use tower_http::{
    compression::{predicate::SizeAbove, CompressionLayer},
    services::ServeDir,
    trace::TraceLayer,
};
use tracing::{error, info, instrument};
use tracing_subscriber::{prelude::*, EnvFilter};

mod handler;
mod session;

const PORT: u16 = 3030;
const PUB_KEY: &str = "./self_signed_certs/cert.pem";
const PRIV_KEY: &str = "./self_signed_certs/key.pem";

#[tokio::main]
#[instrument]
async fn main() {
    init_tracing();

    tracing::debug!("Loading certificates...");

    let rust_tls_config = RustlsConfig::from_pem_file(PUB_KEY, PRIV_KEY)
        .await
        .unwrap();

    let session_store = MemoryStore::new();

    let app = Router::new()
        .route("/", get(handler))
        .fallback(get_service(ServeDir::new("./public")).handle_error(handle_error))
        // Add middleware to all routes
        .layer(CompressionLayer::new().compress_when(SizeAbove::new(0)))
        // inject the store into the request
        .layer(Extension(session_store))
        .layer(TraceLayer::new_for_http()); // @see https://docs.rs/tower-http/latest/tower_http/trace/index.html

    let addr = SocketAddr::from(([127, 0, 0, 1], PORT));

    info!("listening on https://{}", addr);

    axum_server::bind_rustls(addr, rust_tls_config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handle_error(err: io::Error) -> impl IntoResponse {
    error!("{}", err);
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}

fn init_tracing() {
    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();
}
