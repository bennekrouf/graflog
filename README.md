# graflog

Structured JSON logging for Rust applications optimized for Grafana ingestion.

## Features

- JSON-only structured logging
- Grafana-optimized field naming
- Thread-safe file appending
- Zero-configuration setup
- Consistent timestamp formatting
- Service and component tagging

## Installation
```toml
[dependencies]
graflog = "0.1"
```

## Usage
```rust
use graflog::{init_logging, app_log};

fn main() {
    // Initialize logging - crashes if file path invalid
    init_logging!("/var/log/myapp.log", "payment-service", "api");
    
    // Log with automatic service/component tags
    app_log!(info, "Server started on port 8080");
    app_log!(error, "Database connection failed", error_code = 500);
    
    // Log with custom service/component
    app_log!(debug, "user-service", "auth", "Login attempt", user_id = 12345);
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
```

## Command Line Usage

Pass log file path at startup:
```bash
cargo run -- --log-file /var/log/myapp.log
```

No environment files needed - all parameters passed directly.
