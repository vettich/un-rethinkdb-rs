use ql2::term::TermType;
use serde::Serialize;
use unreql_macros::create_cmd;

use crate::{
    cmd::{
        args::{Arg, ManyArgs},
        options::{HttpOptions, JsOptions},
    },
    Command,
};

create_cmd!(
    /// Encapsulate binary data within a query.
    ///
    /// The type of data binary accepts depends on the client language.
    ///
    /// Only a limited subset of ReQL commands may be chained after binary:
    ///
    /// - `coerce_to` can coerce binary objects to string types
    /// - `count` will return the number of bytes in the object
    /// - `slice` will treat bytes like array indexes (i.e., slice(10,20) will return bytes 10–19)
    /// - `type_of` returns PTYPE<BINARY>
    /// - `info` will return information on a binary object.
    ///
    /// TODO
    binary,
    Serialize
);

create_cmd!(
    /// Call an anonymous function using return values from other ReQL commands
    /// or queries as arguments.
    ///
    /// The last argument to `do_` (or, in some forms, the only argument) is an
    /// expression or an anonymous function which receives values from either
    /// the previous arguments or from prefixed commands chained before `do_`.
    /// The `do_` command is essentially a single-element `map`, letting you map
    /// a function over just one document. This allows you to bind a query
    /// result to a local variable within the scope of `do_`, letting you compute
    /// the result just once and reuse it in a complex expression or in
    /// a series of ReQL commands.
    ///
    /// Arguments passed to the `do_` function must be basic data types, and
    /// cannot be streams or selections. (Read about ReQL data types.) While
    /// the arguments will all be evaluated before the function is executed,
    /// they may be evaluated in any order, so their values should not be
    /// dependent on one another. The type of `do_`’s result is the type of
    /// the value returned from the function or last expression.
    ///
    /// ## Example
    /// Compute a golfer’s net score for a game.
    ///
    /// ```
    /// # use unreql::func;
    /// # unreql::example(|r, conn| {
    /// r.table("players").get(3).do_(func!(|player| {
    ///   player.clone().g("gross_score").sub(player.g("course_handicap"))
    /// })).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Return the best scoring player in a two-player golf match.
    ///
    /// ```
    /// # use unreql::func;
    /// # unreql::example(|r, conn| {
    /// r.do_(
    ///   r.args((
    ///     r.table("players").get(1),
    ///     r.table("players").get(2),
    ///     func!(|player1, player2| {
    ///       r.branch(
    ///         player1.clone().g("gross_score").lt(player2.clone().g("gross_score")),
    ///         player1,
    ///         player2
    ///       )
    ///     })
    ///   ))
    /// ).run(conn)
    /// # })
    /// ```
    ///
    /// *Note* that `branch`, the ReQL conditional command, must be used
    /// instead of `if`. See the `branch` documentation for more.
    ///
    /// ## Example
    /// Take different actions based on the result of a ReQL `insert` command.
    ///
    /// ```
    /// # use unreql::{func, rjson};
    /// # unreql::example(|r, conn| {
    /// let new_data = rjson!({
    ///   "id": 100,
    ///   "name": "Agatha",
    ///   "gross_score": 57,
    ///   "course_handicap": 4,
    /// });
    /// r.table("players").insert(new_data).do_(func!(|doc| {
    ///   r.branch(
    ///     doc.clone().g("inserted").ne(0),
    ///     r.table("log").insert(rjson!({"time": r.now(), "response": doc.clone(), "result": "ok"})),
    ///     r.table("log").insert(rjson!({"time": r.now(), "response": doc, "result": "error"})),
    ///   )
    /// })).run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [map](Self::map)
    do_:Funcall,
    ManyArgs<()>
);

create_cmd!(
    /// Perform a branching conditional equivalent to if-then-else.
    ///
    /// The `branch` command takes 2n+1 arguments: pairs of conditional
    /// expressions and commands to be executed if the conditionals
    /// return any value but `false` or `null` (i.e., “truthy” values),
    /// with a final “else” command to be evaluated if all of the
    /// conditionals are `false` or `null`.
    ///
    /// See [branch_ext](Self::branch_ext) for use more test cases.
    ///
    /// ## Example
    /// Test the value of x.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// let x = 10;
    /// r.branch(r.expr(x).gt(5), "big", "small").run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Test the value of x.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// let x = 10;
    /// r.expr(x).gt(5).branch("big", "small").run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [do_](Self::do_)
    only_root,
    branch(test: Serialize, true_action: Serialize, false_action: Serialize)
    only_command,
    branch(true_action: Serialize, false_action: Serialize)
);

