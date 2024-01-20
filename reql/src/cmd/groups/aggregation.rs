use ql2::term::TermType;
use serde::Serialize;
use unreql_macros::create_cmd;

use crate::{
    cmd::{
        args::{Arg, ManyArgs, OneAndSecondOptionalArg, Opt},
        options::{FoldOptions, GroupOptions, Index},
    },
    Command,
};

create_cmd!(
    /// Takes a stream and partitions it into multiple groups based on the fields
    /// or functions provided.
    ///
    /// With the `multi` flag single documents can be assigned to multiple groups,
    /// similar to the behavior of [multi-indexes](https://rethinkdb.com/docs/secondary-indexes/javascript).
    /// When `multi` is `true` and the grouping value is an array, documents will
    /// be placed in each group that corresponds to the elements of the array.
    /// If the array is empty the row will be ignored.
    ///
    /// Suppose that the table games has the following data:
    ///
    /// ```json
    /// [
    ///     {"id": 2, "player": "Bob", "points": 15, "type": "ranked"},
    ///     {"id": 5, "player": "Alice", "points": 7, "type": "free"},
    ///     {"id": 11, "player": "Bob", "points": 10, "type": "free"},
    ///     {"id": 12, "player": "Alice", "points": 2, "type": "free"}
    /// ]
    /// ```
    ///
    /// ## Example
    /// Group games by player.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("games").group("player").run(conn)
    /// # })
    /// ```
    ///
    /// Result:
    ///
    /// ```json
    /// [
    ///     {
    ///         "group": "Alice",
    ///         "values": [
    ///             {"id": 5, "player": "Alice", "points": 7, "type": "free"},
    ///             {"id": 12, "player": "Alice", "points": 2, "type": "free"}
    ///         ]
    ///     },
    ///     {
    ///         "group": "Bob",
    ///         "values": [
    ///             {"id": 2, "player": "Bob", "points": 15, "type": "ranked"},
    ///             {"id": 11, "player": "Bob", "points": 10, "type": "free"}
    ///         ]
    ///     }
    /// ]
    /// ```
    ///
    /// Commands chained after `group` will be called on each of these grouped
    /// sub-streams, producing grouped data.
    ///
    /// ## Example
    /// What is each player’s best game?
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("games").group("player").max("points").run(conn)
    /// # })
    /// ```
    ///
    /// Result:
    ///
    /// ```json
    /// [
    ///     {
    ///         "group": "Alice",
    ///         "values": {"id": 5, "player": "Alice", "points": 7, "type": "free"}
    ///     },
    ///     {
    ///         "group": "Bob",
    ///         "values": {"id": 2, "player": "Bob", "points": 15, "type": "ranked"}
    ///     }
    /// ]
    /// ```
    ///
    /// Commands chained onto grouped data will operate on each grouped datum,
    /// producing more grouped data.
    ///
    /// ## Example
    /// What is the maximum number of points scored by each player?
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("games").group("player").max("points").g("points").run(conn)
    /// # })
    /// ```
    ///
    /// Result:
    ///
    /// ```json
    /// [
    ///     {
    ///         "group": "Alice",
    ///         "values": 7
    ///     },
    ///     {
    ///         "group": "Bob",
    ///         "values": 15
    ///     }
    /// ]
    /// ```
    ///
    /// You can also group by more than one field.
    ///
    /// ## Example
    /// What is the maximum number of points scored by each player for
    /// each game type?
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("games")
    ///   .group(r.args(("player", "type")))
    ///   .max("points")
    ///   .g("points")
    ///   .run(conn)
    /// # })
    /// ```
    ///
    /// Result:
    ///
    /// ```json
    /// [
    ///     {
    ///         "group": ["Alice", "free"],
    ///         "values": 7
    ///     },
    ///     {
    ///         "group": ["Bob", "free"],
    ///         "values": 10
    ///     },
    ///     {
    ///         "group": ["Bob", "ranked"],
    ///         "values": 15
    ///     }
    /// ]
    /// ```
    ///
    /// You can also group by a function.
    ///
    /// ## Example
    /// What is the maximum number of points scored by each player for each game type?
    ///
    /// ```
    /// # use unreql::func;
    /// # unreql::example(|r, conn| {
    /// r.table("games")
    ///   .group(func!(|game| {
    ///     game.pluck(r.args(("player", "type")))
    ///   }))
    ///   .max("points")
    ///   .g("points")
    ///   .run(conn)
    /// # })
    /// ```
    ///
    /// Result:
    ///
    /// ```json
    /// [
    ///     {
    ///         "group": {"player": "Alice", "type": "free"},
    ///         "values": 7
    ///     },
    ///     {
    ///         "group": {"player": "Bob", "type": "free"},
    ///         "values": 10
    ///     },
    ///     {
    ///         "group": {"player": "Bob", "type": "ranked"},
    ///         "values": 15
    ///     }
    /// ]
    /// ```
    ///
    /// Using a function, you can also group by date on a ReQL [date field](https://rethinkdb.com/docs/dates-and-times/javascript/).
    ///
    /// ## Example
    /// How many matches have been played this year by month?
    ///
    /// ```
    /// # use unreql::func;
    /// # unreql::example(|r, conn| {
    /// r.table("matches")
    ///   .group([r.row().g("date").year(), r.row().g("date").month()])
    ///   .count(())
    ///   .run(conn)
    /// # })
    /// ```
    ///
    /// Result:
    ///
    /// ```json
    /// [
    ///     {
    ///         "group": [2014, 2],
    ///         "values": 2
    ///     },
    ///     {
    ///         "group": [2014, 3],
    ///         "values": 2
    ///     },
    ///     {
    ///         "group": [2014, 4],
    ///         "values": 1
    ///     },
    ///     {
    ///         "group": [2014, 5],
    ///         "values": 3
    ///     }
    /// ]
    /// ```
    ///
    /// You can also group on an index (primary key or secondary).
    ///
    /// ## Example
    /// What is the maximum number of points scored by game type?
    ///
    /// ```
    /// # use unreql::func;
    /// # use unreql::cmd::options::GroupOptions;
    /// # unreql::example(|r, conn| {
    /// r.table("games")
    ///   .group(r.with_opt((), GroupOptions::new().index("type".into())))
    ///   .max("points")
    ///   .g("points")
    ///   .run(conn)
    /// # })
    /// ```
    ///
    /// Result:
    ///
    /// ```json
    /// [
    ///     {
    ///         "group": "free",
    ///         "values": 10
    ///     },
    ///     {
    ///         "group": "ranked",
    ///         "values": 15
    ///     }
    /// ]
    /// ```
    ///
    /// See more details on the [javascript api documentation](https://rethinkdb.com/api/javascript/group#organizing-by-value-with-multi).
    ///
    /// # More Examples
    ///
    /// ## Example
    /// What is the maximum number of points scored by each player in free games?
    ///
    /// ```
    /// # use unreql::func;
    /// # unreql::example(|r, conn| {
    /// r.table("games")
    ///   .filter(r.row().g("type").eq("free"))
    ///   .group("player")
    ///   .max("points")
    ///   .g("points")
    ///   .run(conn)
    /// # })
    /// ```
    ///
    /// Result:
    ///
    /// ```json
    /// [
    ///     {
    ///         "group": "Alice",
    ///         "values": 7
    ///     },
    ///     {
    ///         "group": "Bob",
    ///         "values": 10
    ///     }
    /// ]
    /// ```
    ///
    /// ## Example
    /// What is each player’s highest even and odd score?
    ///
    /// ```
    /// # use unreql::func;
    /// # unreql::example(|r, conn| {
    /// r.table("games")
    ///   .group(r.args((
    ///     "name",
    ///     func!(|game| game.g("points").mod_(2))
    ///   )))
    ///   .max("points")
    ///   .g("points")
    ///   .run(conn)
    /// # })
    /// ```
    ///
    /// Result:
    ///
    /// ```json
    /// [
    ///     {
    ///         "group": ["Alice", 1],
    ///         "values": 7
    ///     },
    ///     {
    ///         "group": ["Bob", 0],
    ///         "values": 10
    ///     },
    ///     {
    ///         "group": ["Bob", 1],
    ///         "values": 15
    ///     }
    /// ]
    /// ```
    ///
    /// # Related commands
    /// - [ungroup](Self::ungroup)
    /// - [map](Self::map)
    /// - [reduce](Self::reduce)
    /// - [count](Self::count)
    /// - [sum](Self::sum)
    /// - [avg](Self::avg)
    /// - [min](Self::min)
    /// - [max](Self::max)
    group(field_or_function: ManyArgs<GroupOptions>)
);

