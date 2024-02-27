use std::env;

pub fn connect_opts() -> unreql::cmd::connect::Options {
    let mut opts = unreql::cmd::connect::Options::default();
    if let Ok(host) = env::var("RDB_HOST") {
        opts = opts.host(host);
    }
    if let Ok(port) = env::var("RDB_PORT") {
        opts = opts.port(port.parse().unwrap());
    }
    if let Ok(db) = env::var("RDB_DB") {
        opts = opts.db(db);
    }
    opts
}
