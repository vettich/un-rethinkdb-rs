use ql2::term::TermType;
use serde::Serialize;
use unreql_macros::create_cmd;

use crate::{
    cmd::{
        args::{ManyArgs, OneAndSecondOptionalArg},
        options::{Index, SliceOptions, UnionOptions},
    },
    Command,
};

create_cmd!(
    /// Transform each element of one or more sequences by applying a mapping function to them.
    ///
    /// If `map` is run with two or more sequences, it will iterate for as many
    /// items as there are in the shortest sequence.
    ///
    /// *Note* that `map` can only be applied to sequences, not single values.
    /// If you wish to apply a function to a single value/selection (including
    /// an array), use the [do_](Self::do_) command.
    ///
    /// Example
    /// Return the first five squares.
    ///
    /// ```
    /// # use unreql::func;
    /// # unreql::example(|r, conn| {
    /// r.expr([1, 2, 3, 4, 5])
    ///   .map(func!(|val| val.clone().mul(val)))
    ///   .run(conn)
    /// // Result: [1, 4, 9, 16, 25]
    /// # })
    /// ```
    ///
    /// Example
    /// Sum the elements of three sequences.
    ///
    /// ```
    /// # use unreql::func;
    /// # unreql::example(|r, conn| {
    /// let sequence1 = [100, 200, 300, 400];
    /// let sequence2 = [10, 20, 30, 40];
    /// let sequence3 = [1, 2, 3, 4];
    /// r.map(r.args((sequence1, sequence2, sequence3, func!(|val1, val2, val3| {
    ///     val1.add(val2).add(val3)
    ///   }))))
    ///   .run(conn)
    /// // Result: [1, 4, 9, 16, 25]
    /// # })
    /// ```
    ///
    /// ## Example
    /// Rename a field when retrieving documents using `map` and [merge](Self::merge).
    ///
    /// This example renames the field `id` to `userId` when retrieving documents
    /// from the table `users`.
    ///
    /// ```
    /// # use unreql::{func, rjson};
    /// # unreql::example(|r, conn| {
    /// r.table("users").map(func!(|doc| {
    ///     doc.clone().merge(rjson!({"userId": doc.g("id")})).without("id")
    ///   }))
    ///   .run(conn)
    /// # })
    /// ```
    ///
    /// Note that in this case, `row` may be used as an alternative to writing
    /// an anonymous function, as it returns the same value as the function parameter receives:
    ///
    /// ```
    /// # use unreql::rjson;
    /// # unreql::example(|r, conn| {
    /// r.table("users").map(
    ///     r.row().merge(rjson!({"userId": r.row().g("id")})).without("id")
    /// ).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Assign every superhero an archenemy.
    ///
    /// ```
    /// # use unreql::{func, rjson};
    /// # unreql::example(|r, conn| {
    /// r.table("heroes").map(r.args((r.table("villains"), func!(|hero, villain| {
    ///     hero.merge(rjson!({"villain": villain}))
    ///   }))))
    ///   .run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [concat_map](Self::concat_map)
    /// - [reduce](Self::reduce)
    /// - [do_](Self::do_)
    map(function: ManyArgs<()>)
);

create_cmd!(
    /// Plucks one or more attributes from a sequence of objects,
    /// filtering out any objects in the sequence that do not have
    /// the specified fields.
    ///
    /// Functionally, this is identical to [has_fields](Self::has_fields)
    /// followed by [pluck](Self::pluck) on a sequence.
    ///
    /// ## Example: Get a list of users and their posts, excluding any users who have not made any posts.
    ///
    /// Existing table structure:
    ///
    /// ```json
    /// [
    ///     { "id": 1, "user": "bob", "email": "bob@foo.com", "posts": [ 1, 4, 5 ] },
    ///     { "id": 2, "user": "george", "email": "george@foo.com" },
    ///     { "id": 3, "user": "jane", "email": "jane@foo.com", "posts": [ 2, 3, 6 ] }
    /// ]
    /// ```
    ///
    /// Command and output:
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("users").with_fields(r.args(["id", "user", "posts"])).run(conn)
    ///
    /// // Result
    /// // [
    /// //     { "id": 1, "user": "bob", "posts": [ 1, 4, 5 ] },
    /// //     { "id": 3, "user": "jane", "posts": [ 2, 3, 6 ] }
    /// // ]
    /// # })
    /// ```
    ///
    /// ## Example
    /// Use the [nested field syntax](https://rethinkdb.com/docs/nested-fields/javascript/)
    /// to get a list of users with cell phone numbers in their contacts.
    ///
    /// ```
    /// # use unreql::rjson;
    /// # unreql::example(|r, conn| {
    /// r.table("users").with_fields(r.args(("id", "user", rjson!({"contact": {"phone": "work"}})))).run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [pluck](Self::pluck)
    /// - [has_fields](Self::has_fields)
    /// - [without](Self::without)
    only_command,
    with_fields(selector: ManyArgs<()>)
);