create_cmd!(
    /// Takes a grouped stream or grouped data and turns it into an array of
    /// objects representing the groups. Any commands chained after `ungroup`
    /// will operate on this array, rather than operating on each group
    /// individually. This is useful if you want to e.g. order the groups by
    /// the value of their reduction.
    ///
    /// The format of the array returned by `ungroup` is the same as the default
    /// native format of grouped data in the javascript driver and data explorer.
    ///
    /// Suppose that the table `games` has the following data:
    ///
    /// ```json
    /// [
    ///     {"id": 2, "player": "Bob", "points": 15, "type": "ranked"},
    ///     {"id": 5, "player": "Alice", "points": 7, "type": "free"},
    ///     {"id": 11, "player": "Bob", "points": 10, "type": "free"},
    ///     {"id": 12, "player": "Alice", "points": 2, "type": "free"}
    /// ]
    /// ```
    ///
    /// ## Example
    /// What is the maximum number of points scored by each player, with the highest scorers first?
    ///
    /// ```
    /// # use unreql::func;
    /// # unreql::example(|r, conn| {
    /// r.table("games")
    ///   .group("player")
    ///   .max("points")
    ///   .g("points")
    ///   .ungroup()
    ///   .order_by(r.desc("reduction"))
    ///   .run(conn)
    /// # })
    /// ```
    ///
    /// Result:
    ///
    /// ```json
    /// [
    ///     {
    ///         "group": "Bob",
    ///         "reduction": 15
    ///     },
    ///     {
    ///         "group": "Alice",
    ///         "reduction": 7
    ///     }
    /// ]
    /// ```
    ///
    /// ## Example
    /// Select one random player and all their games.
    ///
    /// ```
    /// # use unreql::func;
    /// # unreql::example(|r, conn| {
    /// r.table("games").group("player").ungroup().sample(1).run(conn)
    /// # })
    /// ```
    ///
    /// Result:
    ///
    /// ```json
    /// [
    ///     {
    ///         "group": "Bob",
    ///         "reduction": 15
    ///     },
    ///     {
    ///         "group": "Alice",
    ///         "reduction": 7
    ///     }
    /// ]
    /// ```
    ///
    /// *Note* that if you didn’t call `ungroup`, you would instead select one
    /// random game from each player:
    ///
    /// ```
    /// # use unreql::func;
    /// # unreql::example(|r, conn| {
    /// r.table("games").group("player").sample(1).run(conn)
    /// # })
    /// ```
    ///
    /// Result: (Note this is a JSON representation of a List<GroupedResult>; see the group documentation for more details.)
    ///
    /// ```json
    /// [
    ///     {
    ///         "group": "Alice",
    ///         "values": [
    ///             {"id": 5, "player": "Alice", "points": 7, "type": "free"}
    ///         ]
    ///     },
    ///     {
    ///         "group": "Bob",
    ///         "values": [
    ///             {"id": 11, "player": "Bob", "points": 10, "type": "free"}
    ///         ]
    ///     }
    /// ]
    /// ```
    ///
    /// ## Example
    /// Finding the arithmetic mode of an array of values:
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.expr([1,2,2,2,3,3])
    ///   .group(r.row())
    ///   .count(())
    ///   .ungroup()
    ///   .order_by("reduction")
    ///   .nth(-1)
    ///   .g("group")
    ///   .run(conn)
    /// // Result: 2
    /// # })
    /// ```
    ///
    /// ## Example
    /// Types!
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// // Returns "GROUPED_STREAM"
    /// r.table("games").group("player").type_of().run(conn)
    /// # });
    ///
    /// # unreql::example(|r, conn| {
    /// // Returns "ARRAY"
    /// r.table("games").group("player").ungroup().type_of().run(conn)
    /// # });
    ///
    /// # unreql::example(|r, conn| {
    /// // Returns "GROUPED_DATA"
    /// r.table("games").group("player").avg("points").run(conn)
    /// # });
    ///
    /// # unreql::example(|r, conn| {
    /// // Returns "ARRAY"
    /// r.table("games").group("player").avg("points").ungroup().run(conn)
    /// # });
    /// ```
    ///
    /// # Related commands
    /// - [group](Self::group)
    ungroup,
);

