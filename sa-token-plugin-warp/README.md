# sa-token-plugin-warp

Warp framework integration for sa-token-rust.

## Features

- ⚡ **Filter-based**: Built for Warp's filter system
- 🎯 **Composable**: Easy to combine with other filters
- 🔧 **Flexible**: Type-safe authentication
- 🛡️ **Complete**: Full auth support

## Installation

```toml
[dependencies]
sa-token-plugin-warp = "0.1.2"
sa-token-core = "0.1.2"
warp = "0.3"
```

## Quick Start

```rust
use warp::Filter;
use sa_token_plugin_warp::{SaTokenState, sa_token_filter};
use sa_token_storage_memory::MemoryStorage;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let state = SaTokenState::builder()
        .storage(Arc::new(MemoryStorage::new()))
        .timeout(7200)
        .build();
    
    let routes = warp::path("api")
        .and(warp::path("user"))
        .and(sa_token_filter(state))
        .and_then(user_handler);
    
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
```

## Author

**金书记**

## License

Licensed under either of Apache-2.0 or MIT.