create_cmd!(
    /// Concatenate one or more elements into a single sequence using a mapping function.
    ///
    /// `concat_map` works in a similar fashion to [map](Self::map), applying
    /// the given function to each element in a sequence, but it will always
    /// return a single sequence. If the mapping function returns a sequence,
    /// `map` would produce a sequence of sequences:
    ///
    /// ```
    /// # use unreql::func;
    /// # unreql::example(|r, conn| {
    /// r.expr([1, 2, 3]).map(func!(|x| r.array([x.clone(), x.mul(2)]))).run(conn)
    /// // Result
    /// // [[1, 2], [2, 4], [3, 6]]
    /// # })
    /// ```
    ///
    /// Whereas `concat_map` with the same mapping function would merge those sequences into one:
    ///
    /// ```
    /// # use unreql::func;
    /// # unreql::example(|r, conn| {
    /// r.expr([1, 2, 3]).concat_map(func!(|x| r.array([x.clone(), x.mul(2)]))).run(conn)
    /// // Result
    /// // [1, 2, 2, 4, 3, 6]
    /// # })
    /// ```
    ///
    /// The return value, array or stream, will be the same type as the input.
    ///
    /// ## Example
    /// Construct a sequence of all monsters defeated by Marvel heroes.
    /// The field “defeatedMonsters” is an array of one or more monster names.
    ///
    /// ```
    /// # use unreql::func;
    /// # unreql::example(|r, conn| {
    /// r.table("marvel").concat_map(func!(|hero| hero.g("defeatedMonsters"))).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Simulate an [eq_join](Self::eq_join) using `concat_map`.
    /// (This is how ReQL joins are implemented internally.)
    ///
    /// ```
    /// # use unreql::{func, rjson};
    /// # unreql::example(|r, conn| {
    /// r.table("posts").concat_map(func!(|post| {
    ///   r.table("comments")
    ///     .get_all(r.with_opt(post.clone().g("id"), r.index("postId")))
    ///     .map(func!(|comment| {
    ///       rjson!({ "left": post.clone(), "right": comment })
    ///     }))
    /// })).run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [map](Self::map)
    /// - [reduce](Self::reduce)
    only_command,
    concat_map(function: Serialize)
);