create_cmd!(
    /// Produce a single value from a sequence through repeated application
    /// of a reduction function.
    ///
    /// The reduction function can be called on:
    ///
    /// - two elements of the sequence
    /// - one element of the sequence and one result of a previous reduction
    /// - two results of previous reductions
    ///
    /// The reduction function can be called on the results of two previous
    /// reductions because the `reduce` command is distributed and parallelized
    /// across shards and CPU cores. A common mistaken when using the `reduce`
    /// command is to suppose that the reduction is executed from left to right.
    /// Read the [map-reduce in RethinkDB](https://rethinkdb.com/docs/map-reduce/) article to see an example.
    ///
    /// If the sequence is empty, the server will produce a `ReqlRuntimeError`
    /// that can be caught with `default`.
    /// If the sequence has only one element, the first element will be returned.
    ///
    /// ## Example
    /// Return the number of documents in the table posts.
    ///
    /// ```
    /// # use unreql::func;
    /// # unreql::example(|r, conn| {
    /// r.table("posts")
    ///   .map(func!(|doc| r.expr(1)))
    ///   .reduce(func!(|left, right| left.add(right)))
    ///   .default(0)
    ///   .run(conn)
    /// # })
    /// ```
    ///
    /// A shorter way to execute this query is to use count.
    ///
    /// ## Example
    /// Suppose that each `post` has a field `comments` that is an array
    /// of comments. Return the number of comments for all posts.
    ///
    /// ```
    /// # use unreql::func;
    /// # unreql::example(|r, conn| {
    /// r.table("posts")
    ///   .map(func!(|doc| doc.g("comments").count(())))
    ///   .reduce(func!(|left, right| left.add(right)))
    ///   .default(0)
    ///   .run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Suppose that each `post` has a field `comments` that is an array of
    /// comments. Return the maximum number comments per post.
    ///
    /// ```
    /// # use unreql::func;
    /// # unreql::example(|r, conn| {
    /// r.table("posts")
    ///   .map(func!(|doc| doc.g("comments").count(())))
    ///   .reduce(func!(|left, right| left.add(right)))
    ///   .default(0)
    ///   .run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Suppose that each `post` has a field `comments` that is an array
    /// of comments. Return the maximum number comments per post.
    ///
    /// ```
    /// # use unreql::func;
    /// # unreql::example(|r, conn| {
    /// r.table("posts")
    ///   .map(func!(|doc| doc.g("comments").count(())))
    ///   .reduce(func!(|left, right|
    ///     r.branch(
    ///       left.clone().gt(right.clone()),
    ///       left,
    ///       right
    ///     )
    ///   ))
    ///   .default(0)
    ///   .run(conn)
    /// # })
    /// ```
    ///
    /// A shorter way to execute this query is to use max.
    ///
    /// # Related commands
    /// - [group](Self::group)
    /// - [map](Self::map)
    /// - [concat_map](Self::concat_map)
    /// - [sum](Self::sum)
    /// - [avg](Self::avg)
    /// - [min](Self::min)
    /// - [max](Self::max)
    only_command,
    reduce(function: OneAndSecondOptionalArg<()>)
);

