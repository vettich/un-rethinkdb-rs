use ql2::term::TermType;
use serde::Serialize;
use unreql_macros::create_cmd;

use crate::{
    cmd::{args::Opt, options::Index},
    Command,
};

create_cmd!(
    /// Returns an inner join of two sequences.
    ///
    /// The returned sequence represents an intersection of the left-hand sequence
    /// and the right-hand sequence: each row of the left-hand sequence will be compared
    /// with each row of the right-hand sequence to find all pairs of rows which satisfy
    /// the predicate. Each matched pair of rows of both sequences are combined into
    /// a result row. In most cases, you will want to follow the join with zip to combine
    /// the left and right results.
    ///
    /// *Note* that `inner_join` is slower and much less efficient than using `eq_join` or
    /// `concat_map` with `get_all`. You should avoid using `inner_join` in commands when possible.
    ///
    /// ## Example
    /// Return a list of all matchups between Marvel and DC heroes in which the DC hero
    /// could beat the Marvel hero in a fight.
    ///
    /// ```
    /// # use unreql::func;
    /// # unreql::example(|r, conn| {
    /// r.table("marvel").inner_join(
    ///     r.table("dc"),
    ///     func!(|marvelRow, dcRow| {
    ///         marvelRow.g("strength").lt(dcRow.g("strength"))
    ///     })
    /// ).zip().run(conn)
    /// # })
    /// ```
    ///
    /// (Compare this to an [outer_join](Self::outer_join) with the same inputs and predicate, which would
    /// return a list of all Marvel heroes along with any DC heroes with a higher strength.)
    ///
    /// # Related commands
    /// - [outer_join](Self::outer_join)
    /// - [eq_join](Self::eq_join)
    /// - [zip](Self::zip)
    only_command,
    inner_join(other_sequence: Serialize, predicate: Serialize)
);

create_cmd!(
    /// Returns a left outer join of two sequences.
    ///
    /// The returned sequence represents a union of the left-hand sequence and
    /// the right-hand sequence: all documents in the left-hand sequence will be returned,
    /// each matched with a document in the right-hand sequence if one satisfies
    /// the predicate condition. In most cases, you will want to follow the join with
    /// [zip](Self::zip) to combine the left and right results.
    ///
    /// *Note* that `outer_join` is slower and much less efficient than using
    /// [concat_map](Self::concat_map) with [get_all](Self::get_all). You should avoid using `outer_join` in commands when possible.
    ///
    /// ## Example
    /// Return a list of all Marvel heroes, paired with any DC heroes who could beat them in a fight.
    ///
    /// ```
    /// # use unreql::func;
    /// # unreql::example(|r, conn| {
    /// r.table("marvel").outer_join(
    ///     r.table("dc"),
    ///     func!(|marvelRow, dcRow| {
    ///         marvelRow.g("strength").lt(dcRow.g("strength"))
    ///     })
    /// ).zip().run(conn)
    /// # })
    /// ```
    ///
    /// (Compare this to an [inner_join](Self::inner_join) with the same inputs and predicate,
    /// which would return a list only of the matchups in which the DC hero has
    /// the higher strength.)
    ///
    /// # Related commands
    /// - [inner_join](Self::inner_join)
    /// - [eq_join](Self::eq_join)
    /// - [zip](Self::zip)
    only_command,
    outer_join(other_sequence: Serialize, predicate: Serialize)
);

