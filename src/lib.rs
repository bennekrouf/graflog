pub use chrono;
pub use tracing;
pub use tracing_subscriber;

#[macro_export]
macro_rules! init_logging {
    ($file_path:expr, $service:expr, $component:expr) => {{
        use std::sync::Once;
        static INIT: Once = Once::new();

        INIT.call_once(|| {
            use std::fs::OpenOptions;
            use $crate::tracing_subscriber::prelude::*;
            use $crate::tracing_subscriber::{EnvFilter, fmt};

            let file = OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open($file_path)
                .expect("Failed to open log file");

            $crate::tracing_subscriber::registry()
                .with(
                    fmt::layer()
                        .json()
                        .with_writer(file)
                        .with_current_span(false)
                        .with_span_list(false)
                        .with_target(false)
                        .with_thread_ids(true)
                        .with_thread_names(true),
                )
                .with(
                    EnvFilter::from_default_env()
                        .add_directive("trace".parse().expect("Invalid log directive")),
                )
                .init();
        });
    }};
}

#[macro_export]
macro_rules! app_log {
    ($level:ident, $($arg:tt)*) => {
        $crate::tracing::$level!(
            service = env!("CARGO_PKG_NAME"),
            component = "main",
            timestamp = $crate::chrono::Utc::now().to_rfc3339(),
            $($arg)*
        )
    };
    ($level:ident, $service:expr, $component:expr, $($arg:tt)*) => {
        $crate::tracing::$level!(
            service = $service,
            component = $component,
            timestamp = $crate::chrono::Utc::now().to_rfc3339(),
            $($arg)*
        )
    };
}

#[macro_export]
macro_rules! app_span {
    ($name:expr, $($field:tt)*) => {
        $crate::tracing::info_span!(
            $name,
            service = env!("CARGO_PKG_NAME"),
            component = "main",
            timestamp = $crate::chrono::Utc::now().to_rfc3339(),
            $($field)*
        )
    };
    ($name:expr, $service:expr, $component:expr, $($field:tt)*) => {
        $crate::tracing::info_span!(
            $name,
            service = $service,
            component = $component,
            timestamp = $crate::chrono::Utc::now().to_rfc3339(),
            $($field)*
        )
    };
}