create_cmd!(
    /// Produce a single value from a sequence through repeated application
    /// of a reduction function.
    ///
    /// See [reduce](Command::reduce) in Command.
    only_root,
    reduce(sequence: Serialize, function: Serialize)
);

create_cmd!(
    /// Apply a function to a sequence in order, maintaining state via an
    /// accumulator. The `fold` command returns either a single value or a new
    /// sequence.
    ///
    /// In its first form, `fold` operates like [reduce](Command::reduce),
    /// returning a value by applying a combining function to each element
    /// in a sequence. The combining function takes two parameters: the
    /// previous reduction result (the accumulator) and the current element.
    /// However, fold has the following differences from reduce:
    ///
    /// - it is guaranteed to proceed through the sequence from first element
    ///   to last.
    /// - it passes an initial base value to the function with the first
    ///   element in place of the previous reduction result.
    ///
    /// `combiningFunction(accumulator | base, element) → newAccumulator`
    ///
    /// In its second form, `fold` operates like [concat_map](Command::concat_map),
    /// returning a new sequence rather than a single value. When an `emit`
    /// function is provided, `fold` will:
    ///
    /// - proceed through the sequence in order and take an initial base value,
    ///   as above.
    /// - for each element in the sequence, call both the combining function
    ///   and a separate emitting function. The emitting function takes three
    ///   parameters: the previous reduction result (the accumulator),
    ///   the current element, and the output from the combining function
    ///   (the new value of the accumulator).
    ///
    /// If provided, the emitting function must return a list.
    ///
    /// `emit(previousAccumulator, element, accumulator) → array`
    ///
    /// A `final_emit` function may also be provided, which will be called at
    /// the end of the sequence. It takes a single parameter: the result of the
    /// last reduction through the iteration (the accumulator), or the original
    /// base value if the input sequence was empty. This function must return
    /// a list, which will be appended to `fold`’s output stream.
    ///
    /// `final_emit(accumulator | base) → array`
    ///
    /// ## Example
    /// Concatenate words from a list.
    ///
    /// ```
    /// # use unreql::func;
    /// # unreql::example(|r, conn| {
    /// r.table("words")
    ///   .order_by("id")
    ///   .fold("", func!(|acc, word| {
    ///     acc.clone().add(r.branch(acc.eq(""), "", ", ")).add(word)
    ///   }), ())
    ///   .run(conn)
    /// # })
    /// ```
    ///
    /// (This example could be implemented with `reduce`, but `fold` will
    /// preserve the order when `words` is a RethinkDB table or other stream,
    /// which is not guaranteed with `reduce`.)
    ///
    /// ## Example
    /// Return every other row in a table.
    ///
    /// ```
    /// # use unreql::{func, rjson};
    /// # use unreql::cmd::options::FoldOptions;
    /// # unreql::example(|r, conn| {
    /// r.table("even_things")
    ///   .order_by("id")
    ///   .fold(
    ///     0,
    ///     func!(|acc, row| {
    ///       acc.add(1)
    ///     }),
    ///     FoldOptions::new().emit(func!(|acc, row, new_acc| {
    ///       r.branch(new_acc.mod_(2).eq(0), [row], rjson!([]))
    ///     }))
    ///   )
    ///   .run(conn)
    /// # })
    /// ```
    ///
    /// The first function increments the accumulator each time it’s called,
    /// starting at `0`; the second function, the emitting function, alternates
    /// between returning a single-item list containing the current row or an
    /// empty list. The `fold` command will return a concatenated list of each
    /// emitted value.
    ///
    /// ## Example
    /// Compute a five-day running average for a weight tracker.
    ///
    /// ```
    /// # use unreql::{func, rjson};
    /// # use unreql::cmd::options::FoldOptions;
    /// # unreql::example(|r, conn| {
    /// r.table("tracker")
    ///   .filter(rjson!({"name": "bob"}))
    ///   .order_by("date")
    ///   .g("weight")
    ///   .fold(
    ///     rjson!([]),
    ///     func!(|acc, row| {
    ///       r.expr([row]).add(acc).limit(5)
    ///     }),
    ///     FoldOptions::new().emit(func!(|acc, row, new_acc| {
    ///       r.branch(new_acc.clone().count(()).eq(5), [new_acc.avg(())], rjson!([]))
    ///     }))
    ///   )
    ///   .run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [reduce](Self::reduce)
    /// - [concat_map](Self::concat_map)
    fold(base: Serialize, function: Serialize, opt: Opt<FoldOptions>)
);