create_cmd!(
    /// Join tables using a field or function on the left-hand sequence
    /// matching primary keys or secondary indexes on the right-hand table.
    ///
    /// `eq_join` is more efficient than other ReQL join types, and operates
    /// much faster. Documents in the result set consist of pairs of left-hand
    /// and right-hand documents, matched when the field on the left-hand side
    /// exists and is non-null and an entry with that field’s value exists
    /// in the specified index on the right-hand side.
    ///
    /// The result set of `eq_join` is a stream or array of objects. Each object
    /// in the returned set will be an object of the form
    /// `{ left: <left-document>, right: <right-document> }`, where the values
    /// of `left` and `right` will be the joined documents. Use the [zip](Self::zip) command
    /// to merge the `left` and `right` fields together.
    ///
    /// The results from `eq_join` are, by default, not ordered. The optional
    /// `ordered: true` parameter will cause `eq_join` to order the output
    /// based on the left side input stream. (If there are multiple matches
    /// on the right side for a document on the left side, their order is
    /// not guaranteed even if `ordered` is `true`.) Requiring ordered results
    /// can significantly slow down `eq_join`, and in many circumstances this
    /// ordering will not be required. (See the first example, in which
    /// ordered results are obtained by using `order_by` after `eq_join`.)
    ///
    /// Suppose the players table contains these documents:
    ///
    /// ```json
    /// [
    ///     { "id": 1, "player": "George", "gameId": 1 },
    ///     { "id": 2, "player": "Agatha", "gameId": 3 },
    ///     { "id": 3, "player": "Fred", "gameId": 2 },
    ///     { "id": 4, "player": "Marie", "gameId": 2 },
    ///     { "id": 5, "player": "Earnest", "gameId": 1 },
    ///     { "id": 6, "player": "Beth", "gameId": 3 }
    /// ]
    /// ```
    ///
    /// The games table contains these documents:
    ///
    /// ```json
    /// [
    ///     { "id": 1, "field": "Little Delving" },
    ///     { "id": 2, "field": "Rushock Bog" },
    ///     { "id": 3, "field": "Bucklebury" }
    /// ]
    /// ```
    ///
    /// ## Example: Match players with the games they’ve played against one another.
    ///
    /// Join these tables using gameId on the player table and id on the games table:
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("players").eq_join("game_id", r.table("games"), ()).run(conn)
    /// # })
    /// ```
    ///
    /// This will return a result set such as the following:
    ///
    /// ```json
    /// [
    ///     {
    ///         "left" : { "gameId" : 3, "id" : 2, "player" : "Agatha" },
    ///         "right" : { "id" : 3, "field" : "Bucklebury" }
    ///     },
    ///     {
    ///         "left" : { "gameId" : 2, "id" : 3, "player" : "Fred" },
    ///         "right" : { "id" : 2, "field" : "Rushock Bog" }
    ///     },
    ///     ...
    /// ]
    /// ```
    ///
    /// What you likely want is the result of using `zip` with that.
    /// For clarity, we’ll use `without` to drop the `id` field from the games
    /// table (it conflicts with the `id` field for the players
    /// and it’s redundant anyway), and we’ll order it by the games.
    ///
    /// ```
    /// # use unreql::rjson;
    /// # unreql::example(|r, conn| {
    /// r.table("players")
    ///   .eq_join("game_id", r.table("games"), ())
    ///   .without(rjson!({ "right": "id" }))
    ///   .zip()
    ///   .order_by("game_id")
    ///   .run(conn)
    /// # })
    /// ```
    ///
    /// ```json
    /// [
    ///     { "field": "Little Delving", "gameId": 1, "id": 5, "player": "Earnest" },
    ///     { "field": "Little Delving", "gameId": 1, "id": 1, "player": "George" },
    ///     { "field": "Rushock Bog", "gameId": 2, "id": 3, "player": "Fred" },
    ///     { "field": "Rushock Bog", "gameId": 2, "id": 4, "player": "Marie" },
    ///     { "field": "Bucklebury", "gameId": 3, "id": 6, "player": "Beth" },
    ///     { "field": "Bucklebury", "gameId": 3, "id": 2, "player": "Agatha" }
    /// ]
    /// ```
    ///
    /// For more information, see [Table joins in RethinkDB](https://rethinkdb.com/docs/table-joins/).
    ///
    /// ## Example
    /// Use a secondary index on the right table rather than
    /// the primary key. If players have a secondary index on their cities,
    /// we can get a list of arenas with players in the same area.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("players")
    ///   .eq_join("game_id", r.table("games"), r.index("cityId"))
    ///   .run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Use a nested key as the join field. Suppose the documents
    /// in the players table were structured like this:
    ///
    /// ```json
    /// { "id": 1, "player": "George", "game": {"id": 1} },
    /// { "id": 2, "player": "Agatha", "game": {"id": 3} },
    /// ...
    /// ```
    ///
    /// Simply specify the field using the `row` command instead of a string.
    ///
    /// ```
    /// # use unreql::rjson;
    /// # unreql::example(|r, conn| {
    /// r.table("players")
    ///   .eq_join(r.row().g("game").g("id"), r.table("games"), ())
    ///   .without(rjson!({ "right": "id" }))
    ///   .zip()
    ///   .run(conn)
    /// # })
    /// ```
    ///
    /// ```json
    /// [
    ///     { "field": "Little Delving", "game": { "id": 1 }, "id": 5, "player": "Earnest" },
    ///     { "field": "Little Delving", "game": { "id": 1 }, "id": 1, "player": "George" },
    ///     ...
    /// ]
    /// ```
    ///
    /// ## Example
    /// Use a function instead of a field to join on a more complicated
    /// expression. Suppose the players have lists of favorite games ranked
    /// in order in a field such as `favorites: [3, 2, 1]`. Get a list of players
    /// and their top favorite:
    ///
    /// ```
    /// # use unreql::{func, rjson};
    /// # unreql::example(|r, conn| {
    /// r.table("players")
    ///   .eq_join(
    ///     func!(|player| {
    ///       player.g("favorites").nth(0)
    ///     }),
    ///     r.table("games"),
    ///     ()
    ///   )
    ///   .without(rjson!([{ "left": ["favorites", "gameId", "id"]}, { "right": "id"}]))
    ///   .zip()
    ///   .run(conn)
    /// # })
    /// ```
    ///
    /// Result:
    ///
    /// ```json
    /// [
    /// 	{ "field": "Rushock Bog", "name": "Fred" },
    /// 	{ "field": "Little Delving", "name": "George" },
    /// 	...
    /// ]
    /// ```
    ///
    /// # Related commands
    /// - [inner_join](Self::inner_join)
    /// - [outer_join](Self::outer_join)
    /// - [without](Self::without)
    /// - [zip](Self::zip)
    only_command,
    eq_join(left_field: Serialize, right_table: Serialize, opt: Opt<Index>)
);

create_cmd!(
    /// Used to ‘zip’ up the result of a join by merging the ‘right’ fields into ‘left’ fields of each member of the sequence.
    ///
    /// ## Example
    /// ‘zips up’ the sequence by merging the left and right fields produced by a join.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("marvel")
    ///   .eq_join("main_dc_collaborator", r.table("dc"), ())
    ///   .zip()
    ///   .run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [eq_join](Self::eq_join)
    /// - [inner_join](Self::inner_join)
    /// - [outer_join](Self::outer_join)
    only_command,
    zip,
);
