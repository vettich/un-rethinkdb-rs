use futures::{stream::Stream, TryStreamExt};
use ql2::term::TermType;
use serde::de::DeserializeOwned;

use crate::{
    cmd::{args::Opt, options::ChangesOptions, run},
    Command,
};

impl Command {
    /// Run a query on a connection.
    ///
    /// # Related commands
    /// - [exec](Self::exec)
    /// - [exec_to_vec](Self::exec_to_vec)
    pub fn run<A, T>(self, arg: A) -> impl Stream<Item = crate::Result<T>>
    where
        A: run::Arg,
        T: Unpin + DeserializeOwned,
    {
        Box::pin(run::new(self, arg))
    }

    /// Run a query on a connection and return one result.
    ///
    /// ## Example
    /// Return one result from table.
    ///
    /// ```
    /// # use unreql::{r, Session};
    /// # use serde_json::Value;
    /// # async fn example(conn: &mut Session) -> unreql::Result<()> {
    /// let doc: Value = r.table("test")
    ///   .get("id")
    ///   .exec(conn)
    ///   .await?;
    ///   # Ok(())
    /// # }
    /// ```
    ///
    /// You can also call `run` instead of `exec`:
    ///
    /// ```
    /// # use unreql::{r, Session};
    /// # use serde_json::Value;
    /// # use futures::TryStreamExt;
    /// # async fn example(conn: &mut Session) -> unreql::Result<()> {
    /// let doc = r.table("test")
    ///   .get("id")
    ///   .run::<_, Value>(conn)
    ///   .try_next()
    ///   .await?;
    ///   # Ok(())
    /// # }
    /// ```
    ///
    /// # Related commands
    /// - [run](Self::run)
    /// - [exec_to_vec](Self::exec_to_vec)
    pub async fn exec<A, T>(self, arg: A) -> crate::Result<T>
    where
        A: run::Arg,
        T: Unpin + DeserializeOwned,
    {
        match self.run(arg).try_next().await? {
            Some(result) => Ok(result),
            None => Err(crate::Driver::NotFound.into()),
        }
    }

    /// Run a query on a connection and collect all the results as `Vec`.
    ///
    /// ## Example
    /// Collect all results from table.
    ///
    /// ```
    /// # use unreql::{r, Session};
    /// # use serde_json::Value;
    /// # async fn example(conn: &mut Session) -> unreql::Result<()> {
    /// let docs: Vec<Value> = r.table("test").exec_to_vec(conn).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// You can also call `run` instead of `exec_to_vec`:
    ///
    /// ```
    /// # use unreql::{r, Session};
    /// # use serde_json::Value;
    /// # use futures::TryStreamExt;
    /// # async fn example(conn: &mut Session) {
    /// let mut cur = r.table("test").run::<_, Value>(conn);
    /// let mut docs = vec![];
    /// while let Ok(Some(doc)) = cur.try_next().await {
    ///   docs.push(doc);
    /// }
    /// # }
    /// ```
    ///
    /// # Related commands
    /// - [run](Self::run)
    /// - [exec](Self::exec)
    pub async fn exec_to_vec<A, T>(self, arg: A) -> crate::Result<Vec<T>>
    where
        A: run::Arg,
        T: Unpin + DeserializeOwned,
    {
        self.run(arg).try_collect().await
    }

    /// Turn a query into a changefeed, an infinite stream of objects
    /// representing changes to the query’s results as they occur.
    /// A changefeed may return changes to a table or an individual
    /// document (a “point” changefeed). Commands such as `filter`
    /// or `map` may be used before the `changes` command to transform
    /// or filter the output, and many commands that operate on sequences
    /// can be chained after `changes`.
    ///
    /// There are currently two states:
    ///
    /// - `{state: 'initializing'}` indicates the following documents
    ///   represent initial values on the feed rather than changes.
    ///   This will be the first document of a feed that returns initial values.
    /// - `{state: 'ready'}` indicates the following documents represent
    ///   changes. This will be the first document of a feed that does not
    ///   return initial values; otherwise, it will indicate the initial
    ///   values have all been sent.
    ///
    /// *Note*: Starting with RethinkDB 2.2, state documents will *only* be
    /// sent if the `includeStates` option is `true`, even on point
    /// changefeeds. Initial values will only be sent if `includeInitial`
    /// is `true`. If `includeStates` is `true` and `includeInitial` is
    /// `false`, the first document on the feed will be `{state: 'ready'}`.
    ///
    /// If the table becomes unavailable, the changefeed will be disconnected,
    /// and a runtime exception will be thrown by the driver.
    ///
    /// Changefeed notifications take the form of a two-field object:
    ///
    /// ```text
    /// {
    ///     "old_val": <document before change>,
    ///     "new_val": <document after change>
    /// }
    /// ```
    ///
    /// When includeTypes is true, there will be three fields:
    ///
    /// ```text
    /// {
    ///     "old_val": <document before change>,
    ///     "new_val": <document after change>,
    ///     "type": <result type>
    /// }
    /// ```
    ///
    /// See more details in [javascript api documentation](https://rethinkdb.com/api/javascript/changes)
    ///
    /// ## Example
    /// Subscribe to the changes on a table.
    /// Start monitoring the changefeed in one client:
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("games").changes(()).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Return all the changes that increase a player’s score.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("test")
    ///   .changes(())
    ///   .filter(r.row().g("new_val").g("score").gt(r.row().g("old_val").g("score")))
    ///   .run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Return all the changes to a specific player’s score that increase it past 10.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("test")
    ///   .get(1)
    ///   .filter(r.row().g("score").gt(10))
    ///   .changes(())
    ///   .run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Return all the inserts on a table.
    ///
    /// ```
    /// # use unreql::rjson;
    /// # unreql::example(|r, conn| {
    /// r.table("test")
    ///   .changes(())
    ///   .filter(r.row().g("old_val").eq(rjson!(null)))
    ///   .run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Return all the changes to game 1, with state notifications and initial values.
    ///
    /// ```
    /// # use unreql::cmd::options::ChangesOptions;
    /// # unreql::example(|r, conn| {
    /// r.table("games")
    ///   .get(1)
    ///   .changes(ChangesOptions::new().include_initial(true).include_states(true))
    ///   .run(conn)
    /// # })
    /// ```
    ///
    /// Result returned on changefeed
    ///
    /// ```text
    /// {state: 'initializing'}
    /// {new_val: {id: 1, score: 12, arena: 'Hobbiton Field'}}
    /// {state: 'ready'}
    /// {
    ///     old_val: {id: 1, score: 12, arena: 'Hobbiton Field'},
    ///     new_val: {id: 1, score: 14, arena: 'Hobbiton Field'}
    /// }
    /// {
    ///     old_val: {id: 1, score: 14, arena: 'Hobbiton Field'},
    ///     new_val: {id: 1, score: 17, arena: 'Hobbiton Field', winner: 'Frodo'}
    /// }
    /// ```
    ///
    /// ## Example
    /// Return all the changes to the top 10 games. This assumes the presence
    /// of a `score` secondary index on the `games` table.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("test")
    ///   .order_by(r.index(r.desc("score")))
    ///   .limit(10)
    ///   .changes(())
    ///   .run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [table](Self::table)
    pub fn changes(self, opt: impl Opt<ChangesOptions>) -> Command {
        opt.with_cmd(Command::new(TermType::Changes))
            .mark_change_feed()
            .with_parent(self)
    }
}