create_cmd!(
    /// Perform a branching conditional equivalent to if-then-else.
    ///
    /// ## Example
    /// Categorize heroes by victory counts.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("marvel").map(
    ///   r.branch_ext(r.args([
    ///     r.row().g("victories").gt(100),
    ///     r.row().g("name").add(" is a superhero"),
    ///     r.row().g("victories").gt(10),
    ///     r.row().g("name").add(" is a hero"),
    ///     r.row().g("name").add(" is a very nice")
    ///   ]))
    /// ).run(conn)
    /// # })
    /// ```
    ///
    /// To use for simple if-then-else see [branch](Self::branch).
    only_root,
    branch_ext:Branch(test_then_actions: ManyArgs<()>)
);

create_cmd!(
    /// Loop over a sequence, evaluating the given write query for each element.
    ///
    /// ## Example
    /// Now that our heroes have defeated their villains, we can safely remove
    /// them from the villain table.
    ///
    /// ```
    /// # use unreql::func;
    /// # unreql::example(|r, conn| {
    /// r.table("marvel").for_each(func!(|hero| {
    ///   r.table("villiains").get(hero.g("villainDefeated")).delete(())
    /// })).run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [map](Self::map)
    only_command,
    for_each(write_function: Serialize)
);

create_cmd!(
    /// Generate a stream of sequential integers in a specified range.
    ///
    /// `range` takes 0, 1 or 2 arguments:
    ///
    /// - With no arguments, `range` returns an “infinite” stream from 0 up
    ///   to and including the maximum integer value;
    /// - With one argument, `range` returns a stream from 0 up to but not
    ///   including the end value;
    /// - With two arguments, `range` returns a stream from the start value up
    ///   to but not including the end value.
    ///
    /// Note that the left bound (including the implied left bound of 0 in the
    /// 0- and 1-argument form) is always closed and the right bound is always
    /// open: the start value will always be included in the returned range
    /// and the end value will not be included in the returned range.
    ///
    /// Any specified arguments must be integers, or a `ReqlRuntimeError` will
    /// be thrown. If the start value is equal or to higher than the end value,
    /// no error will be thrown but a zero-element stream will be returned.
    ///
    /// ## Example
    /// Return a four-element range of `[0, 1, 2, 3]`.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.range(4).run(conn)
    /// // Result: [0, 1, 2, 3]
    /// # })
    /// ```
    ///
    /// You can also use the limit command with the no-argument variant to
    /// achieve the same result in this case:
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.range(()).limit(4).run(conn)
    /// // Result: [0, 1, 2, 3]
    /// # })
    /// ```
    ///
    /// ## Example
    /// Return a range from -5 through 5.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.range(r.args([-5, 6])).run(conn)
    /// // Result: [-5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5]
    /// # })
    /// ```
    only_root,
    range(start_end_values: ManyArgs<()>)
);

create_cmd!(
    /// Throw a runtime error. If called with no arguments inside the
    /// second argument to `default`, re-throw the current error.
    ///
    /// ## Example
    /// Iron Man can’t possibly have lost a battle:
    ///
    /// ```
    /// # use unreql::func;
    /// # unreql::example(|r, conn| {
    /// r.table("marvel").get("IronMan").do_(func!(|ironman| {
    ///     r.branch(
    ///         ironman.clone().g("victories").lt(ironman.clone().g("battles")),
    ///         r.error("impossible code path"),
    ///         ironman
    ///     )
    /// })).run(conn)
    /// # })
    /// ```
    ///
    only_root,
    error(message: Serialize)
);

