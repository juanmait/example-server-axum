use tracing::{debug, debug_span, event, instrument, Level};
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
#[instrument]
async fn main() {
    init_tracing();

    let _span = debug_span!("main").entered();
    debug!("hello");
    event!(Level::DEBUG, "something happened inside my_span");
    foo();
}

fn init_tracing() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();
}

#[instrument]
fn foo() {
    let foo = 32;
    debug!(foo, "I'm foo");
}
