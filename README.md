# graflog

Structured JSON logging optimized for Grafana ingestion. Clean enum-based configuration.

## Features

- JSON-only structured logging
- Grafana-optimized field naming
- Thread-safe file appending
- Zero-configuration setup
- Consistent timestamp formatting
- Service and component tagging
- Distributed tracing with spans
- Clean enum-based configuration

## Installation

```bash
cargo add graflog
```

## Usage

```rust
use graflog::{init_logging, app_log, app_span, LogOption};

fn main() {
    // Simple logging
    init_logging!(&log_path, "payment-service", "api", &[LogOption::Debug]);
    
    // Silent rocket with console disabled
    init_logging!(&log_path, "api0", "uploader", &[
        LogOption::Debug,
        LogOption::RocketOff,
        LogOption::NoConsole
    ]);
    
    // Multiple filters in any order
    init_logging!(&log_path, "service", "component", &[
        LogOption::ActixWarn,
        LogOption::Info,
        LogOption::HyperOff
    ]);
    
    // Custom filter
    init_logging!(&log_path, "service", "component", &[
        LogOption::Debug,
        LogOption::Custom("my_crate::module=warn".to_string())
    ]);
    
    // Log with automatic service/component tags
    app_log!(info, "Server started on port 8080");
    app_log!(error, "Database connection failed", error_code = 500);
    
    // Log with custom service/component
    app_log!(debug, "user-service", "auth", "Login attempt", user_id = 12345);
    
    // Create spans for distributed tracing
    let process_span = app_span!(
        "process_payment",
        user_id = %user_id,
        amount = amount
    );
    let _enter = process_span.enter();
}
```

## Configuration Options

### Log Levels
- `LogOption::Trace`
- `LogOption::Debug` 
- `LogOption::Info`
- `LogOption::Warn`
- `LogOption::Error`

### Framework Filters
- `LogOption::RocketOff` - Turn off rocket logs
- `LogOption::ActixOff` / `LogOption::ActixWarn` - Control actix_web logs
- `LogOption::HyperOff` / `LogOption::HyperWarn` - Control hyper logs  
- `LogOption::TokioOff` / `LogOption::TokioWarn` - Control tokio logs

### Console Control
- `LogOption::Console` - Enable console output (default)
- `LogOption::NoConsole` - Disable console output

### Custom Filters
- `LogOption::Custom("target=level".to_string())` - Any custom filter

## Grafana Integration

Logs are formatted with consistent fields:
- `service`: Service identifier
- `component`: Component within service  
- `timestamp`: RFC3339 formatted timestamp
- `level`: Log level (trace, debug, info, warn, error)
- Custom fields from your log statements

Perfect for Grafana Loki queries:
```logql
{service="payment-service"} | json | level="error"
{service="auth-service"} | json | component="jwt"
```

## Migration from Previous Versions

Old syntax:
```rust
init_logging!(&log_path, "service", "component", "debug", "rocket::server=off", false);
```

New syntax:
```rust
init_logging!(&log_path, "service", "component", &[
    LogOption::Debug,
    LogOption::RocketOff, 
    LogOption::NoConsole
]);
```
