pub use chrono;
pub use tracing;
pub use tracing_subscriber;

#[macro_export]
macro_rules! init_logging {
    ($file_path:expr, $service:expr, $component:expr, $log_level:expr) => {
        $crate::init_logging!($file_path, $service, $component, $log_level, true)
    };
    ($file_path:expr, $service:expr, $component:expr, $log_level:expr, $console:expr) => {{
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

            let file_layer = fmt::layer()
                .json()
                .with_writer(file)
                .with_current_span(false)
                .with_span_list(false)
                .with_target(false)
                .with_thread_ids(false)
                .with_thread_names(false);

            let filter = EnvFilter::from_default_env()
                .add_directive($log_level.parse().expect("Invalid log directive"));

            if $console {
                let console_layer = fmt::layer()
                    .json()
                    .with_writer(std::io::stdout)
                    .with_current_span(false)
                    .with_span_list(false)
                    .with_target(false)
                    .with_thread_ids(false)
                    .with_thread_names(false);

                $crate::tracing_subscriber::registry()
                    .with(file_layer)
                    .with(console_layer)
                    .with(filter)
                    .try_init()
                    .ok();
            } else {
                $crate::tracing_subscriber::registry()
                    .with(file_layer)
                    .with(filter)
                    .try_init()
                    .ok();
            }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_logging() {
        let _ = init_logging!("/tmp/test.log", "test", "component", "info");
        app_log!(info, "Test log message");
    }

    #[test]
    fn test_init_logging_no_console() {
        let _ = init_logging!(
            "/tmp/test_no_console.log",
            "test",
            "component",
            "info",
            false
        );
        app_log!(warn, "Test warning");
    }
}

#[test]
fn test_complex_filter_directives() {
    use std::sync::Once;
    static INIT: Once = Once::new();

    INIT.call_once(|| {
        // Test only target-specific directives (no global level)
        init_logging!(
            "/tmp/test_complex.log",
            "test",
            "component",
            "graflog=debug"
        );
    });

    app_log!(warn, "Testing complex filter directive");
}
