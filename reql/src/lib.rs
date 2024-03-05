//! # Unofficial RethinkDB Driver for Rust
//!
//! Well documented and easy to use
//!
//! ## Import
//!
//! ```
//! use unreql::r;
//! ```
//!
//! ## Connect
//!
//! ```
//! use unreql::{r, cmd::connect::Options};
//!
//! # async fn example() -> unreql::Result<()> {
//! let conn = r.connect(Options::new().db("marvel")).await?;
//! # Ok(()) }
//! ```
//!
//! ## Get data
//!
//! Get by ID
//!
//! ```
//! # use unreql::r;
//! # #[derive(serde::Deserialize)]
//! # struct User;
//! # async fn example() -> unreql::Result<()> {
//! # let conn = r.connect(()).await?;
//! let user: User = r.table("users").get(1).exec(&conn).await?;
//! # Ok(()) }
//! ```
//!
//! Get all data
//!
//! ```
//! # use unreql::r;
//! # #[derive(serde::Deserialize)]
//! # struct User;
//! # async fn example() -> unreql::Result<()> {
//! # let conn = r.connect(()).await?;
//! let users: Vec<User> = r.table("users").exec_to_vec(&conn).await?;
//! # Ok(()) }
//! ```
//!
//! or
//!
//! ```
//! # use unreql::r;
//! use futures::TryStreamExt;
//!
//! # #[derive(Debug, serde::Deserialize)]
//! # struct User;
//! # async fn example() -> unreql::Result<()> {
//! # let conn = r.connect(()).await?;
//! let mut cur = r.table("users").run::<_, User>(&conn);
//! while let Ok(Some(user)) = cur.try_next().await {
//!   // do something with user
//!   dbg!(user);
//! }
//! # Ok(()) }
//! ```
//!
//! ## Update data
//!
//! ```
//! use unreql::{r, rjson};
//!
//! # async fn example() -> unreql::Result<()> {
//! # let conn = r.connect(()).await?;
//! r.table("users")
//!     .get(1)
//!     .update(rjson!({
//!         "name": "Jonh",
//!         "upd_count": r.row().g("upd_count").add(1),
//!     }))
//!     .run::<_, serde_json::Value>(&conn);
//! # Ok(()) }
//! ```

pub mod cmd;
mod err;
mod proto;
mod tools;

#[macro_use]
mod rjson_macros;

use async_net::TcpStream;
use cmd::args::{Args, ArgsWithOpt};
use cmd::run::Response;
use dashmap::DashMap;
use futures::channel::mpsc::{self, UnboundedReceiver, UnboundedSender};
use futures::lock::Mutex;
use proto::Payload;
use ql2::query::QueryType;
use ql2::response::ResponseType;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::borrow::Cow;
use std::ops::Drop;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use tools::StaticString;
use tracing::trace;

pub use cmd::func::Func;
pub use err::*;
pub use proto::{Command, DateTime, Datum};
pub use unreql_macros::func;

#[doc(hidden)]
pub static VAR_COUNTER: AtomicU64 = AtomicU64::new(1);

#[doc(hidden)]
pub fn var_counter() -> u64 {
    VAR_COUNTER.fetch_add(1, Ordering::SeqCst)
}

#[allow(dead_code)]
#[cfg(test)]
fn current_counter() -> u64 {
    VAR_COUNTER.load(Ordering::SeqCst)
}

/// Custom result returned by various ReQL commands
pub type Result<T> = std::result::Result<T, Error>;

type Sender = UnboundedSender<Result<(ResponseType, Response)>>;
type Receiver = UnboundedReceiver<Result<(ResponseType, Response)>>;

#[derive(Debug)]
struct InnerSession {
    db: Mutex<Cow<'static, str>>,
    stream: Mutex<TcpStream>,
    channels: DashMap<u64, Sender>,
    token: AtomicU64,
    broken: AtomicBool,
    change_feed: AtomicBool,
}

