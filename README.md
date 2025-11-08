# graflog

Structured JSON logging optimized for Grafana ingestion. And abstraction boring tracing subscriber syntax, by the way.

## Features

- JSON-only structured logging
- Grafana-optimized field naming
- Thread-safe file appending
- Zero-configuration setup
- Consistent timestamp formatting
- Service and component tagging
- Distributed tracing with spans

## Installation
```bash
cargo add graflog
```

## Usage
```rust
use graflog::{init_logging, app_log, app_span};

fn main() {
    // Initialize logging - crashes if file path invalid
    init_logging!("/var/log/myapp.log", "payment-service", "api", "info");

    // Disable console output
    // init_logging!("/var/log/myapp.log", "payment-service", "api", "info", false);i
 
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
    
    // Custom service/component span
    let auth_span = app_span!(
        "validate_token",
        "auth-service",
        "jwt",
        token_id = %token_id
    );
}
```

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

## Command Line Usage

Pass log file path at startup:
```bash

cargo run -- --log-file /var/log/myapp.log --log-level info

```

No environment files needed - all parameters passed directly.

Supported log levels: trace, debug, info, warn, error

Supports complex filters: `"info"`, `"debug,rocket::server=off"`, `"trace,hyper=warn"`Retry

Logs output to both file and console by default. Pass `false` as last parameter to disable console output.Retry

