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
unreql = "0.1.3"
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

```rust
let query = r.table("users").get(1).run(&conn);
let user: Option<User> = query.try_next().await?;
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
