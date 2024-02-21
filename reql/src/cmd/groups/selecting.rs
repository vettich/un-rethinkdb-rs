use ql2::term::TermType;
use serde::Serialize;
use unreql_macros::create_cmd;

use crate::{
    cmd::{
        args::{Arg, ManyArgs, Opt},
        options::{BetweenOptions, FilterOptions, Index, TableOptions},
    },
    Command,
};

create_cmd!(
    /// Reference a database.
    ///
    /// The db command is optional. If it is not present in a query, the query will run against the default database for the connection, specified in the db argument to connect.
    ///
    /// ## Example
    /// Explicitly specify a database for a query
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.db("heroes").table("marvel").run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [table](Self::table)
    /// - [db_list](Self::db_list)
    only_root,
    db(db_name: Serialize)
);

create_cmd!(
    /// Return all documents in a table. Other commands may be chained after table to return a subset of documents (such as get and filter) or perform further processing.
    ///
    /// There are two optional arguments.
    /// - readMode: One of three possible values affecting the consistency guarantee for the table read:
    ///   - single returns values that are in memory (but not necessarily written to disk) on the primary replica. This is the default.
    ///   - majority will only return values that are safely committed on disk on a majority of replicas. This requires sending a message to every replica on each read, so it is the slowest but most consistent.
    ///   - outdated will return values that are in memory on an arbitrarily-selected replica. This is the fastest but least consistent.
    /// - identifierFormat: possible values are name and uuid, with a default of name. If set to uuid, then system tables will refer to servers, databases and tables by UUID rather than name. (This only has an effect when used with system tables.)
    ///
    /// ## Example
    /// Return all documents in the table ‘marvel’ of the default database.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("marvel").run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Allow potentially out-of-date data in exchange for faster reads
    ///
    /// ```
    /// # use unreql::cmd::options::{TableOptions, ReadMode};
    /// # unreql::example(|r, conn| {
    /// let opts = TableOptions {
    ///   read_mode: Some(ReadMode::Outdated),
    ///   ..Default::default()
    /// };
    /// r.db("heroes").table(r.with_opt("marvel", opts)).run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [filter](Self::filter)
    /// - [get](Self::get)
    table(name: Arg<TableOptions>)
);

create_cmd!(
    /// Get a document by primary key.
    ///
    /// If no document exists with that primary key, `get` will return `null`.
    ///
    /// ## Example
    /// Find a document by UUID.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("posts").get("a9849eef-7176-4411-935b-79a6e3c56a74").run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Find a document and merge another document with it.
    ///
    /// ```
    /// # use unreql::rjson;
    /// # unreql::example(|r, conn| {
    /// r.table("heroes")
    ///   .get(3)
    ///   .merge(rjson!({ "powers": ["invisibility", "speed"] }))
    ///   .run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Subscribe to a document’s changefeed.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("heroes").get(3).changes(()).run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [get_all](Self::get_all)
    /// - [between](Self::between)
    /// - [filter](Self::filter)
    only_command,
    get(key: Serialize)
);

create_cmd!(
    /// Get all documents where the given value matches the value of the requested index.
    ///
    /// ## Example
    /// Secondary index keys are not guaranteed to be unique so we cannot query via get when using a secondary index.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("marvel").get_all(r.with_opt("man_of_steel", r.index("code_name"))).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Without an index argument, we default to the primary index. While `get` will either return the document or `null` when no document with such a primary key value exists, this will return either a one or zero length stream.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("dc").get_all("superman").run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// You can get multiple documents in a single call to `get_all`.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("dc").get_all(r.args(["superman", "ant man"])).run(conn)
    /// # })
    /// ```
    ///
    /// *Note*: getAll does not perform any de-duplication. If you pass the same key more
    /// than once, the same document will be returned multiple times.
    ///
    /// ## Example
    /// You can use args with getAll to retrieve multiple documents whose keys are in a
    /// list. This uses getAll to get a list of female superheroes, coerces that to an
    /// array, and then gets a list of villains who have those superheroes as enemies.
    ///
    /// ```
    /// # use unreql::func;
    /// # unreql::example(|r, conn| {
    /// r.do_(r.args((
    ///   r.table("dc").get_all(r.with_opt("f", r.index("gender"))).g("id").coerce_to("array"),
    ///   func!(|heroines| {
    ///     r.table("villains").get_all(r.args(heroines))
    ///   })
    /// ))).run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [get](Self::get)
    /// - [between](Self::between)
    /// - [filter](Self::filter)
    only_command,
    get_all(args: ManyArgs<Index>)
);

