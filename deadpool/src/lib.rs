//! # Deadpool for UnReQL
//!
//! This crate implements a [`deadpool`](https://crates.io/crates/deadpool)
//! manager for [`unreql`](https://crates.io/crates/unreql).
//!
//! ## Example
//!
//! ```rust
//! use unreql::{r, cmd::connect};
//! use unreql_deadpool::{IntoPoolWrapper, SessionManager};
//! use deadpool::managed::Pool;
//!
//! # async fn example() -> unreql::Result<()> {
//! let cfg = connect::Options::default();
//! let manager = SessionManager::new(cfg);
//! let pool = Pool::builder(manager).max_size(20).build().unwrap().wrapper();
//! # #[derive(serde::Deserialize)] struct User;
//! let user: User = r.table("users").get("id").exec(&pool).await?;
//! # Ok(()) }
//! ```

use std::ops::Deref;

use async_trait::async_trait;
use deadpool::managed::{self, Pool, PoolError};

use unreql::{
    cmd::{connect, run},
    r, Connection, Error, Session,
};

#[derive(Debug)]
pub struct SessionManager {
    options: connect::Options,
}

impl SessionManager {
    pub fn new(options: connect::Options) -> Self {
        Self { options }
    }

    /// Get a new session outside the pool.
    /// Use the new session to create a connection for changes
    pub async fn new_session(&self) -> Result<Session, Error> {
        r.connect(self.options.clone()).await
    }
}

#[async_trait]
impl managed::Manager for SessionManager {
    type Type = Session;
    type Error = Error;

    async fn create(&self) -> Result<Self::Type, Self::Error> {
        self.new_session().await
    }

    async fn recycle(
        &self,
        conn: &mut Self::Type,
        _: &managed::Metrics,
    ) -> managed::RecycleResult<Error> {
        let _: i64 = r.expr(200).exec(conn).await?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct PoolWrapper(Pool<SessionManager>);

impl Deref for PoolWrapper {
    type Target = Pool<SessionManager>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[async_trait]
impl run::Arg for &PoolWrapper {
    async fn into_run_opts(self, for_changes: bool) -> Result<(Connection, run::Options), Error> {
        if for_changes {
            // for `changes` create a separate new connection to DB
            let sess = self.manager().new_session().await?;
            sess.into_run_opts(for_changes).await
        } else {
            // otherwise the available connection is used
            let sess = match self.get().await {
                Ok(v) => v,
                Err(err) => {
                    return match err {
                        PoolError::Backend(err) => Err(err),
                        _ => Err(Error::Driver(unreql::Driver::Other(err.to_string()))),
                    }
                }
            };
            sess.into_run_opts(for_changes).await
        }
    }
}

pub trait IntoPoolWrapper {
    fn wrapper(self) -> PoolWrapper;
}

impl IntoPoolWrapper for Pool<SessionManager> {
    fn wrapper(self) -> PoolWrapper {
        self.into()
    }
}

impl From<Pool<SessionManager>> for PoolWrapper {
    fn from(pool: Pool<SessionManager>) -> Self {
        Self(pool)
    }
}
