# Server Axum

This is just a simple example of a HTTPS server written in Rust using the axum framework.

-   It have tracing instrumentation.
-   It implements basic in-memory sessions based on cookies.
-   It serves public static files.
-   It uses RustTLS for https which enables HTTP2 protocol (axum flag `http2` must be enabled).
-   It supports brotli compression

```bash
# run the server
cargo run
# run the server in watch mode
cargo watch -q -c -w src/ -x run
# enable tower_http request/response tracing
RUST_LOG=server_axum=debug,tower_http=debug cargo run
# enable tower_http request/response tracing and run the server in watch mode
RUST_LOG=server_axum=debug,tower_http=debug cargo watch -q -c -w src/ -w public/index.html -x run
```

> Note: you need to `cargo install cargo-watch` to use the _watch_ mode.

## TODOs

-   handle session not found for cookie found (BUG).
-   upgrade versions of axum, tower-http and axum-server.

## Notes

### Trait axum::extract::FromRequest

-   Things that can be created from a requests.
-   `FromRequest` is generic over the request body.

### axum::response::IntoResponse

-   A response can be created from any type that implements `IntoResponse`.
-   Types that implement IntoResponse can be returned from handlers.

## Read

-   [Async Session](https://docs.rs/async-session/latest/async_session/)
-   [Axum Feature Flags](https://docs.rs/axum/latest/axum/index.html#feature-flags)
-   [Axum Examples](https://github.com/tokio-rs/axum/tree/main/examples)
-   [Tracing](https://docs.rs/tracing/latest/tracing/)
-   [Tracing Examples](https://github.com/tokio-rs/tracing/tree/master/examples)
-   [Tracing Subscriber](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/)
-   [Tower HTTP Trace](https://docs.rs/tower-http/latest/tower_http/trace/index.html)
-   [Env Logger](https://docs.rs/env_logger/latest/env_logger/)
-   [Log](https://docs.rs/log/latest/log/)
-   [Rust Unofficial Patterns](https://rust-unofficial.github.io/patterns/)

# Versioning

```bash
git tag v0.1.1
git push
git push --tags
```