create_cmd!(
    /// Counts the number of elements in a sequence or key/value pairs in an
    /// object, or returns the size of a string or binary object.
    ///
    /// When `count` is called on a sequence with a predicate value or function,
    /// it returns the number of elements in the sequence equal to that value
    /// or where the function returns `true`. On a `binary` object, `count`
    /// returns the size of the object in bytes; on strings, `count` returns
    /// the string’s length. This is determined by counting the number of
    /// Unicode codepoints in the string, counting combining codepoints
    /// separately.
    ///
    /// ## Example
    /// Count the number of users.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("users").count(()).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Count the number of 18 year old users.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("users").g("age").count(18).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Count the number of users over 18.
    ///
    /// ```
    /// # use unreql::func;
    /// # unreql::example(|r, conn| {
    /// r.table("users").g("age").count(func!(|age| age.gt(18))).run(conn)
    /// # })
    /// ```
    ///
    /// ```
    /// # use unreql::func;
    /// # unreql::example(|r, conn| {
    /// r.table("users").count(func!(|user| user.g("age").gt(18))).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Return the length of a Unicode string.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.expr("こんにちは").count(()).run(conn)
    /// // Return: 5
    /// # })
    /// ```
    ///
    /// ## Example
    /// Return the length of an array.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.expr(["0","1","2"]).count(()).run(conn)
    /// // Return: 3
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [map](Self::map)
    /// - [reduce](Self::reduce)
    /// - [sum](Self::sum)
    /// - [avg](Self::avg)
    /// - [min](Self::min)
    /// - [max](Self::max)
    /// - [group](Self::group)
    count(value: ManyArgs<()>)
);

