use tracing_subscriber::fmt;
use tracing_subscriber::fmt::time::{OffsetTime, UtcTime};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::config::Settings;

fn init_detailed_tracing() {
    let settings = Settings::get();
    let log = &settings.logger;

    // Create the environment filter based on the log level
    let env_filter = tracing_subscriber::EnvFilter::new(log.level.clone());

    let timer = time::format_description::parse(
        "[year]-[month padding:zero]-[day padding:zero] [hour]:[minute]:[second]",
    )
    .expect("Failed to parse time format description");
    let time_offset =
        time::UtcOffset::current_local_offset().unwrap_or_else(|_| time::UtcOffset::UTC);
    let offset_timer = OffsetTime::new(time_offset, timer);

    // Configure the formatter based on the desired log format (pretty, json, or compact)
    let fmt_layer = match log.format.as_str() {
        "json" => fmt::layer()
            .json()
            .with_timer(offset_timer.clone()) // Use OffsetTime for JSON format
            .with_thread_ids(true)
            .with_current_span(true)
            .with_span_events(fmt::format::FmtSpan::FULL),
        "compact" => fmt::layer()
            .compact()
            .with_timer(UtcTime::rfc_3339()) // Use UtcTime for Compact format
            .with_thread_ids(true)
            .with_span_events(fmt::format::FmtSpan::FULL),
        _ => fmt::layer()
            .pretty() // Default to pretty if format is unknown
            .with_timer(offset_timer)
            .with_thread_ids(true)
            .with_span_events(fmt::format::FmtSpan::FULL),
    };

    // Initialize the tracing subscriber
    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer)
        .init();

    // If pretty backtrace is enabled, set RUST_BACKTRACE=1
    if log.pretty_backtrace {
        std::env::set_var("RUST_BACKTRACE", "1");
    }
}
