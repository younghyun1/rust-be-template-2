use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{Layer, layer::SubscriberExt, util::SubscriberInitExt};

pub fn setup_logging() -> WorkerGuard {
    let app_name_version = format!("{}_v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    let filename = "log";
    let file_appender =
        tracing_appender::rolling::hourly(format!("./logs/{}", app_name_version), filename);
    let (non_blocking_file, guard) = tracing_appender::non_blocking(file_appender);

    let console_layer = tracing_subscriber::fmt::layer()
        .with_ansi(true)
        .with_target(true)
        .with_filter(tracing_subscriber::filter::LevelFilter::INFO);

    let file_layer = tracing_subscriber::fmt::layer()
        .with_target(true)
        .json()
        .with_writer(non_blocking_file)
        .with_filter(tracing_subscriber::filter::LevelFilter::DEBUG);

    tracing_subscriber::registry()
        .with(console_layer)
        .with(file_layer)
        .init();

    guard
}
