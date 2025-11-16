pub use chrono;
pub use tracing;
pub use tracing_subscriber;

#[derive(Debug, Clone)]
pub enum LogOption {
    // Log levels
    Trace,
    Debug,
    Info,
    Warn,
    Error,

    // Common framework filters
    RocketOff,
    ActixOff,
    ActixWarn,
    HyperOff,
    HyperWarn,
    TokioOff,
    TokioWarn,

    // Console control
    Console,
    NoConsole,

    // Custom filter for anything else
    Custom(String),
}

impl LogOption {
    pub fn apply(&self, level: &mut String, filters: &mut Vec<String>, console: &mut bool) {
        match self {
            LogOption::Trace => *level = "trace".to_string(),
            LogOption::Debug => *level = "debug".to_string(),
            LogOption::Info => *level = "info".to_string(),
            LogOption::Warn => *level = "warn".to_string(),
            LogOption::Error => *level = "error".to_string(),

            LogOption::RocketOff => filters.push("rocket=off".to_string()),
            LogOption::ActixOff => filters.push("actix_web=off".to_string()),
            LogOption::ActixWarn => filters.push("actix_web=warn".to_string()),
            LogOption::HyperOff => filters.push("hyper=off".to_string()),
            LogOption::HyperWarn => filters.push("hyper=warn".to_string()),
            LogOption::TokioOff => filters.push("tokio=off".to_string()),
            LogOption::TokioWarn => filters.push("tokio=warn".to_string()),

            LogOption::Console => *console = true,
            LogOption::NoConsole => *console = false,

            LogOption::Custom(filter) => filters.push(filter.clone()),
        }
    }
}

#[macro_export]
macro_rules! init_logging {
    ($file_path:expr, $service:expr, $component:expr, $options:expr) => {{
        use std::sync::Once;
        static INIT: Once = Once::new();

        INIT.call_once(|| {
            use std::fs::OpenOptions;
            use $crate::tracing_subscriber::prelude::*;
            use $crate::tracing_subscriber::{EnvFilter, fmt};

            let mut level = "info".to_string(); // default
            let mut filters = vec![];
            let mut console = true; // default

            for option in $options {
                option.apply(&mut level, &mut filters, &mut console);
            }

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

            let mut filter = EnvFilter::from_default_env().add_directive(
                level
                    .parse()
                    .unwrap_or_else(|e| panic!("Invalid log level '{}': {}", level, e)),
            );

            if !filters.is_empty() {
                let filter_str = filters.join(",");
                for filter_directive in filter_str.split(',') {
                    if !filter_directive.trim().is_empty() {
                        filter = filter.add_directive(
                            filter_directive.trim().parse().unwrap_or_else(|e| {
                                panic!("Invalid filter directive '{}': {}", filter_directive, e)
                            }),
                        );
                    }
                }
            }

            if console {
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
        init_logging!("/tmp/test.log", "test", "component", &[LogOption::Info]);
        app_log!(info, "Test log message");
    }

    #[test]
    fn test_init_logging_no_console() {
        init_logging!(
            "/tmp/test_no_console.log",
            "test",
            "component",
            &[LogOption::Info, LogOption::NoConsole]
        );
        app_log!(warn, "Test warning");
    }

    #[test]
    fn test_logging_with_filters() {
        init_logging!(
            "/tmp/test_filters.log",
            "test",
            "component",
            &[LogOption::Debug, LogOption::RocketOff]
        );
        app_log!(debug, "Testing with filters");
    }
}