create_cmd!(
    /// Sums all the elements of a sequence.
    ///
    /// If called with a field name, sums all the values of that field in the
    /// sequence, skipping elements of the sequence that lack that field. If
    /// called with a function, calls that function on every element of the
    /// sequence and sums the results, skipping elements of the sequence where
    /// that function returns `null` or a non-existence error.
    ///
    /// Returns `0` when called on an empty sequence.
    ///
    /// ## Example
    /// What’s 3 + 5 + 7?
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.expr([3, 5, 7]).sum(()).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// How many points have been scored across all games?
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("games").sum("points").run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// How many points have been scored across all games, counting bonus points?
    ///
    /// ```
    /// # use unreql::func;
    /// # unreql::example(|r, conn| {
    /// r.table("games").sum(func!(|game| {
    ///   game.clone().g("points").add(game.g("bonus_points"))
    /// })).run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [map](Self::map)
    /// - [reduce](Self::reduce)
    /// - [count](Self::count)
    /// - [avg](Self::avg)
    /// - [min](Self::min)
    /// - [max](Self::max)
    /// - [group](Self::group)
    only_command,
    sum(field_or_function: Serialize)
);

create_cmd!(
    /// Sums all the elements of a sequence.
    ///
    /// See [sum](Command::sum) in Command.
    only_root,
    sum(sequence: Serialize, field: Serialize)
);

create_cmd!(
    /// Averages all the elements of a sequence.
    ///
    /// If called with a field name, averages all the values of that field in
    /// the sequence, skipping elements of the sequence that lack that field.
    /// If called with a function, calls that function on every element of the
    /// sequence and averages the results, skipping elements of the sequence
    /// where that function returns `null` or a non-existence error.
    ///
    /// Produces a non-existence error when called on an empty sequence. You
    /// can handle this case with `default`.
    ///
    /// ## Example
    /// What’s the average of 3, 5, and 7?
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.expr([3, 5, 7]).avg(()).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// What’s the average number of points scored in a game?
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("games").avg("points").run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// What’s the average number of points scored in a game, counting bonus
    /// points?
    ///
    /// ```
    /// # use unreql::func;
    /// # unreql::example(|r, conn| {
    /// r.table("games").avg(func!(|game| {
    ///   game.clone().g("points").add(game.g("bonus_points"))
    /// })).run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [map](Self::map)
    /// - [reduce](Self::reduce)
    /// - [count](Self::count)
    /// - [sum](Self::sum)
    /// - [min](Self::min)
    /// - [max](Self::max)
    /// - [group](Self::group)
    only_command,
    avg(field_or_function: Serialize)
);