create_cmd!(
    /// Sort the sequence by document values of the given key(s). To specify
    /// the ordering, wrap the attribute with either `r.asc` or `r.desc`
    /// (defaults to ascending).
    ///
    /// *Note*: RethinkDB uses byte-wise ordering for `order_by` and does not
    /// support Unicode collations; non-ASCII characters will be sorted
    /// by UTF-8 codepoint. For more information on RethinkDB’s sorting order,
    /// read the section in [ReQL data types](https://rethinkdb.com/docs/data-types/#sorting-order).
    ///
    /// Sorting without an index requires the server to hold the sequence
    /// in memory, and is limited to 100,000 documents (or the setting
    /// of the `arrayLimit` option for `run`). Sorting with an index can be
    /// done on arbitrarily large tables, or after a `between` command using
    /// the same index. This applies to both secondary indexes
    /// and the primary key (e.g., `r.index('id')`).
    ///
    /// Sorting functions passed to `order_by` must be deterministic. You cannot,
    /// for instance, order rows using the [random](Self::random) command.
    /// Using a non-deterministic function with `order_by` will raise
    /// a `ReqlQueryLogicError`.
    ///
    /// ## Example
    /// Order all the posts using the index date.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("posts").order_by(r.index("date")).run(conn)
    /// # })
    /// ```
    ///
    /// The index must either be the primary key or have been previously created
    /// with [index_create](Self::index_create).
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("posts").index_create("date").run(conn)
    /// # })
    /// ```
    ///
    /// You can also select a descending ordering:
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("posts").order_by(r.index(r.desc("date"))).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Order a sequence without an index.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("posts")
    ///   .get(1).g("comments")
    ///   .order_by("date")
    ///   .run(conn)
    /// # })
    /// ```
    ///
    /// You can also select a descending ordering:
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("posts")
    ///   .get(1).g("comments")
    ///   .order_by(r.desc("date"))
    ///   .run(conn)
    /// # })
    /// ```
    ///
    /// If you’re doing ad-hoc analysis and know your table won’t have more then
    /// 100,000 elements (or you’ve changed the setting of the `array_limit`
    /// option for `run`) you can run `order_by` without an index:
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("small_table").order_by("date").run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// You can efficiently order using multiple fields by using
    /// a [compound index](http://www.rethinkdb.com/docs/secondary-indexes/javascript/).
    ///
    /// Order by date and title.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("posts").order_by(r.index("dateAndTitle")).run(conn)
    /// # })
    /// ```
    ///
    /// The index must either be the primary key or have been previously created
    /// with [index_create](Self::indexCreate).
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("comments")
    ///   .index_create(r.args(("dateAndTitle", [r.row().g("date"), r.row().g("title")])))
    ///   .run(conn)
    /// # })
    /// ```
    ///
    /// *Note*: You cannot specify multiple orders in a compound index.
    /// See [issue #2306](https://github.com/rethinkdb/rethinkdb/issues/2306) to track progress.
    ///
    /// ## Example: If you have a sequence with fewer documents than
    /// the `arrayLimit`, you can order it by multiple fields without an index.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("small_table").order_by(r.args(("date", r.desc("title")))).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Notice that an index ordering always has highest precedence.
    /// The following query orders posts by date, and if multiple posts were
    /// published on the same date, they will be ordered by title.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("post")
    ///   .order_by(r.with_opt("title", r.index("date")))
    ///   .run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Use [nested field](https://rethinkdb.com/docs/cookbook/javascript/#filtering-based-on-nested-fields) syntax to sort on fields from subdocuments.
    /// (You can also create indexes on nested fields using this syntax
    /// with [index_create](Self::index_create).)
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("post")
    ///   .order_by(r.row().g("group").g("id"))
    ///   .run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// You can efficiently order data on arbitrary expressions using indexes.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("posts").order_by(r.index("votes")).run(conn)
    /// # })
    /// ```
    ///
    /// The index must have been previously created with [index_create](Self::index_create).
    ///
    /// ```
    /// # use unreql::func;
    /// # unreql::example(|r, conn| {
    /// r.table("posts")
    ///   .index_create(r.args(("votes", func!(|post| {
    ///     post.clone().g("upvotes").sub(post.g("downvotes"))
    ///   }))))
    ///   .run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// If you have a sequence with fewer documents than the `arrayLimit`,
    /// you can order it with an arbitrary function directly.
    ///
    /// ```
    /// # use unreql::func;
    /// # unreql::example(|r, conn| {
    /// r.table("small_table")
    ///   .order_by(func!(|doc| {
    ///     doc.clone().g("upvotes").sub(doc.g("downvotes"))
    ///   }))
    ///   .run(conn)
    /// # })
    /// ```
    ///
    /// You can also select a descending ordering:
    ///
    /// ```
    /// # use unreql::func;
    /// # unreql::example(|r, conn| {
    /// r.table("small_table")
    ///   .order_by(r.desc(func!(|doc| {
    ///     doc.clone().g("upvotes").sub(doc.g("downvotes"))
    ///   })))
    ///   .run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Ordering after a `between` command can be done as long as
    /// the same index is being used.
    ///
    /// ```
    /// # use unreql::cmd::options::BetweenOptions;
    /// # unreql::example(|r, conn| {
    /// let opts = BetweenOptions {
    ///   index: Some("date".into()),
    ///   ..Default::default()
    /// };
    /// r.table("posts")
    ///   .between(r.time(2013, 1, 1, "+00:00"), r.time(2013, 1, 1, "+00:00"), opts)
    ///   .order_by(r.index("date"))
    ///   .run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [skip](Self::skip)
    /// - [limit](Self::limit)
    /// - [slice](Self::slice)
    only_command,
    order_by(key_or_function: ManyArgs<Index>)
);