create_cmd!(
    /// Get all documents between two keys.
    ///
    /// You may also use the special constants `r.minval` and `r.maxval` for boundaries,
    /// which represent “less than any index key” and “more than any index key”
    /// respectively. For instance, if you use `r.minval` as the lower key, then `between`
    /// will return all documents whose primary keys (or indexes) are less than the
    /// specified upper key.
    ///
    /// If you use arrays as indexes (compound indexes), they will be sorted using
    /// lexicographical order. Take the following range as an example:
    ///
    /// ```[[1, "c"] ... [5, "e"]]```
    ///
    /// This range includes all compound keys:
    ///
    /// - whose first item is 1 and second item is equal or greater than “c”;
    /// - whose first item is between 1 and 5, regardless of the value of the second item;
    /// - whose first item is 5 and second item is less than or equal to “e”.
    ///
    /// ## Example
    /// Find all users with primary key >= 10 and < 20 (a normal half-open interval).
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("marvel").between(10, 20, ()).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Find all users with primary key >= 10 and <= 20 (an interval closed on both sides).
    ///
    /// ```
    /// # use unreql::cmd::options::{BetweenOptions, Status};
    /// # unreql::example(|r, conn| {
    /// let opts = BetweenOptions::new().right_bound(Status::Closed);
    /// r.table("marvel").between(10, 20, opts).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Find all users with primary key < 20.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("marvel").between(r.minval(), 20, ()).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Find all users with primary key > 10.
    ///
    /// ```
    /// # use unreql::cmd::options::{BetweenOptions, Status};
    /// # unreql::example(|r, conn| {
    /// let opts = BetweenOptions::new().left_bound(Status::Open);
    /// r.table("marvel").between(10, r.maxval(), opts).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Between can be used on secondary indexes too. Just pass an optional index
    /// argument giving the secondary index to query.
    ///
    /// ```
    /// # use unreql::cmd::options::{BetweenOptions, Status};
    /// # unreql::example(|r, conn| {
    /// r.table("dc").between("dark_knight", "man_of_steel", r.index("code_name")).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Get all users whose full name is between “John Smith” and “Wade Welles.”
    ///
    /// ```
    /// # use unreql::cmd::options::{BetweenOptions, Status};
    /// # unreql::example(|r, conn| {
    /// r.table("users").between(["Smith", "John"], ["Welles", "Wade"], r.index("full_name")).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Get the top 10 ranked teams in order.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("teams").order_by(r.index("rank")).between(1, 11, ()).run(conn)
    /// # })
    /// ```
    ///
    /// *Note*: When `between` is chained after `order_by`, both commands must use the same
    /// index; `between` will default to the index `order_by` is using, so in this example
    /// `"rank"` is automatically being used by `between`. Trying to specify another index
    /// will result in a `ReqlRuntimeError`.
    ///
    /// ## Example
    /// Subscribe to a changefeed of teams ranked in the top 10.
    ///
    /// ```
    /// # use unreql::cmd::options::{BetweenOptions, Status};
    /// # unreql::example(|r, conn| {
    /// r.table("teams").between(1, 11, r.index("rank")).changes(()).run(conn)
    /// # })
    /// ```
    ///
    /// The `between` command works with secondary indexes on date fields, but will not work
    /// with unindexed date fields. To test whether a date value is between two other
    /// dates, use the `during` command, not `between`.
    ///
    /// Secondary indexes can be used in extremely powerful ways with `between` and other
    /// commands; read the full article on [secondary indexes](https://rethinkdb.com/docs/secondary-indexes) for examples using boolean
    /// operations, `contains` and more.
    ///
    /// RethinkDB uses byte-wise ordering for between and does not support Unicode
    /// collations; non-ASCII characters will be sorted by UTF-8 codepoint.
    ///
    /// # Related commands
    /// - [get](Self::get)
    /// - [get_all](Self::get_all)
    /// - [filter](Self::filter)
    only_command,
    between(lower_key: Serialize, upper_key: Serialize, opt: Opt<BetweenOptions>)
);