create_cmd!(
    /// Averages all the elements of a sequence.
    ///
    /// See [avg](Command::avg) in Command.
    only_root,
    avg(sequence: Serialize, field: Serialize)
);

create_cmd!(
    /// Finds the minimum element of a sequence.
    ///
    /// The `min` command can be called with:
    ///
    /// - a `field name`, to return the element of the sequence with the
    ///   smallest value in that field;
    /// - an `index` (the primary key or a secondary index), to return the
    ///   element of the sequence with the smallest value in that index;
    /// - a `function`, to apply the function to every element within the
    ///   sequence and return the element which returns the smallest value
    ///   from the function, ignoring any elements where the function
    ///   produces a non-existence error.
    ///
    /// For more information on RethinkDB’s sorting order, read the section in
    /// [ReQL data types](https://rethinkdb.com/docs/data-types/#sorting-order).
    ///
    /// Calling `min` on an empty sequence will throw a non-existence error;
    /// this can be handled using the `default` command.
    ///
    /// ## Example
    /// Return the minimum value in the list `[3, 5, 7]`.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.expr([3, 5, 7]).min(()).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Return the user who has scored the fewest points.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("users").min("points").run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// The same as above, but using a secondary index on the points field.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("users").min(r.index("points")).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Return the user who has scored the fewest points, adding in bonus points
    /// from a separate field using a function.
    ///
    /// ```
    /// # use unreql::func;
    /// # unreql::example(|r, conn| {
    /// r.table("users").min(func!(|user| {
    ///   user.clone().g("points").add(user.g("bonusPoints"))
    /// })).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Return the smallest number of points any user has ever scored. This
    /// returns the value of that `points` field, not a document.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("users").min("points").g("points").run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Return the user who has scored the fewest points, but add a default
    /// `null` return value to prevent an error if no user has ever scored points.
    ///
    /// ```
    /// # use unreql::rjson;
    /// # unreql::example(|r, conn| {
    /// r.table("users").min("points").default(rjson!(null)).run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [map](Self::map)
    /// - [reduce](Self::reduce)
    /// - [count](Self::count)
    /// - [sum](Self::sum)
    /// - [avg](Self::avg)
    /// - [max](Self::max)
    /// - [group](Self::group)
    only_command,
    min(field_or_function: Arg<Index>)
);

create_cmd!(
    /// Finds the minimum element of a sequence.
    ///
    /// See [min](Command::min) in Command.
    only_root,
    min(sequence: Serialize, field: Arg<Index>)
);

create_cmd!(
    /// Finds the maximum element of a sequence.
    ///
    /// The `max` command can be called with:
    ///
    /// - a `field name`, to return the element of the sequence with the
    ///   largest value in that field;
    /// - an `index` (the primary key or a secondary index), to return the
    ///   element of the sequence with the largest value in that index;
    /// - a `function`, to apply the function to every element within the
    ///   sequence and return the element which returns the largest value
    ///   from the function, ignoring any elements where the function
    ///   produces a non-existence error.
    ///
    /// For more information on RethinkDB’s sorting order, read the section in
    /// [ReQL data types](https://rethinkdb.com/docs/data-types/#sorting-order).
    ///
    /// Calling `max` on an empty sequence will throw a non-existence error;
    /// this can be handled using the `default` command.
    ///
    /// ## Example
    /// Return the maximum value in the list `[3, 5, 7]`.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.expr([3, 5, 7]).max(()).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Return the user who has scored the most points.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("users").max("points").run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// The same as above, but using a secondary index on the points field.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("users").max(r.index("points")).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Return the user who has scored the most points, adding in bonus points
    /// from a separate field using a function.
    ///
    /// ```
    /// # use unreql::func;
    /// # unreql::example(|r, conn| {
    /// r.table("users").max(func!(|user| {
    ///   user.clone().g("points").add(user.g("bonusPoints"))
    /// })).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Return the highest number of points any user has ever scored. This
    /// returns the value of that `points` field, not a document.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("users").max("points").g("points").run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Return the user who has scored the most points, but add a default
    /// `null` return value to prevent an error if no user has ever scored points.
    ///
    /// ```
    /// # use unreql::rjson;
    /// # unreql::example(|r, conn| {
    /// r.table("users").max("points").default(rjson!(null)).run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [map](Self::map)
    /// - [reduce](Self::reduce)
    /// - [count](Self::count)
    /// - [sum](Self::sum)
    /// - [avg](Self::avg)
    /// - [min](Self::min)
    /// - [group](Self::group)
    only_command,
    max(field_or_function: Arg<Index>)
);

