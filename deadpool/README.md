# Deadpool for UnReQL

This crate implements a [`deadpool`](https://crates.io/crates/deadpool)
manager for [`unreql`](https://crates.io/crates/unreql).

## Example

```rust
use unreql::{r, cmd::connect};
use unreql_deadpool::{IntoPoolWrapper, SessionManager};
use deadpool::managed::Pool;

let cfg = connect::Options::default();
let manager = SessionManager::new(cfg);
let pool = Pool::builder(manager).max_size(20).build().unwrap().wrapper();
let user: User = r.table("users").get("id").exec(&pool).await?;
```