impl InnerSession {
    fn token(&self) -> u64 {
        let token = self
            .token
            .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x| Some(x + 1))
            .unwrap();
        if token == u64::MAX {
            self.mark_broken();
        }
        token
    }

    fn mark_broken(&self) {
        self.broken.store(true, Ordering::SeqCst);
    }

    fn broken(&self) -> Result<()> {
        if self.broken.load(Ordering::SeqCst) {
            return Err(err::Driver::ConnectionBroken.into());
        }
        Ok(())
    }

    fn mark_change_feed(&self) {
        self.change_feed.store(true, Ordering::SeqCst);
    }

    fn unmark_change_feed(&self) {
        self.change_feed.store(false, Ordering::SeqCst);
    }

    fn is_change_feed(&self) -> bool {
        self.change_feed.load(Ordering::SeqCst)
    }

    fn change_feed(&self) -> Result<()> {
        if self.change_feed.load(Ordering::SeqCst) {
            return Err(err::Driver::ConnectionLocked.into());
        }
        Ok(())
    }
}

/// The connection object returned by `r.connect()`
#[derive(Debug, Clone)]
pub struct Session {
    inner: Arc<InnerSession>,
}

impl Session {
    pub fn connection(&self) -> Result<Connection> {
        self.inner.broken()?;
        self.inner.change_feed()?;
        let token = self.inner.token();
        let (tx, rx) = mpsc::unbounded();
        self.inner.channels.insert(token, tx);
        Ok(Connection::new(self.clone(), rx, token))
    }

    /// Change the default database on this connection
    ///
    /// ## Example
    ///
    /// Change the default database so that we don’t need to specify the
    /// database when referencing a table.
    ///
    /// ```
    /// # use unreql::r;
    /// # async fn example() {
    /// let mut conn = r.connect(()).await.unwrap();
    /// conn.use_("marvel").await;
    /// r.table("heroes"); // refers to r.db("marvel").table("heroes")
    /// # }
    /// ```
    ///
    /// ## Related commands
    /// * [connect](r::connect)
    /// * [close](Connection::close)
    pub async fn use_<T>(&mut self, db_name: T)
    where
        T: StaticString,
    {
        *self.inner.db.lock().await = db_name.static_string();
    }

    /// Ensures that previous queries with the `noreply` flag have been
    /// processed by the server
    ///
    /// Note that this guarantee only applies to queries run on the given
    /// connection.
    ///
    /// ## Example
    ///
    /// We have previously run queries with [noreply](cmd::run::Options::noreply())
    /// set to `true`. Now wait until the server has processed them.
    ///
    /// ```
    /// # async fn example() -> unreql::Result<()> {
    /// # let session = unreql::r.connect(()).await?;
    /// session.noreply_wait().await
    /// # }
    /// ```
    ///
    pub async fn noreply_wait(&self) -> Result<()> {
        let mut conn = self.connection()?;
        let payload = Payload(QueryType::NoreplyWait, None, Default::default());
        trace!(
            "waiting for noreply operations to finish; token: {}",
            conn.token
        );
        let (typ, _) = conn.request(&payload, false).await?;
        trace!(
            "session.noreply_wait() run; token: {}, response type: {:?}",
            conn.token,
            typ,
        );
        Ok(())
    }

    pub async fn server(&self) -> Result<ServerInfo> {
        let mut conn = self.connection()?;
        let payload = Payload(QueryType::ServerInfo, None, Default::default());
        trace!("retrieving server information; token: {}", conn.token);
        let (typ, resp) = conn.request(&payload, false).await?;
        trace!(
            "session.server() run; token: {}, response type: {:?}",
            conn.token,
            typ,
        );
        let mut vec = serde_json::from_value::<Vec<ServerInfo>>(resp.r)?;
        let info = vec
            .pop()
            .ok_or_else(|| Driver::Other("server info is empty".into()))?;
        Ok(info)
    }