create_cmd!(
    /// Provide a default value in case of non-existence errors.
    ///
    /// The `default` command evaluates its first argument (the value it’s
    /// chained to). If that argument returns `null` or a non-existence error
    /// is thrown in evaluation, then `default` returns its second argument.
    /// The second argument is usually a default value, but it can be
    /// a function that returns a value.
    ///
    /// ## Example
    /// Retrieve the titles and authors of the table `posts`. In the case where
    /// the author field is missing or `null`, we want to retrieve the string
    /// `Anonymous`.
    ///
    /// ```
    /// # use unreql::{func, rjson};
    /// # unreql::example(|r, conn| {
    /// r.table("posts").map(func!(|post| {
    ///     rjson!({
    ///         "title": post.clone().g("title"),
    ///         "author": post.g("author").default("Anonymous")
    ///     })
    /// })).run(conn)
    /// # })
    /// ```
    ///
    /// We can rewrite the previous query with r.branch too.
    ///
    /// ```
    /// # use unreql::{func, rjson};
    /// # unreql::example(|r, conn| {
    /// r.table("posts").map(func!(|post| {
    ///     r.branch(
    ///         post.clone().has_fields("author"),
    ///         rjson!({
    ///             "title": post.clone().g("title"),
    ///             "author": post.clone().g("author")
    ///         }),
    ///         rjson!({
    ///             "title": post.g("title"),
    ///             "author": "Anonymous"
    ///         })
    ///     )
    /// })).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// The `default` command can also be used to filter documents. Retrieve
    /// all our users who are not grown-ups or whose age is unknown
    /// (i.e., the field `age` is missing or equals `null`).
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("users").filter(
    ///     r.row().g("age").lt(18).default(true)
    /// ).run(conn)
    /// # })
    /// ```
    ///
    /// One more way to write the previous query is to set the age
    /// to be `-1` when the field is missing.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("users").filter(
    ///     r.row().g("age").default(-1).lt(18)
    /// ).run(conn)
    /// # })
    /// ```
    ///
    /// This can be accomplished with `has_fields` rather than `default`.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("users").filter(
    ///     r.row().has_fields("age").not().or(r.row().g("age").lt(18))
    /// ).run(conn)
    /// # })
    /// ```
    ///
    /// The body of every `filter` is wrapped in an implicit `.default(false)`.
    /// You can overwrite the value `false` with the `default` option.
    ///
    /// ```
    /// # use unreql::cmd::options::FilterOptions;
    /// # unreql::example(|r, conn| {
    /// r.table("users").filter(r.with_opt(
    ///     r.row().g("age").lt(18),
    ///     FilterOptions::new().default(true)
    /// )).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// The function form of `default` receives the error message
    /// as its argument.
    ///
    /// ```
    /// # use unreql::{func, rjson};
    /// # unreql::example(|r, conn| {
    /// r.table("posts").map(func!(|post| {
    ///     rjson!({
    ///         "title": post.clone().g("title"),
    ///         "author": post.clone().default(func!(|err| err))
    ///     })
    /// })).run(conn)
    /// # })
    /// ```
    ///
    /// This particular example simply returns the error message, so it isn’t
    /// very useful. But it would be possible to change the default value
    /// based on the specific error message thrown.
    only_command,
    default(value_or_function: Serialize)
);

create_cmd!(
    /// Create a javascript expression.
    ///
    /// timeout is the number of seconds before r.js times out.
    /// The default value is 5 seconds.
    ///
    /// *Note*: Whenever possible, you should use native ReQL commands
    /// rather than `r.js` for better performance.
    ///
    /// ## Example
    /// Concatenate two strings using JavaScript.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.js("'str1' + 'str2'").run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Select all documents where the ‘magazines’ field is greater than 5 by
    /// running JavaScript on the server.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("marvel").filter(
    ///     r.js("(function (row) { return row.magazines.length > 5; })")
    /// ).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// You may also specify a timeout in seconds (defaults to 5).
    ///
    /// ```
    /// # use unreql::cmd::options::JsOptions;
    /// # unreql::example(|r, conn| {
    /// r.js(r.with_opt("while(true) {}", JsOptions::new().timeout(1.3))).run(conn)
    /// # })
    /// ```
    only_root,
    js:Javascript(js_string: Arg<JsOptions>)
);

create_cmd!(
    /// Convert a value of one type into another.
    ///
    /// - a sequence, selection or object can be coerced to an array
    /// - a sequence, selection or an array of key-value pairs can be coerced
    ///   to an object
    /// - a string can be coerced to a number
    /// - any datum (single value) can be coerced to to a string
    /// - a binary object can be coerced to a string and vice-versa
    ///
    /// ## Example
    /// Coerce a stream to an array to store its output in a field. (A stream
    /// cannot be stored in a field directly.)
    ///
    /// ```
    /// # use unreql::{func, rjson};
    /// # unreql::example(|r, conn| {
    /// r.table("posts").map(func!(|post| {
    ///     post.clone().merge(rjson!({
    ///         "comments": r.table("comments")
    ///             .get_all(r.with_opt(post.g("id"), r.index("postId")))
    ///             .coerce_to("array")
    ///     }))
    /// })).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Coerce an array of key-value pairs into an object.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.expr((("name", "Ironman"), ("victories", 2000)))
    ///     .coerce_to("object")
    ///     .run(conn)
    /// # })
    /// ```
    ///
    /// *Note*: To coerce a list of key-value pairs like
    /// `['name', 'Ironman', 'victories', 2000]` to an object,
    /// use the `object` command.
    ///
    /// ## Example
    /// Coerce a number to a string.
    ///
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.expr(1).coerce_to("string").run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [object](Self::object)
    only_command,
    coerce_to(type_: Serialize)
);

create_cmd!(
    /// Gets the type of a ReQL query’s return value.
    ///
    /// The type will be returned as a string:
    ///
    /// - `ARRAY`
    /// - `BOOL`
    /// - `DB`
    /// - `FUNCTION`
    /// - `GROUPED_DATA`
    /// - `GROUPED_STREAM`
    /// - `MAXVAL`
    /// - `MINVAL`
    /// - `NULL`
    /// - `NUMBER`
    /// - `OBJECT`
    /// - `PTYPE<BINARY>`
    /// - `PTYPE<GEOMETRY>`
    /// - `PTYPE<TIME>`
    /// - `SELECTION<ARRAY>`
    /// - `SELECTION<OBJECT>`
    /// - `SELECTION<STREAM>`
    /// - `STREAM`
    /// - `STRING`
    /// - `TABLE_SLICE`
    /// - `TABLE`
    ///
    /// Read the article on ReQL data types for a more detailed discussion.
    /// Note that some possible return values from typeOf are internal values,
    /// such as MAXVAL, and unlikely to be returned from queries in standard
    /// practice.
    ///
    /// ## Example
    /// Get the type of a string.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.expr("foo").type_of().run(conn)
    /// // Result: "STRING"
    /// # })
    /// ```
    only_command,
    type_of,
);

create_cmd!(
    /// Get information about a ReQL value.
    ///
    /// ## Example
    /// Get information about a table such as primary key, or cache size.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("marvel").info().run(conn)
    /// # })
    /// ```
    only_root,
    info(any: Serialize)
    only_command,
    info
);

create_cmd!(
    /// Parse a JSON string on the server.
    ///
    /// ## Example
    /// Send an array to the server.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.json("[1,2,3]").run(conn)
    /// # })
    /// ```
    only_root,
    json(json_string: Serialize)
);

create_cmd!(
    /// Convert a ReQL value or object to a JSON string.
    ///
    /// ## Example
    /// Get a ReQL document as a JSON string.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("hero").get(1).to_json().run(conn)
    /// # })
    /// ```
    ///
    /// Result:
    ///
    /// ```json
    /// {"id": 1, "name": "Batman", "city": "Gotham", "powers": ["martial arts", "cinematic entrances"]}
    /// ```
    only_command,
    to_json:ToJsonString
);

create_cmd!(
    /// Retrieve data from the specified URL over HTTP.
    ///
    /// The return type depends on the resultFormat option, which checks the
    /// Content-Type of the response by default. Make sure that you never use
    /// this command for user provided URLs.
    ///
    /// ## Example
    /// Perform an HTTP GET and store the result in a table.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("posts").insert(r.http("http://httpbin.org/get")).run(conn)
    /// # })
    /// ```
    ///
    /// See [the tutorial](https://rethinkdb.com/docs/external-api-access/) on `r.http` for more examples on how to use this command.
    http(url: Arg<HttpOptions>)
);

create_cmd!(
    /// Return a UUID (universally unique identifier), a string that can be
    /// used as a unique ID.
    ///
    /// If a string is passed to `uuid` as an argument, the UUID will be
    /// deterministic, derived from the string’s SHA-1 hash.
    ///
    /// RethinkDB’s UUIDs are standards-compliant. Without the optional
    /// argument, a version 4 random UUID will be generated; with that
    /// argument, a version 5 UUID will be generated, using a fixed
    /// namespace UUID of `91461c99-f89d-49d2-af96-d8e2e14e9b58`.
    /// For more information, read [Wikipedia’s UUID article](https://en.wikipedia.org/wiki/Universally_unique_identifier).
    ///
    /// *Note* Please take into consideration when you generating version 5
    /// UUIDs can’t be considered guaranteed unique if they’re computing based
    /// on user data because they use SHA-1 algorithm.
    ///
    /// ## Example
    /// Generate a UUID.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.uuid(()).run(conn)
    /// // Result: "27961a0e-f4e8-4eb3-bf95-c5203e1d87b9"
    /// # })
    /// ```
    ///
    /// ## Example
    /// Generate a UUID based on a string.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.uuid("slava@example.com").run(conn)
    /// // Result: "90691cbc-b5ea-5826-ae98-951e30fc3b2d"
    /// # })
    /// ```
    only_root,
    uuid(string: Arg<()>)
);