create_cmd!(
    /// Skip a number of elements from the head of the sequence.
    ///
    /// ## Example
    /// Here in conjunction with [order_by](Self::order_by) we choose to ignore
    /// the most successful heroes.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("marvel").order_by("successMetric").skip(10).run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [limit](Self::limit)
    /// - [slice](Self::slice)
    /// - [nth](Self::nth)
    /// - [order_by](Self::order_by)
    only_command,
    skip(number: Serialize)
);

create_cmd!(
    /// End the sequence after the given number of elements.
    ///
    /// ## Example
    /// Only so many can fit in our Pantheon of heroes.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("marvel").order_by("belovedness").limit(10).run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [skip](Self::skip)
    /// - [slice](Self::slice)
    /// - [nth](Self::nth)
    /// - [order_by](Self::order_by)
    only_command,
    limit(number: Serialize)
);

create_cmd!(
    /// Return the elements of a sequence within the specified range.
    ///
    /// `slice` returns the range between `startOffset` and `endOffset`. If only
    /// `startOffset` is specified, `slice` returns the range from that index
    /// to the end of the sequence. Specify `leftBound` or `rightBound` as `open`
    /// or `closed` to indicate whether to include that endpoint of the range by
    /// default: `closed` returns that endpoint, while `open` does not. By default,
    /// `leftBound` is closed and `rightBound` is open, so the range `(10,13)`
    /// will return the tenth, eleventh and twelfth elements in the sequence.
    ///
    /// If `endOffset` is past the end of the sequence, all elements from
    /// `startOffset` to the end of the sequence will be returned. If `startOffset`
    /// is past the end of the sequence or `endOffset` is less than `startOffset`,
    /// a zero-element sequence will be returned.
    ///
    /// Negative `startOffset` and `endOffset` values are allowed with arrays;
    /// in that case, the returned range counts back from the array’s end.
    /// That is, the range `(-2)` returns the last two elements, and the range
    /// of `(2,-1)` returns the second element through the next-to-last element
    /// of the range. An error will be raised on a negative `startOffset` or
    /// `endOffset` with non-arrays. (An `endOffset` of −1 is allowed with
    /// a stream if `rightBound` is closed; this behaves as if no `endOffset`
    /// was specified.)
    ///
    /// If `slice` is used with a `binary` object, the indexes refer to byte
    /// positions within the object. That is, the range `(10,20)` will refer
    /// to the 10th byte through the 19th byte.
    ///
    /// With a string, `slice` behaves similarly, with the indexes referring
    /// to Unicode codepoints. String indexes start at `0`. (Note that *combining
    /// codepoints* are counted separately.)
    ///
    /// ## Example
    /// Return the fourth, fifth and sixth youngest players. (The youngest player
    /// is at index 0, so those are elements 3–5.)
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("players").order_by(r.index("age")).slice(r.args((3, 6))).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Return all but the top three players who have a red flag.
    ///
    /// ```
    /// # use unreql::rjson;
    /// # unreql::example(|r, conn| {
    /// r.table("players")
    ///   .filter(rjson!({"flag": "red"}))
    ///   .order_by(r.desc("score"))
    ///   .slice(3)
    ///   .run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Return holders of tickets `X` through `Y`, assuming tickets are numbered
    /// sequentially. We want to include ticket `Y`.
    ///
    /// ```
    /// # use unreql::rjson;
    /// # use unreql::cmd::options::{SliceOptions, Status};
    /// # unreql::example(|r, conn| {
    /// # let x = 1;
    /// # let y = 2;
    /// let opts = SliceOptions {
    ///   right_bound: Some(Status::Closed),
    ///   ..Default::default()
    /// };
    /// let opts = SliceOptions::new().right_bound(Status::Closed);
    /// r.table("users")
    ///   .order_by("ticket")
    ///   .slice(r.with_opt(r.args((x, y)), opts))
    ///   .run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Return the elements of an array from the second through two from the end
    /// (that is, not including the last two).
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.expr([0,1,2,3,4,5]).slice(r.args((2,-2))).run(conn)
    /// // Result: [2, 3]
    /// # })
    /// ```
    ///
    /// ## Example
    /// Return the third through fifth characters of a string.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.expr("rutabaga").slice(r.args((2,5))).run(conn)
    /// // Result: "tab"
    /// # })
    /// ```
    only_command,
    slice(start_end_offset: OneAndSecondOptionalArg<SliceOptions>)
);