    #[doc(hidden)]
    pub fn is_broken(&self) -> bool {
        self.inner.broken.load(Ordering::SeqCst)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[non_exhaustive]
pub struct ServerInfo {
    pub id: String,
    pub proxy: bool,
    pub name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Connection {
    session: Session,
    rx: Arc<Mutex<Receiver>>,
    token: u64,
    closed: Arc<AtomicBool>,
}

impl Connection {
    fn new(session: Session, rx: Receiver, token: u64) -> Connection {
        Connection {
            session,
            token,
            rx: Arc::new(Mutex::new(rx)),
            closed: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Close an open connection
    ///
    /// ## Example
    ///
    /// Close an open connection, waiting for noreply writes to finish.
    ///
    /// ```
    /// # async fn example() -> unreql::Result<()> {
    /// # let session = unreql::r.connect(()).await?;
    /// # let mut conn = session.connection()?;
    /// conn.close(()).await
    /// # }
    /// ```
    ///
    /// [Read more about this command →](cmd::close)
    pub async fn close<T>(&mut self, arg: T) -> Result<()>
    where
        T: cmd::close::Arg,
    {
        if !self.session.inner.is_change_feed() {
            trace!(
                "ignoring conn.close() called on a normal connection; token: {}",
                self.token
            );
            return Ok(());
        }
        self.set_closed(true);
        let arg = if arg.noreply_wait() {
            None
        } else {
            Some(r.expr(json!({ "noreply": false })))
        };
        let payload = Payload(QueryType::Stop, arg.as_ref(), Default::default());
        trace!("closing a changefeed; token: {}", self.token);
        let (typ, _) = self.request(&payload, false).await?;
        self.session.inner.unmark_change_feed();
        trace!(
            "conn.close() run; token: {}, response type: {:?}",
            self.token,
            typ,
        );
        Ok(())
    }

    fn closed(&self) -> bool {
        self.closed.load(Ordering::SeqCst)
    }

    fn set_closed(&self, closed: bool) {
        self.closed.store(closed, Ordering::SeqCst);
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        self.session.inner.channels.remove(&self.token);
        if self.session.inner.is_change_feed() {
            self.session.inner.unmark_change_feed();
        }
    }
}

/// The top-level ReQL namespace
///
/// # Example
///
/// Set up your top-level namespace.
///
/// ```
/// use unreql::r;
/// ```
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug)]
pub struct r;

impl r {
    /// Create a new connection to the database server
    ///
    /// # Example
    ///
    /// Open a connection using the default host and port, specifying the default database.
    ///
    /// ```
    /// use unreql::{r, cmd::connect::Options};
    ///
    /// # async fn example() -> unreql::Result<()> {
    /// let session = r.connect(Options::new().db("marvel")).await?;
    /// # Ok(()) }
    /// ```
    ///
    /// Read more about this command [connect](cmd::connect)
    pub async fn connect<T>(self, options: T) -> Result<Session>
    where
        T: cmd::connect::Arg,
    {
        cmd::connect::new(options.into_connect_opts()).await
    }

    /// Construct a ReQL JSON object from a native object.
    ///
    /// ## Example
    /// Objects wrapped with expr can then be manipulated by ReQL API functions.
    ///
    /// ```
    /// # use serde_json::json;
    /// # unreql::example(|r, conn| {
    /// r.expr(json!({"a":"b"})).merge(json!({"b":[1,2,3]})).run(conn)
    /// # })
    /// ```
    pub fn expr(self, arg: impl Serialize) -> Command {
        Command::from_json(arg)
    }

    /// `r.args` is a special term that’s used to splice an array of
    /// arguments into another term. This is useful when you want to
    /// call a variadic term such as `get_all` with a set of arguments
    /// produced at runtime.
    ///
    /// It is also the basic term for passing parameters to many driver
    /// methods in Rust. It allows you to accept an arbitrary number of
    /// arguments for commands that can have 0, 1 or more optional arguments
    ///
    /// ## Example
    /// Get Alice and Bob from the table people.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("people").get_all(r.args(["Alice", "Bob"])).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Get all of Alice’s children from the table people.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// // r.table('people').get('Alice') returns {id: 'Alice', children: ['Bob', 'Carol']}
    /// r.table("people")
    ///   .get_all(r.args(r.table("people").get("Alice").g("children")))
    ///   .run(conn)
    /// # })
    /// ```
    pub fn args<T>(self, arg: T) -> Args<T> {
        Args(arg)
    }

    /// The term with_opt is used in conjunction with `args` to pass
    /// optional options to the command
    ///
    /// ## Example
    /// Fetch all heroes by `secondary` index
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("heroes")
    ///   .get_all(r.with_opt(r.args([1, 2]), r.index("secondary")))
    ///   .run(conn)
    /// # })
    /// ```
    pub fn with_opt<T, P>(self, arg: T, opt: P) -> ArgsWithOpt<T, P> {
        ArgsWithOpt(arg, opt)
    }
}

// Helper for making writing examples less verbose
#[doc(hidden)]
pub fn example<'a, Q, R>(_query: Q)
where
    Q: FnOnce(r, &'a mut Session) -> R,
    R: futures::Stream<Item = Result<()>>,
{
}
