use tower_http::{
    classify::SharedClassifier,
    trace::{DefaultMakeSpan, TraceLayer},
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub struct AppLogger;

impl AppLogger {
    pub fn setup_logger() {
        tracing_subscriber::registry()
            .with(
                tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                    format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
                }),
            )
            .with(tracing_subscriber::fmt::layer())
            .init();
    }

    pub fn get_trace_layer(
    ) -> TraceLayer<SharedClassifier<tower_http::classify::ServerErrorsAsFailures>> {
        TraceLayer::new_for_http().make_span_with(DefaultMakeSpan::default().include_headers(true))
    }
}