create_cmd!(
    /// Finds the maximum element of a sequence.
    ///
    /// See [max](Command::max) in Command.
    only_root,
    max(sequence: Serialize, field: Arg<Index>)
);

create_cmd!(
    /// Removes duplicates from elements in a sequence.
    ///
    /// The `distinct` command can be called on any sequence or table with an index.
    ///
    /// *Note*. While `distinct` can be called on a table without an index, the only
    /// effect will be to convert the table into a stream; the content of
    /// the stream will not be affected.
    ///
    /// ## Example
    /// Which unique villains have been vanquished by Marvel heroes?
    ///
    /// ```
    /// # use unreql::func;
    /// # unreql::example(|r, conn| {
    /// r.table("marvel")
    ///   .concat_map(func!(|hero| hero.g("villainList")))
    ///   .distinct(())
    ///   .run(conn)
    /// # })
    /// ```
    ///
    /// ## Example: Topics in a table of messages have a secondary index on them,
    /// and more than one message can have the same topic. What are the unique
    /// topics in the table?
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("messages").distinct(r.index("topics")).run(conn)
    /// # })
    /// ```
    ///
    /// The above structure is functionally identical to:
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("messages").g("topics").distinct(()).run(conn)
    /// # })
    /// ```
    ///
    /// However, the first form (passing the index as an argument to `distinct`)
    /// is faster, and won’t run into array limit issues since it’s returning
    /// a stream.
    ///
    /// # Related commands
    /// - [map](Self::map)
    /// - [concat_map](Self::concat_map)
    /// - [group](Self::group)
    distinct,
    ManyArgs<Index>
);

create_cmd!(
    /// When called with values, returns `true` if a sequence contains all the
    /// specified values.
    ///
    /// When called with predicate functions, returns `true` if for each predicate
    /// there exists at least one element of the stream where that predicate
    /// returns `true`.
    ///
    /// Values and predicates may be mixed freely in the argument list.
    ///
    /// ## Example
    /// Has Iron Man ever fought Superman?
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("marvel").get("ironman").g("opponents").contains("superman").run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Has Iron Man ever defeated Superman in battle?
    ///
    /// ```
    /// # use unreql::func;
    /// # unreql::example(|r, conn| {
    /// r.table("marvel").get("ironman").g("battles").contains(func!(|battle| {
    ///   battle.clone().g("winner").eq("ironman").and(battle.g("loser").eq("superman"))
    /// })).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Return all heroes who have fought both Loki and the Hulk.
    ///
    /// ```
    /// # use unreql::func;
    /// # unreql::example(|r, conn| {
    /// r.table("marvel").filter(func!(|hero| {
    ///   hero.g("opponents").contains(r.args(("loki", "hulk")))
    /// })).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Use `contains` with a predicate function to simulate an `or`. Return the
    /// Marvel superheroes who live in Detroit, Chicago or Hoboken.
    ///
    /// ```
    /// # use unreql::func;
    /// # unreql::example(|r, conn| {
    /// r.table("marvel").filter(func!(|hero| {
    ///   r.expr(["Detroit", "Chicago", "Hoboken"]).contains(hero.g("city"))
    /// })).run(conn)
    /// # })
    /// ```
    contains(value: ManyArgs<()>)
);
