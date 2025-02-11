# Unofficial RethinkDB Driver for Rust

Well documented and easy to use

[<img alt="github" src="https://img.shields.io/badge/github-vettich/un--rethinkdb--rs-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/vettich/un-rethinkdb-rs)
[<img alt="crates.io" src="https://img.shields.io/crates/v/unreql.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/unreql)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-unreql-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/unreql)

## Motivation

The official driver is difficult to support, awkward to use, and has little to no documentation or examples. Therefore, an attempt was made by me to remedy these shortcomings

## Install

```bash
$ cargo add unreql
```

or

```toml
[dependencies]
unreql = "0.2"
```

## Import

```rust
use unreql::r;
```

## Connect

```rust
let conn = r.connect(()).await?;
```

## Get data

Get by ID

```rust
let user: User = r.table("users").get(1).exec(&conn).await?;
```

Get all data

```rust
let users: Vec<User> = r.table("users").exec_to_vec(&conn).await?;
```

or

```rust
let mut cur = r.table("users").run(&conn);
let mut users: Vec<User> = vec![];
while let Ok(Some(user)) = cur.try_next().await? {
    users.push(user);
}
```

## Update data

Use a nested reql query

```rust
r.table("users")
  .get(1)
  .update(rjson!({
    "name": "John",
    "upd_count": r.row().g("upd_count").add(1),
  }))
  .run(&conn);
```

## Use connection pool

Implemented session manager for async `deadpool`

```toml
[dependencies]
unreql = "0.2"
unreql_deadpool = "0.2"
deadpool = "0.12"
```

```rust
use unreql::{r, cmd::connect};
use unreql_deadpool::{IntoPoolWrapper, SessionManager};
use deadpool::managed::Pool;

// config to connect to rethinkdb
let config = connect::Options::default();
// new session manager
let manager = SessionManager::new(config);
// create a pool that is wrapped for ease of use (to be able to be passed to `.run(&pool)`)
let pool = Pool::builder(manager).max_size(20).build().unwrap().wrapper();

// now you can to pass `pool` to `.run()` and `.exec()`
let user: User = r.table("users").get(1).exec(&pool).await?;
```