create_cmd!(
    /// Return all the elements in a sequence for which the given predicate is true.
    ///
    /// The return value of filter will be the same as the input (sequence, stream,
    /// or array). Documents can be filtered in a variety of ways-ranges, nested values,
    /// boolean conditions, and the results of anonymous functions.
    ///
    /// By default, `filter` will silently skip documents with missing fields: if the
    /// predicate tries to access a field that doesn’t exist (for instance, the predicate
    /// `{age: 30}` applied to a document with no `age` field), that document will not be
    /// returned in the result set, and no error will be generated. This behavior can be
    /// changed with the `default` optional argument.
    ///
    /// - If `default` is set to `true`, documents with missing fields will be returned
    ///   rather than skipped.
    /// - If `default` is set to `r.error()`, an `ReqlRuntimeError` will be thrown when
    ///   a document with a missing field is tested.
    /// - If `default` is set to `false` (the default), documents with missing fields
    ///   will be skipped.
    ///
    /// *Note*: filter does not use secondary indexes. For retrieving documents via
    /// secondary indexes, consider getAll, between and eqJoin.
    ///
    /// # Basic predicates
    ///
    /// ## Example
    /// Get all users who are 30 years old.
    ///
    /// ```
    /// # use unreql::rjson;
    /// # unreql::example(|r, conn| {
    /// r.table("users").filter(rjson!({ "age": 30 })).run(conn)
    /// # })
    /// ```
    ///
    /// The predicate `{age: 30}` selects documents in the `users` table with an `age` field
    /// whose value is `30`. Documents with an `age` field set to any other value or with
    /// no `age` field present are skipped.
    ///
    /// While the `{field: value}` style of predicate is useful for exact matches, a more
    /// general way to write a predicate is to use the [row](Self::row) command with a comparison
    /// operator such as [eq](Self::eq) or [gt](Self::gt), or to use an anonymous function that returns `true`
    /// or `false`.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("users").filter(r.row().g("age").eq(30)).run(conn)
    /// # })
    /// ```
    ///
    /// In this case, the predicate `r.row().g("age").eq(30)` returns `true` if the field
    /// `age` is equal to 30. You can write this predicate as an anonymous function instead:
    ///
    /// ```
    /// # use unreql::func;
    /// # unreql::example(|r, conn| {
    /// r.table("users").filter(func!(|user| {
    ///   user.g("age").eq(30)
    /// })).run(conn)
    /// # })
    /// ```
    ///
    /// Predicates to `filter` are evaluated on the server, and must use ReQL expressions.
    /// You cannot use standard JavaScript comparison operators
    /// such as `==`, `<`/`>` and `||`/`&&`.
    ///
    /// Also, predicates must evaluate document fields. They cannot evaluate
    /// [secondary indexes](https://rethinkdb.com/docs/secondary-indexes/javascript/).
    ///
    /// ## Example
    /// Get all users who are more than 18 years old.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("users").filter(r.row().g("age").eq(18)).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Get all users who are less than 18 years old and more than 13 years old.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("users").filter(
    ///   r.row().g("age").lt(18).and(r.row().g("age").gt(13))
    /// ).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Get all users who are more than 18 years old or have their parental consent.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("users").filter(
    ///   r.row().g("age").ge(18).and(r.row().g("hasParentalConsent"))
    /// ).run(conn)
    /// # })
    /// ```
    ///
    /// # More complex predicates
    ///
    /// ## Example
    /// Retrieve all users who subscribed between January 1st, 2012 (included)
    /// and January 1st, 2013 (excluded).
    ///
    /// ```
    /// # use unreql::func;
    /// # unreql::example(|r, conn| {
    /// r.table("users").filter(func!(|user| {
    ///   user.g("subscriptionDate").during(
    ///     r.time(2012, 1, 1, 'Z'),
    ///     r.time(2013, 1, 1, 'Z'),
    ///     ()
    ///   )
    /// })).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Retrieve all users who have a gmail account (whose field `email` ends with `@gmail.com`).
    ///
    /// ```
    /// # use unreql::func;
    /// # unreql::example(|r, conn| {
    /// r.table("users").filter(func!(|user| {
    ///   user.g("email").match_("@gmail.com$")
    /// })).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Filter based on the presence of a value in an array.
    ///
    /// Given this schema for the `users` table:
    ///
    /// ```text
    /// {
    ///     name: String
    ///     placesVisited: [String]
    /// }
    /// ```
    ///
    /// Retrieve all users whose field placesVisited contains France.
    ///
    /// ```
    /// # use unreql::func;
    /// # unreql::example(|r, conn| {
    /// r.table("users").filter(func!(|user| {
    ///   user.g("placesVisited").contains("France")
    /// })).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Filter based on nested fields.
    ///
    /// Given this schema for the `users` table:
    ///
    /// ```text
    /// {
    ///     id: String
    ///     name: {
    ///         first: String,
    ///         middle: String,
    ///         last: String
    ///     }
    /// }
    /// ```
    ///
    /// Retrieve all users named “William Adama” (first name “William”, last name “Adama”),
    /// with any middle name.
    ///
    /// ```
    /// # use unreql::rjson;
    /// # unreql::example(|r, conn| {
    /// r.table("users").filter(rjson!({
    ///   "name": {
    ///     "first": "William",
    ///     "last": "Adama"
    ///   }
    /// })).run(conn)
    /// # })
    /// ```
    ///
    /// If you want an exact match for a field that is an object, you will have to use `r.literal`.
    ///
    /// Retrieve all users named “William Adama” (first name “William”, last name “Adama”),
    /// and who do not have a middle name.
    ///
    /// ```
    /// # use unreql::rjson;
    /// # unreql::example(|r, conn| {
    /// r.table("users").filter(r.literal(rjson!({
    ///   "name": {
    ///     "first": "William",
    ///     "last": "Adama"
    ///   }
    /// }))).run(conn)
    /// # })
    /// ```
    ///
    /// You may rewrite these with anonymous functions.
    ///
    ///
    /// ```
    /// # use unreql::{func, rjson};
    /// # unreql::example(|r, conn| {
    /// r.table("users").filter(func!(|user| {
    ///   let left = user.clone().g("name").g("first").eq("William");
    ///   let right = user.g("name").g("last").eq("Adama");
    ///   left.and(right)
    /// })).run(conn)
    /// # });
    ///
    /// // or
    ///
    /// # unreql::example(|r, conn| {
    /// r.table("users").filter(func!(|user| {
    ///   user.g("name").eq(rjson!({
    ///     "first": "William",
    ///     "last": "Adama"
    ///   }))
    /// })).run(conn)
    /// # })
    /// ```
    ///
    /// # Handling missing fields
    ///
    /// By default, documents missing fields tested by the `filter` predicate are skipped.
    /// In the previous examples, users without an `age` field are not returned. By passing
    /// the optional `default` argument to `filter`, you can change this behavior.
    ///
    /// ## Example
    /// Get all users less than 18 years old or whose age field is missing.
    ///
    /// ```
    /// # use unreql::cmd::options::FilterOptions;
    /// # unreql::example(|r, conn| {
    /// r.table("users").filter(r.with_opt(
    ///   r.row().g("age").lt(18),
    ///   FilterOptions { default: Some(true) }
    /// )).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Get all users more than 18 years old. Throw an error if a document is missing
    /// the field `age`.
    ///
    /// *TODO*: `FilterOptions { default: Some(r.error()) }` not implemented now
    ///
    /// ```text
    /// r.table("users").filter(r.with_opt(
    ///   r.row().g("age").lt(18),
    ///   FilterOptions { default: Some(r.error()) }
    /// )).run(conn)
    /// ```
    ///
    /// ## Example
    /// Get all users who have given their phone number (all the documents whose field
    /// `phoneNumber` exists and is not `null`).
    ///
    /// ```
    /// # use unreql::func;
    /// # unreql::example(|r, conn| {
    /// r.table("users").filter(func!(|user| {
    ///   user.has_fields("phoneNumber")
    /// })).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Get all users with an “editor” role or an “admin” privilege.
    ///
    /// ```
    /// # use unreql::func;
    /// # unreql::example(|r, conn| {
    /// r.table("users").filter(func!(|user| {
    ///   let editor = user.clone().g("role").eq("editor").default(false);
    ///   let admin = user.g("role").eq("admin").default(false);
    ///   editor.or(admin)
    /// })).run(conn)
    /// # })
    /// ```
    ///
    /// Instead of using the `default` optional argument to `filter`, we have to use
    /// default values on the fields within the `or` clause. Why? If the field on
    /// the left side of the or clause is missing from a document - in this case,
    /// if the user doesn’t have a `role` field - the predicate will generate an error,
    /// and will return `false` (or the value the `default` argument is set to) without
    /// evaluating the right side of the or. By using `.default(false)` on the fields,
    /// each side of the `or` will evaluate to either the field’s value or `false`
    /// if the field doesn’t exist.
    ///
    /// # Related commands
    /// - [get](Self::get)
    /// - [get_all](Self::get_all)
    /// - [between](Self::between)
    only_command,
    filter(predicate: Arg<FilterOptions>)
);
