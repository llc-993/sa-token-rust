# sa-token-plugin-actix-web

Actix-web framework integration for sa-token-rust.

## Features

- ⚡ **High Performance**: Built for Actix-web 4.x
- 🎯 **Complete Integration**: Middleware, extractors, and more
- 🔧 **Flexible Configuration**: Builder pattern support
- 🛡️ **Production Ready**: Battle-tested in production

## Installation

```toml
[dependencies]
sa-token-plugin-actix-web = "0.1.2"
sa-token-core = "0.1.2"
actix-web = "4.4"
```

## Quick Start

```rust
use actix_web::{web, App, HttpServer};
use sa_token_plugin_actix_web::SaTokenState;
use sa_token_storage_memory::MemoryStorage;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = SaTokenState::builder()
        .storage(Arc::new(MemoryStorage::new()))
        .timeout(7200)
        .build();
    
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/api/user", web::get().to(user_info))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

## Author

**金书记**

## License

Licensed under either of Apache-2.0 or MIT.