create_cmd!(
    /// Get the `nth` element of a sequence, counting from zero. If the argument
    /// is negative, count from the last element.
    ///
    /// Example
    /// Select the second element in the array.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.expr([1,2,3]).nth(1).run(conn)
    /// # });
    /// // or
    /// # unreql::example(|r, conn| {
    /// r.expr([1,2,3]).bracket(1).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Select the bronze medalist from the competitors.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("players").order_by(r.index("score")).nth(2).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Select the last place competitor.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("players").order_by(r.index("score")).nth(-1).run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [skip](Self::skip)
    /// - [limit](Self::limit)
    /// - [slice](Self::slice)
    /// - [order_by](Self::order_by)
    only_command,
    nth(index: Serialize)
);

create_cmd!(
    /// Get the indexes of an element in a sequence. If the argument
    /// is a predicate, get the indexes of all elements matching it.
    ///
    /// ## Example
    /// Find the position of the letter ‘c’.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.expr(["a","b","c"]).offsets_of("c").run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Find the popularity ranking of invisible heroes.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("marvel")
    ///   .union(r.table("dc"))
    ///   .order_by("popularity")
    ///   .offsets_of(r.row().g("superpowers").contains("invisibility"))
    ///   .run(conn)
    /// # })
    /// ```
    only_command,
    offsets_of(datum_or_predicate: Serialize)
);

create_cmd!(
    /// Test if a sequence is empty.
    ///
    /// ## Example
    /// Are there any documents in the marvel table?
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("marvel").is_empty().run(conn)
    /// # })
    /// ```
    ///
    /// # Related commands
    /// - [offsets_of](Self::offsets_of)
    only_command,
    is_empty,
);

create_cmd!(
    /// Merge two or more sequences.
    ///
    /// The optional interleave argument controls how the sequences will be merged:
    ///
    /// - `true`: results will be mixed together; this is the fastest setting,
    ///   but ordering of elements is not guaranteed. (This is the default.)
    /// - `false`: input sequences will be appended to one another, left to right.
    /// - `"field_name"`: a string will be taken as the name of a field to
    ///   perform a merge-sort on. The input sequences must be ordered before
    ///   being passed to `union`.
    /// - function: the `interleave` argument can take a function whose argument
    ///   is the current row, and whose return value is a value to perform
    ///   a merge-sort on.
    ///
    /// # Example
    /// Construct a stream of all heroes.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("marvel").union(r.table("dc")).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Combine four arrays into one.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.expr([1, 2]).union(r.args(([3, 4], [5, 6], [7, 8, 9]))).run(conn)
    /// // Result: [1, 2, 3, 4, 5, 6, 7, 8, 9]
    /// # })
    /// ```
    ///
    /// ## Example
    /// Create a changefeed from the first example.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("marvel").union(r.table("dc")).changes(()).run(conn)
    /// # })
    /// ```
    ///
    /// Now, when any heroes are added, modified or deleted from either table,
    /// a change notification will be sent out.
    ///
    /// ## Example
    /// Merge-sort the tables of heroes, ordered by name.
    ///
    /// ```
    /// # use unreql::cmd::options::UnionOptions;
    /// # unreql::example(|r, conn| {
    /// r.table("marvel")
    ///   .order_by("name")
    ///   .union(r.with_opt(
    ///     r.table("dc").order_by("name"),
    ///     UnionOptions::new().interleave("name".into())
    ///   ))
    ///   .run(conn)
    /// # })
    /// ```
    union(sequence: ManyArgs<UnionOptions>)
);

create_cmd!(
    /// Select a given number of elements from a sequence with uniform random
    /// distribution. Selection is done without replacement.
    ///
    /// If the sequence has less than the requested number of elements
    /// (i.e., calling `sample(10)` on a sequence with only five elements),
    /// `sample` will return the entire sequence in a random order.
    ///
    /// ## Example
    /// Select 3 random heroes.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("marvel").sample(3).run(conn)
    /// # })
    /// ```
    ///
    /// ## Example
    /// Select and stratify 3 random heroes by belovedness.
    ///
    /// ```
    /// # unreql::example(|r, conn| {
    /// r.table("marvel").group("belovedness").sample(3).run(conn)
    /// # })
    /// ```
    only_command,
    sample(number: Serialize)
);
